lambda-build:
	cargo lambda build --release
lambda-deploy:
	cargo lambda deploy --iam-role arn:aws:iam::730335309914:role/cargo-lambda-role-84dc1533-0eae-4da2-8caf-539f1ce54847