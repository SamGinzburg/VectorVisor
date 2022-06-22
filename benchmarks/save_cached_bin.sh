#!/bin/bash

# Run each benchmark, compiling it
# Benchmarks on the RTX2080 are compiled for sm_75 which should run on Tesla+ GPUs

# Args:
# 1 = bench name
# 2 = heap size
# 3 = stack size
# 4 = hcall buf size
function cachebin() {
  cd ${1}/
  cargo build --release
  cd ..
  cp ${1}/target/wasm32-wasi/release/${1}.wasm .
  wasm-snip ${1}.wasm --snip-rust-panicking-code -o ${1}-snip.wasm -p rust_oom __rg_oom
  wasm-opt ${1}-snip.wasm -O1 -g -c -o ${1}-opt.wasm
}

cachebin "imagehash" "4194304" "131072" "262144"
cachebin "imagehash-modified" "4194304" "131072" "262144"
cachebin "imageblur" "4194304" "262144" "409600"
cachebin "imageblur-bmp" "4194304" "262144" "409600"
cachebin "json-compression" "4194304" "131072" "524288"
cachebin "pbkdf2" "3145728" "262144" "131072"
cachebin "scrypt" "3145728" "262144" "131072"
cachebin "average" "3145728" "131072" "262144"
cachebin "nlp-count-vectorizer" "4194304" "131072" "524288"

# Save the generated *.bin files
tar -zcvf nvbin.backup $( find -name "*.bin" )

# save the nvcache
tar -zcvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore nvcache
# tar -zxvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore *.bin
# tar -zxvf nvbin.backup
