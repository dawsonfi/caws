import { Duration } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { Function, Runtime, Alias, Version, CfnFunction } from 'aws-cdk-lib/aws-lambda'
import { SnsEventSource } from 'aws-cdk-lib/aws-lambda-event-sources'
import { ITopic, SubscriptionFilter, Topic } from 'aws-cdk-lib/aws-sns'
import { LambdaSubscription, SqsSubscription } from 'aws-cdk-lib/aws-sns-subscriptions'
import { ILambdaDeploymentConfig, LambdaDeploymentGroup } from 'aws-cdk-lib/aws-codedeploy'
import { IFilterPattern } from 'aws-cdk-lib/aws-logs'
import { IVpc, ISecurityGroup } from 'aws-cdk-lib/aws-ec2'
import { IAlarm } from 'aws-cdk-lib/aws-cloudwatch'
import { IQueue } from 'aws-cdk-lib/aws-sqs'
import { Role, ServicePrincipal, ManagedPolicy } from 'aws-cdk-lib/aws-iam'

export interface CawsLambdaProps {
  readonly functionName: string
  readonly brazilPackageName: string
  readonly deploymentConfig?: ILambdaDeploymentConfig
  readonly runtime: Runtime
  readonly handler: string
  readonly timeout: Duration
  readonly memorySize: number
  readonly environment?: { [key: string]: string }  
}

export class BrazilPackageLambda extends Function {
  readonly id: string
  readonly dlqQueue?: IQueue
  readonly dlqTopic?: ITopic

  private readonly deploymentGroup?: LambdaDeploymentGroup

  constructor(scope: Construct, id: string, props: BrazilPackageLambdaProps) {
    const lambdaCode = LambdaAsset.fromBrazil({
      brazilPackage: BrazilPackage.fromString(props.brazilPackageName),
      componentName: props.brazilPackageName,
      autoBuild: true,
    })

    const date = new Date().toISOString()
    const description = `Generated on: ${date}`

    super(scope, id, {
      ...props,
      code: lambdaCode,
      description: description,
      role: props.shouldReuse ? BrazilPackageLambda.createRole(scope, props.functionName) : undefined,
    })

    this.id = id
    const functionConstruct = this.node.defaultChild as CfnFunction

    if (props.enableSnapStart) {
      functionConstruct.addPropertyOverride('SnapStart', {
        ApplyOn: 'PublishedVersions',
      })
    }

    if (props.shouldHaveDeadLetterQueue) {
      const dlq = BrazilPackageLambda.createDLQ(scope, props.functionName)
      this.dlqQueue = dlq.queue

      this.dlqTopic = BrazilPackageLambda.createTopic(scope, props.functionName)
      this.dlqTopic.addSubscription(new SqsSubscription(dlq.queue))
      this.dlqTopic.grantPublish(this)

      functionConstruct.deadLetterConfig = {
        targetArn: this.dlqTopic.topicArn,
      }
    }

    if (props.deploymentConfig) {
      this.deploymentGroup = this.createDeploymentGroup(props.functionName, props.deploymentConfig, this.currentVersion)
    }
  }

  addSNSEventSource(topic: ITopic): void {
    this.addEventSource(new SnsEventSource(topic))
  }

  addSNSEventSourceWithFilter(topic: ITopic, filterPolicy?: { [attribute: string]: SubscriptionFilter }): void {
    topic.addSubscription(
      new LambdaSubscription(this, {
        filterPolicy: filterPolicy,
      }),
    )
  }

  addAutoRollbackAlarm(alarm: IAlarm): void {
    if (!this.deploymentGroup) {
      throw new Error(
        'adding an auto rollback alarm requires a deployment group. Please define a valid deploymentConfig when declaring this BrazilPackageLambda instance',
      )
    }
    this.deploymentGroup.addAlarm(alarm)
  }

  private createDeploymentGroup(functionName: string, config: ILambdaDeploymentConfig, version: Version): LambdaDeploymentGroup {
    const alias = new Alias(this, `${functionName}LambdaAlias`, {
      aliasName: 'live',
      version: version,
    })
    const deploymentGroupName = `${functionName}DeploymentGroup`
    return new LambdaDeploymentGroup(this, deploymentGroupName, {
      alias: alias,
      deploymentGroupName: deploymentGroupName,
      deploymentConfig: config,
      ignorePollAlarmsFailure: true,
      autoRollback: {
        stoppedDeployment: true,
        failedDeployment: true,
      },
    })
  }

  private static createRole(scope: Construct, functionName: string): Role {
    return new Role(scope, functionName.concat('-LambdaRole'), {
      assumedBy: new ServicePrincipal('lambda.amazonaws.com'),
      roleName: functionName.concat('-LambdaRole'),
      managedPolicies: [
        ManagedPolicy.fromAwsManagedPolicyName('service-role/AWSLambdaVPCAccessExecutionRole'),
        ManagedPolicy.fromAwsManagedPolicyName('service-role/AWSLambdaBasicExecutionRole'),
      ],
    })
  }
}