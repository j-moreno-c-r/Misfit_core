# Misfit-Core

A tool to create specific valid or invalid scenarios for testing Bitcoin blocks and transactions.


## Dependencies
 * Rust 
 * Cargo
 * Bitcoin

 
## Installation
Clone the repo
```b
git clone https://github.com/j-moreno-c-r/Misfit_core
```
Enter the directory
```b
cd Misfit_core
```
Compile the project with Cargo
```b
cargo build 
```
Run the binary
```b
./target/debug/Misfit_core 
```
Or run with Cargo
```b
cargo run
```
For now, the flags are not working, they are just here for development annotation.

## Basic usage

Type `help` on the CLI to see the available commands.
```b
> help
```

## Advaced usage 
In```/src/reference_implementation/``` you will find: ```defaults.json```, ```api.rs``` , ```cli.rs```,  ```read_defaults.rs```, you can change the json file to personalize the defaults to random generation of blocks or transactions, or you can read this folder to learn how to use by importing our lib, you can use Misfit_core directly on your project generating direclty your use cases.