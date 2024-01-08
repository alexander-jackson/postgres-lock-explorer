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

You can then make requests to the server with the CLI:

```bash
> pglx query -i "ALTER TABLE other ADD CONSTRAINT fk_other_example_id FOREIGN KEY (example_id) REFERENCES example (id)"
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'RowShareLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example_pkey'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'other'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'other'
```

Or if you want to see the locks on a specific relation:

```bash
> pglx query -i "SELECT * FROM example" -r "example"
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example'
```

You can also read the query from a file:

```bash
> pglx query -i @query.sql
```
