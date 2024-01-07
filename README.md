# kanic (C Compiler written by Rust)

## What is this repository?

Implement a C compiler in Rust according to the following site.

<https://www.sigbus.info/compilerbook>

## Run Container

```bash
./bin/host/build_image
./bin/host/run_container
```

## Run C Program in Container

```bash
./bin/run ./tmp/main.c
```

## Run test

```bash
cargo test
```
