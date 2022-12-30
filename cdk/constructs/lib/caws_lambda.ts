import { Duration } from 'aws-cdk-lib'
import { Construct } from 'constructs'
import { Function, Runtime, Alias, Version, AssetCode } from 'aws-cdk-lib/aws-lambda'
import { ILambdaDeploymentConfig, LambdaDeploymentGroup } from 'aws-cdk-lib/aws-codedeploy'
import { LambdaDeploymentConfig } from 'aws-cdk-lib/aws-codedeploy'

export interface CawsLambdaProps {
  readonly functionName: string
  readonly brazilPackagePath?: string
  readonly deploymentConfig?: ILambdaDeploymentConfig  
  readonly timeout?: Duration
  readonly memorySize?: number
  readonly environment?: { [key: string]: string }  
}

export class CawsLambda extends Function {  
  constructor(scope: Construct, id: string, props: CawsLambdaProps) {
    super(scope, id, {
      ...props,
      code: new AssetCode(props.brazilPackagePath?? 'target/lambda/release/bootstrap.zip'),
      description: `Generated on: ${new Date().toISOString()}`,
      runtime: Runtime.PROVIDED_AL2,
      handler: 'doesnt.matter'
    })

    this.createDeploymentGroup(
      props.functionName, 
      props.deploymentConfig ?? LambdaDeploymentConfig.ALL_AT_ONCE,
      this.currentVersion
    )
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
}