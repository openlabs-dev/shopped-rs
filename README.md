# shopped API

This is the source code for https://store.openlabs.online/.

## Getting started

If you are just interested in making changes to the backend, you can run the
backend in a development.

### Prerequisites

- Install Rust (https://rustup.rs/)

- Set up `.env` file:
  1. Copy `.env.example` to `.env`
  2. Set `DATABASE_URL` to point to your local Postgres database.

- Install `sqlx` by running `cargo install sqlx-cli`

**macOS**

- Postgres installed and running: `brew install postgresql`
- Postgres database created with `createdb shopped`
- Postgres user created and granted access to the database
- Run `cargo sqlx migrate run`
  - If you get the error `role "postgres" does not exist`, run
    `createuser -s postgres`.

### Migrating the database

When the database schema has been changed, you can migrate the local database by
running this command:

```sh
sqlx migrate run
```

### Running

- `cargo run` in your terminal

## LICENSE

[MIT LICENSE](./LICENSE)
