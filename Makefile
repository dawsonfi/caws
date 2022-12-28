build-zip:
	cargo lambda build --release --arm64 --output-format zip

release: build-zip

cdk-install:
	npm --prefix cdk install cdk

cdk-build: cdk-install
	npm --prefix cdk run build

deploy: release cdk-build
	cdk deploy *-stack --profile kevin --app "node cdk/index" --require-approval never