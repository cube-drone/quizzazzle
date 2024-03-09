default:
	cat Makefile

# generate rocket secret key with: openssl rand -base64 32
# AWS_ACCESS_KEY_ID: <fill this in if you want to test email locally>
# AWS_SECRET_ACCESS_KEY: <you keep it in keepass, under ses-groovelet access key
# AWS_DEFAULT_REGION: us-west-2
start:
	GROOVELET_ROOT_EMAIL="test@test.test" \
	GROOVELET_ROOT_AUTH_PASSWORD="test" \
	ROCKET_ADDRESS="0.0.0.0" \
	ROCKET_PORT="3333" \
	ROCKET_PUBLIC_ADDRESS="http://localhost:3333" \
	USE_INSECURE_HASHES="true" \
	ROCKET_SECRET_KEY="grPMXJszWoqdlGv1M7Y+9f0FsPhxsycGVjuQ6mfobIk=" \
	RUST_BACKTRACE="full" \
	cargo run

test:
	cargo test -- --nocapture
unit:
	cargo test -- --nocapture
release-test:
	cargo test --release -- --nocapture