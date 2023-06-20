# Postgres Lock Explorer

Sometimes, it's difficult to know what type of lock a specific statement will
take on your Postgres database.

This provides a simple API which consumes an initial setup statement (such as
creating an example table) alongside a statement you'd like to be run, and
returns information about the lock.
