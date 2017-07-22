DATABASE_URL=`sed -n -e 's/^.*url: //p' config.yml`

default: check

build:
	@DATABASE_URL=${DATABASE_URL} cargo build

run:
	@DATABASE_URL=${DATABASE_URL} cargo run

check:
	@DATABASE_URL=${DATABASE_URL} cargo check

clean:
	@DATABASE_URL=${DATABASE_URL} cargo clean

db-migrate:
	@DATABASE_URL=${DATABASE_URL} diesel migration run

db-redo:
	@DATABASE_URL=${DATABASE_URL} diesel migration redo
