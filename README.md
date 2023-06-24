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
Do you want to specify a relation? yes
✔ Enter a relation · example
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example'
```

Or if you want to see the locks on all relations:

```bash
> cargo run --bin cli
✔ Enter a query · ALTER TABLE other ADD CONSTRAINT fk_other_example FOREIGN KEY (example_id) REFERENCES example (id)
Do you want to specify a relation? no
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example_pkey'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'RowShareLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'example'
Lock of type 'relation' with mode 'AccessShareLock' will be taken on relation 'other'
Lock of type 'relation' with mode 'ShareRowExclusiveLock' will be taken on relation 'other'
```
