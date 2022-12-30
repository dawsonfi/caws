import { App } from 'aws-cdk-lib';
import { CawsStack } from './lib/caws_stack';

const app = new App();

const stackProps = {
    env: { 
        region: 'us-west-2',
        account: process.env.AWS_ACCOUNT_ID
    }
}

new CawsStack(app, 'caws-stack', stackProps);

app.synth();