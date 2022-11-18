#!/bin/bash

# Run each benchmark, for local testing

# Args:
# 1 = bench name
# 2 = heap size
# 3 = stack size
# 4 = hcall buf size
# 5 = vmcount (T4)
# 6 = vmcount (A10G)
function runbin() {
  cargo run --release -- -i $1-opt-4.wasm.bin --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --interleave=8 --uw=true --serverless=true --rt=100 --lgroup=64
}

function runwasm() {
  cargo run --release -- -i $1-opt-4.wasm --wasmtime=true --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --interleave=4 --uw=true --serverless=true --rt=200 --profile=true
}

function wasm() {
  cargo run --release -- -i $1-opt-4.wasm --wasmtime=true --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --interleave=4 --uw=true --serverless=true --rt=200 
}

function comp() {
  cd ${1}/
  cargo build --release
  #RUSTFLAGS='-C llvm-args=-unroll-threshold=1000' cargo build --release
  cd ..
  cp ${1}/target/wasm32-wasi/release/${1}.wasm .
  wasm-snip ${1}.wasm --snip-rust-panicking-code -o ${1}-snip.wasm -p rust_oom __rg_oom slice_error_fail slice_index_order_fail slice_end_index_len_fail slice_start_index_len_fail
  wasm-opt ${1}-snip.wasm -O3 -g -c -o ${1}-opt.wasm
  cp ${1}-opt.wasm ${1}-opt-4.wasm
  cp ${1}-opt.wasm ${1}-opt-8.wasm
  cp ${1}-opt.wasm a10g_${1}-opt.wasm
  cp ${1}-opt.wasm a10g_${1}-opt-4.wasm
  cp ${1}-opt.wasm a10g_${1}-opt-8.wasm
  cargo run --release -- -i $1-opt-4.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --interleave=4 --serverless=true --rt=200 --uw=true
}

function comp_only() {
  cargo run --release -- -i $1-opt-4.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --interleave=8 --serverless=true --rt=200 --uw=true --lgroup=32
}

#comp "nlp-count-vectorizer" "4194304" "131072" "524288" "2048" "4608"
#runbin "captcha" "4194304" "131072" "409600" "2048" "2048"
#comp "captcha" "4194304" "131072" "409600" "2048" "2048"
#runbin "json-compression-lz4" "4194304" "131072" "524288" "2048" "2048"
#runwasm "json-compression-lz4" "4194304" "131072" "524288" "2048" "2048"
#comp "json-compression-lz4" "4194304" "131072" "524288" "2048" "2048"
#runbin "average" "3145728" "131072" "262144" "3072" "2048"
#comp "average" "3145728" "131072" "262144" "2048" "2048"
#comp "hello_go" "4194304" "131072" "409600" "2048" "2048"
#runbin "rust-pdfwriter" "4194304" "131072" "524288" "2048" "2048"
comp "rust-pdfwriter" "4194304" "131072" "524288" "2048" "2048"
#comp_only "rust-pdfwriter" "4194304" "131072" "409600" "2048" "2048"
#runwasm "rust-pdfwriter" "4194304" "131072" "409600" "2048" "2048"
#wasm "rust-pdfwriter" "4194304" "131072" "409600" "2048" "2048"
#runwasm "test" "125194304" "131072" "409600" "2048" "2048"
#wasm "test" "419430400" "131072" "409600" "2048" "2048"
#runbin "test" "4194304" "131072" "524288" "2048" "2048"
#comp_only "test" "4194304" "262144" "524288" "2048" "2048"
exit
runbin "pbkdf2" "3145728" "262144" "131072" "4096" "6144"
runbin "imagehash" "4194304" "131072" "262144" "3072" "4608"
runbin "imagehash-modified" "4194304" "131072" "262144" "3072" "4608"
runbin "imageblur" "4194304" "262144" "409600" "3072" "4608"
runbin "imageblur-bmp" "4194304" "262144" "409600" "3072" "4608"
runbin "json-compression" "4194304" "131072" "524288" "3072" "4608"
runbin "scrypt" "3145728" "262144" "131072" "4096" "6144"
runbin "average" "3145728" "131072" "262144" "4096" "5120"
runbin "nlp-count-vectorizer" "4194304" "131072" "524288" "3072" "4608"
#runbin "genpdf" "3145728" "131072" "262144" "4096" "5120"

# Save the generated *.bin files
#tar -zcvf nvbin.backup $( find -name "*.bin" )

# save the nvcache
#tar -zcvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore nvcache
# tar -zxvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore *.bin
# tar -zxvf nvbin.backup
