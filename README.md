# Testing PR https://github.com/stellar/soroban-cli/pull/159

This repo contains a test for testing https://github.com/stellar/soroban-cli/pull/159.

## Setup

Check out this repo:

```
git clone ...
cd ..
```

Install soroban-cli that has the `serveaccount` change from https://github.com/stellar/soroban-cli/pull/159:

```
cargo install --locked --git https://github.com/stellar/soroban-cli --branch serveaccount soroban-cli
```

## Usage

Build contract:
```
make build-optimized
```

Deploy contract:
```
soroban-cli deploy --id 1 --wasm target/wasm32-unknown-unknown/release/contract.wasm
```

Check invoking works:
```
soroban-cli invoke --id 1 --fn put --arg mykey --arg 2
```

Check that the data is as expected:
```
soroban-cli read --id 1
```
```
"[[""Account"",""GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF""],""mykey""]",2
```

Run soroban-cli serve:
```
soroban-cli serve
```

Generate invoke tx:
```
cargo run --bin tx
```

Copy the tx base64 that is output to stdout into a post:
```
ht POST :8080/api/v1/jsonrpc jsonrpc=2.0 method=sendTransaction params:='["AAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAMgAAAAAAAAAAQAAAAAAAAAAAAAAAQAAAAAAAAAYAAAAAAAAAAMAAAAEAAAAAQAAAAQAAAAgAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAFAAAAA3B1dAAAAAAEAAAAAQAAAAAAAAACAAAABQAAAAVteWtleQAAAAAAAAEAAAB7AAAAAAAAAAEAAAAGAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAEAAAAEAAAAAQAAAAAAAAACAAAABAAAAAEAAAAHAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAABAAAABQAAAAVteWtleQAAAAAAAAAAAAAA"]'
```

Expect to see a response like:
```
HTTP/1.1 200 OK
Content-Length: 129
Content-Type: application/json; charset=utf-8
Date: Wed, 28 Sep 2022 19:32:00 GMT

{
    "jsonrpc": "2.0",
    "id": null,
    "result": {
        "id": "e91bbb12c6c0dac8ad04cae8344d1103ca94cc364782c0147bbc82a755604a99",
        "status": "pending"
    }
}
```

Check the status of the transaction:
```
ht POST :8080/api/v1/jsonrpc jsonrpc=2.0 method=getTransactionStatus params:='["e91bbb12c6c0dac8ad04cae8344d1103ca94cc364782c0147bbc82a755604a99"]'
```
```
...
```

Check that the data is updated:
```
soroban-cli read --id 1
```
```
"[[""Account"",""GAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAWHF""],""mykey""]",123
```

---

Note the commands use `ht` which is a httpie-like command line tool:
https://github.com/nojima/httpie-go
