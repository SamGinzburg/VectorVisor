#!/bin/bash

# Run each benchmark, compiling it
# Benchmarks on the RTX2080 are compiled for sm_75 which should run on Tesla+ GPUs
# remember to clear ~/.nv/ComputeCache/ before running to minimize file sizes

# Args:
# 1 = bench name
# 2 = heap size
# 3 = stack size
# 4 = hcall buf size
# 5 = vmcount (T4)
# 6 = vmcount (A10G)
function cachebin() {
  cd ${1}/
  cargo build --release
  #RUSTFLAGS='-C llvm-args=-unroll-threshold=1000' cargo build --release
  cd ..
  cp ${1}/target/wasm32-wasi/release/${1}.wasm .
  wasm-snip ${1}.wasm --snip-rust-panicking-code -o ${1}-snip.wasm -p rust_oom __rg_oom slice_error_fail slice_index_order_fail slice_end_index_len_fail slice_start_index_len_fail
  wasm-opt ${1}-snip.wasm -O1 -g -c -o ${1}-opt.wasm
  cp ${1}-opt.wasm ${1}-opt-4.wasm
  cp ${1}-opt.wasm ${1}-opt-8.wasm
  # generate an instrumented binary as well
  vv-profiler --input ${1}-opt.wasm --output ${1}-opt-instrument.wasm
}

function nlp-script() {
  cp ${1}/release-opt.wasm ${1}-opt.wasm
  cp ${1}-opt.wasm ${1}-opt-4.wasm
  cp ${1}-opt.wasm ${1}-opt-8.wasm
  vv-profiler --input ${1}-opt.wasm --output ${1}-opt-instrument.wasm
}

cachebin "rust-pdfwriter" "4194304" "131072" "409600" "2048" "4608"
cachebin "pbkdf2" "3145728" "262144" "131072" "2048" "6144"
cachebin "imagehash" "4194304" "131072" "262144" "2048" "4608"
cachebin "imagehash-modified" "4194304" "131072" "262144" "2048" "4608"
cachebin "imageblur" "4194304" "262144" "409600" "3072" "2048"
cachebin "imageblur-bmp" "4194304" "262144" "409600" "2048" "4608"
cachebin "json-compression" "4194304" "131072" "524288" "2048" "4608"
cachebin "scrypt" "3145728" "262144" "131072" "2048" "6144"
cachebin "average" "3145728" "131072" "262144" "2048" "5120"
cachebin "nlp-count-vectorizer" "4194304" "131072" "524288" "2048" "4608"
nlp-script "nlp-assemblyscript" "3145728" "131072" "8192" "4096" "4608" "false"
nlp-script "nlp-go" "3145728" "131072" "8192" "4096" "4608" "true"
#cachebin "genpdf" "3145728" "131072" "262144" "4096" "5120"

# Save the generated *.bin files
#tar -zcvf nvbin.backup $( find -name "*.bin" )

# save the nvcache
#tar -zcvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore nvcache
# tar -zxvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore *.bin
# tar -zxvf nvbin.backup
