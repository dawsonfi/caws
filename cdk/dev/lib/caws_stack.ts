import { App, Stack, StackProps } from 'aws-cdk-lib';
import { CawsLambda } from 'caws-constructs';

export class CawsStack extends Stack {    
    constructor(parent: App, name: string, props: StackProps) {
        super(parent, name, props);

        new CawsLambda(this, 'caws-lambda', {
          functionName: 'caws-lambda',
        })
   }
}

