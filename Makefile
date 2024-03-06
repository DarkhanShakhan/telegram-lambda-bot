lambda-build:
	cargo lambda build --release
lambda-deploy:
	cargo lambda deploy --iam-role arn:aws:iam::637423632440:role/service-role/telegram-bot-role-quvyfmor