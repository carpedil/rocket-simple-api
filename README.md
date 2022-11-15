# Running Migrator CLI

1. sea-orm-cli migrate init

2. write migrate files

3. create .env file 

4. set DATABASE_URL 

5. uncomment  the last two  line to enable async_runtime & database driver features
<!-- migration/Cargo.toml -->
<!-- [dependencies.sea-orm-migration]
version = "^0.10.0"
features = [
  # Enable at least one `ASYNC_RUNTIME` and `DATABASE_DRIVER` feature if you want to run migration via CLI.
  # View the list of supported features at https://www.sea-ql.org/SeaORM/docs/install-and-config/database-and-async-runtime.
  # e.g.
  "runtime-tokio-rustls",  # `ASYNC_RUNTIME` feature
  "sqlx-postgres",         # `DATABASE_DRIVER` feature
] -->

6. execute 'sea-orm-cli migrate up' in terminal

7. generate entities files
cargo new entity --lib  and add dependencies to the toml file
sea-orm = "0.9.1"


8. execute 'sea-orm-cli generate entity -o entity/src'


9. mv entity/src/mod.rs entity/src/lib.rs
