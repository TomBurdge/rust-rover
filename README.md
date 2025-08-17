# Rust Rover
This repo solves `The Problem` as outlined below, with an interface for solving the `The Problem` with both rust and go.

## The Problem
A squad of robotic rovers are to be landed by NASA on a plateau on Mars.

This plateau, which is curiously rectangular, must be navigated by the rovers so that their on board cameras can get a complete view of the surrounding terrain to send back to Earth.

A rover's position is represented by a combination of an x and y co-ordinates and a letter representing one of the four cardinal compass points. The plateau is divided up into a grid to simplify navigation. An example position might be 0, 0, N, which means the rover is in the bottom left corner and facing North.

In order to control a rover, NASA sends a simple string of letters. The possible letters are 'L', 'R' and 'M'. 'L' and 'R' makes the rover spin 90 degrees left or right respectively, without moving from its current spot.

'M' means move forward one grid point, and maintain the same heading.

Assume that the square directly North from (x, y) is (x, y+1).


### Input:
The first line of input is the upper-right coordinates of the plateau, the lower-left coordinates are assumed to be 0,0.
The rest of the input is information pertaining to the rovers that have been deployed. Each rover has two lines of input. The first line gives the rover's position, and the second line is a series of instructions telling the rover how to explore the plateau.
The position is made up of two integers and a letter separated by spaces, corresponding to the x and y co-ordinates and the rover's orientation.
Each rover will be finished sequentially, which means that the second rover won't start to move until the first one has finished moving.

### Output:
The output for each rover should be its final co-ordinates and heading.
Test Input:
```
```5 5
1 2 N
LMLMLMLMM
3 3 E
MMRMMRMRRM
```

Expected Output:
```
1 3 N
5 1 E
```

## Rust Workspace Structure
This repository is a [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) with two crates.

Aside from small benefits of compile times, it's useful to completely isolate the core rust functionality from the C interface. I often find it's easy to end up with lots of files which you all mark as public, so that you can use them in different files, with a single crate. With a workspace, the *only* public component of the `rover` crate is the function we want to expose: it solves the problem from the section above. We could serve this crate, if the use case did not involve use of go.

There are two crates:
* **Rover**: The core rust functionality for solving the Rover exercises outlined above.
* **C_Rover**: The C FFI interface for the `Rover` crate. This crate is used by the Go code in this repo

### Mask Instructions
For reading through the repo, I recommend you [install mask](https://github.com/jacobdeichert/mask), which is a CLI task runner in markdown form.

To install mask:
```sh
cargo install mask
```

When in this repo's root, you can see the commands available with `mask --help`.
Mask is great because it's in markdown form, so you can see details on a command in the `maskfile.md` itself.

### Crate 1: Rover
The public function processes two String inputs to find the rover(s)'s final position.

The module is tested for expected failures where inputs are malformed.

Does allow for over-lapping final rover positions, but an error could be raised if this were a problem with a Hashmap.

There are docs comments with further reflections. Therefore, if you have installed Mask, then you can read through the docs with `mask rover-docs`.

### Crate 2: C_Rover
The C Rover crate is the interface for calling the `rover` crate in another language via [the C FFI](https://doc.rust-lang.org/nomicon/ffi.html).

There are docs comments with further reflections. Therefore, if you have installed Mask, then you can read through the docs with `mask c-docs`.


## Go Structure
There are Go files in the root of the repository.

I have not solved the rover  problem in go. Instead, go can call rust via the `c_rover` crate through each language's (rust and go) C FFI. If I had solved the problem in go, it would have likely been fairly similar to rust, except for without some of rust's specific features (Enums/implementing intos).

Why write an interface to be called by go?

To see the interface running in action, you will first need to install `cargo zigbuild`:
```sh
cargo install --locked cargo-zigbuild
```
cargo zigbuild allows you to cross-compile rust code for different architectures.

Then, you can run the go interface with:
`mask build-run-rust-go`

This will:
* Build the workspace for most common operating systems (including Apple).
* Runs the `main.go` file, which uses the base (passing) example from the problem above.

There is also a rust-go integration test in go.
It is an integration test, because it assumes the right C library for the host architecture has already been built.
You can run the integration test with `go test` if you have already run `mask build`, or `mask build` (which will always run the cargo build step). The test is only for a successful integration, so it does not repeat  all the cases in the `rover` crate.

On the go-side, Cgo is not needed, because [purego](https://github.com/ebitengine/purego) does magic to call C functions without it. This is great because CGo is often challenging for cross-compilation/behaves differently when cross-compiled.

### What is the point of writing this go/rust interface?
Although this is only an exercise, I think this is a good example where: 
* We have already solved a problem in one language.
* If wanted to also solve it another language, we would not need to either re-write/network interface (with an infra and data transfer overhead) via two micro-services.

In the real-world, this interface can be useful if:
* There is a particularly good open-source crate we want to use which is not available in go.
* An operation is particularly performance critical, and rust can provide a benefit.

# Reflections

## What went well
* Testing - I feel that it is fairly complete and gives fairly good error messages, particularly for parsing inputs.
* I feel that I have used Rust's type system reasonably well with `enums`/`intos`/`try_into`.
* Error messages (with `thiserror`) have come out quite nicely, with reasonably informative error messages.
* There  is a `mask.md` for easy testing/recreation by anyone reading this repo.
* `c_rover` FFI - I wanted to demonstrate some go competence without re-doing the exercise.

## What I would want to refactor
* &str rather than String: with the stack/heap differences between String and &str, the (over-)reliance on strings in this repo could have peformance implications. [The first comment in this thread](https://users.rust-lang.org/t/understanding-when-to-use-string-vs-str/103746/2) is quite helpful for unerstanding where a bit of re-factoring could lead to improvement - in some cases the String already exists, or we have processed it, so we could go with a `&str`. The C interface needs to use Strings rather than `&str`.

* C does not require CGo (thanks to purego), but does require cross-compiling. Using WASM for the rust-go interface would be probably the same for performance, but would not need the same cross-compiling. It would also need fewer/no unsafe blocks. At a later time, I will likely return to this repo and make a `wasm_rover` crate, which creates the `WASM` interface, and uses [gravity](https://github.com/arcjet/gravity) to call this functionality from go via `WASM` rather than C.
