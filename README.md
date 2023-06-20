# Postgres Lock Explorer

Sometimes, it's difficult to know what type of lock a specific statement will
take on your Postgres database.

This provides a simple API which takes a statement you'd like to be run and
returns information about the lock that was taken.

## Usage

You'll need to run the server separately against a Postgres database for
testing:

```bash
cargo run -- -h localhost -U postgres -d testing
```

You can then make requests to the server using `curl` (and parsing with `jq`):

```bash
curl \
    -s -X PUT -H "Content-Type: application/json" \
    --data '{"query": "SELECT * FROM example", "relation": "example"}' \
    http://localhost:5430/analyse | jq

# Output
{
  "locktype": "relation",
  "mode": "AccessShareLock"
}
```
