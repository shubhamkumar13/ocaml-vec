# ocaml-vec

An [ocaml-rs](https://github.com/zshipko/ocaml-rs) wrapper around a Rust `Vec<i32>` for use in OCaml. It mainly exists to serve as a template for creating OCaml libraries using Rust.

## Building

    dune build

to run the tests:

    dune runtest

## Installation

    opam pin add vec . --inplace-build


## API

See `src/vec.mli`

