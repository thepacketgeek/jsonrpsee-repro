# Attempt to repro a jsonrpsee issue
https://github.com/paritytech/jsonrpsee/issues/98

## Run Server

```
$ cargo run -p server
```
This will run unlimited, and previously the client would block on receiving > 32 items.

```
$ cargo run -p server -- --limit 30
```
This will limit the response and should allow the client to receive the response without issue

## Run Client

```
$ cargo run -p client
```