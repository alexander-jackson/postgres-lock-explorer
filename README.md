# Postgres Lock Explorer

Sometimes, it's difficult to know what type of lock a specific statement will
take on your Postgres database.

This provides a simple API which takes a statement you'd like to be run and
returns information about the lock that was taken.

## Usage

You'll need to run the server separately against a Postgres database for
testing:

```bash
> cargo run --bin postgres-lock-explorer -- -h localhost -U postgres -d testing
```

You can then make requests to the server with the CLI:

```bash
> cargo run --bin cli
✔ Enter a query · SELECT * FROM example
✔ Enter a relation · example
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example'
```
