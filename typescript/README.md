# Overview

Deno allows the call of Rust functions from JS/TS easily

## Installation without Docker

```shell
curl -fsSL https://deno.land/install.sh | sh
```

See <https://docs.deno.com/runtime/manual/> for details

## Warning

They're some limitation with TS support
<https://docs.deno.com/runtime/manual/advanced/typescript/overview>

We need to check they're ok for us

## Dumb test

```shell
./run.sh
```

## Installation with Docker

### build image

```sh
docker build --tag deno_runner .
```

Then

```sh
cd ../ (to place in root dir)
docker run -v ./:/app deno_runner
````
