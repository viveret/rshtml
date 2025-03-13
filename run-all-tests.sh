#!/bin/sh
cd core_lib
cargo test --
cd ..


cd core_macro_lib
cargo test --
cd ..


cd mvc_lib
cargo test --
cd ..

cd mvc_macro_lib
cargo test --
cd ..			     

cd example_web_app
cargo test --
cd ..
