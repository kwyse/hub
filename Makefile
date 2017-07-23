DATABASE_URL=`sed -n -e 's/^.*database_url: //p' config.yml`

default: lint

build:
	@DATABASE_URL=${DATABASE_URL} cargo build

run:
	@DATABASE_URL=${DATABASE_URL} cargo run

test:
	@DATABASE_URL=${DATABASE_URL} cargo test

lint:
	@DATABASE_URL=${DATABASE_URL} cargo check --features 'clippy'

clean:
	@DATABASE_URL=${DATABASE_URL} cargo clean

db-migrate:
	@DATABASE_URL=${DATABASE_URL} diesel migration run

db-redo:
	@DATABASE_URL=${DATABASE_URL} diesel migration redo

db-revert:
	@DATABASE_URL=${DATABASE_URL} diesel migration revert

db-reset:
	@DATABASE_URL=${DATABASE_URL} diesel database reset
