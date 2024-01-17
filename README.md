# Postgres Lock Explorer

Sometimes, it's difficult to know what type of lock a specific statement will
take on your Postgres database.

This provides a simple API which takes a statement you'd like to be run and
returns information about the lock that was taken.

## Installation

First, you'll need a Rust toolchain to compile the binary. You can run the
following:

```bash
# Install the Rust installation manager
brew install rustup-init

# Hit Enter for the default installation configuration
rustup-init

# Install the stable version of Rust
rustup update stable
```

This should give you the `cargo` binary:

```bash
cargo --version
```

You can then install the `pglx` binary:

```bash
cargo install --git https://github.com/alexander-jackson/postgres-lock-explorer.git
```

## Usage

### Running the server

You'll need to run the server separately against a Postgres database for
testing:

```bash
> pglx serve --help
Usage: pglx serve [OPTIONS] --user <USER> --database <DATABASE>

Options:
      --host <HOST>                Hostname of the database server
  -U, --user <USER>                Username for connecting to the database server
      --password <PASSWORD>        Password for connecting to the database server
  -d, --database <DATABASE>        Name of the database to connect to
  -p, --port <DATABASE_PORT>       Port of the database server
      --server-port <SERVER_PORT>  Port to run the server itself on
  -h, --help                       Print help

> pglx serve -U postgres -d testing --password test
```

### Making queries

You can then make requests to the server with the CLI:

```bash
> pglx query -i "ALTER TABLE other ADD CONSTRAINT fk_other_example_id FOREIGN KEY (example_id) REFERENCES example (id)"
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'public.example'
Lock of type 'relation' with mode 'RowShareLock' will be taken on relation 'public.example'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'public.example'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'public.example_pkey'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'public.other'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'public.other'
```

Or if you want to see the locks on a specific relation:

```bash
> pglx query -i "SELECT * FROM example" -r "example"
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'public.example'
```

Or if you want to see the locks on relations in a specific schema:

```bash
> pglx query -i "SELECT * FROM example" -s "public"
```

You can also read the query from a file:

```bash
> pglx query -i @query.sql
```

### Lock explanations

`pglx` also comes with some documentation about the different lock types and
their implications:

```bash
> pglx explain RowExclusiveLock
RowExclusiveLock

Conflicts with:
- ShareLock
- ShareRowExclusiveLock
- ExclusiveLock
- AccessExclusiveLock

Example queries acquiring this lock type:
- UPDATE payment SET status = 'COMPLETED'
- INSERT INTO payment (amount, currency) VALUES (25, 'GBP')
- DELETE FROM payment WHERE payment_id = 1

Example queries blocked by this lock type:
- CREATE INDEX idx_account_account_uid ON account (account_uid)
- CREATE TRIGGER check_update BEFORE UPDATE ON account FOR EACH ROW EXECUTE FUNCTION check_account_update()
- REFRESH MATERIALIZED VIEW CONCURRENTLY password_statistics
- ALTER TABLE account ADD COLUMN name TEXT
- DROP TABLE account
```
