# kanic (C Compiler written by Rust)

## What is this repository?

C Compiler written by Rust. Based on <https://www.sigbus.info/compilerbook>.

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
# In container
cargo test
```
