#!/bin/sh
cd rusthtml
cargo test --
cd ..


cd rusthtml-macro
cargo test --
cd ..


cd mvc_lib
cargo test --
cd ..
