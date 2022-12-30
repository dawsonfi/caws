release:
	cargo lambda build --release --x86-64 --output-format zip

cdk-install:
	npm --prefix cdk/dev install

cdk-build: cdk-install
	npm --prefix cdk/dev run build

bootstrap: release cdk-build
	cdk bootstrap --app "node cdk/dev/dist/index"

deploy: release cdk-build
	cdk deploy *-stack --app "node cdk/dev/dist/index" --require-approval never