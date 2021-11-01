# sam-app

This project is a minimal template containing 2 lambda functions configured using 
sam.

The first is a Python function, using the default python 3.9 runtime.

The second is written in Rust, using a custom Runtime.

# build and run

```
# Builds the sam project.
sam build

# invokes the python function
sam local invoke HelloWorldFunction 

# invokes the rust function
sam local invoke HelloRustFunction  

# deploys both functions to aws
sam deploy                          
```

# The Python Function

There is nothing special with the python function!


# The Rust Function

The rust function is fully housed in the `hello_world_rust` directory. 
The diretory contains a rust project. The project structure is not special. It is a regular cargo project.

## What makes this project work with SAM?

> Answer: The Makefile

Sam expects a makefile with a rule called `build-HelloRustFunction` to compile the `HelloRustFunction`.
It expects you to place a `bootstrap` binary inside the `$(ARTIFACTS_DIR)`.

```
build-HelloRustFunction:
	cargo build --release --target x86_64-unknown-linux-musl
	cp ./target/x86_64-unknown-linux-musl/release/bootstrap $(ARTIFACTS_DIR)
```
To add new rust functions:

1. Creating the new function in the `template.yaml` sam file.
2. Add a new binary to the cargo project.
3. Add a new entry to the Makefile.
4. That's it!

