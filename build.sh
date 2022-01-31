#!/bin/bash

set -xe

wit-bindgen js --export console.wit \
               --import image_module.wit
rm *.d.ts
mv *.js static/

cargo build --release --target=wasm32-unknown-unknown
