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
  cargo run --release -- -i $1-opt-4-profile.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --jt=true --interleave=4 --uw=true --nvidia=false --patch=true &> /vv/$1-opt-4-profile.log
  cargo run --release -- -i $1-opt-8-profile.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --jt=true --interleave=8 --uw=true --nvidia=false --patch=true &> /vv/$1-opt-8-profile.log
}

function nlpscript() {
  cargo run --release -- -i $1-opt-4-profile.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --jt=true --interleave=4 --uw=true --pinput=$7 --nvidia=false --patch=true &> /vv/$1-opt-4-profile.log
  cargo run --release -- -i $1-opt-8-profile.wasm --heap=$2 --stack=$3 --hcallsize=$4 --vmcount=$5 --partition=false --maxdup=0 --jt=true --interleave=8 --uw=true --pinput=$7 --nvidia=false --patch=true &> /vv/$1-opt-8-profile.log
}

#cachebin "rust-pdfwriter" "4194304" "131072" "409600" "1536" "4608"
#cachebin "pbkdf2" "3145728" "262144" "131072" "2048" "6144"
cachebin "imagehash" "4194304" "131072" "262144" "1536" "4608"
cachebin "imagehash-modified" "4194304" "131072" "262144" "1536" "4608"
cachebin "imageblur" "4194304" "262144" "409600" "1536" "4608"
cachebin "imageblur-bmp" "4194304" "262144" "409600" "1536" "4608"
cachebin "json-compression" "4194304" "131072" "524288" "1536" "4608"
cachebin "scrypt" "3145728" "262144" "131072" "2048" "6144"
cachebin "average" "3145728" "131072" "262144" "2048" "5120"
#cachebin "nlp-count-vectorizer" "3145728" "131072" "8192" "2048" "4608"
#nlpscript "nlp-assemblyscript" "3145728" "131072" "8192" "2048" "4608" "false"
#nlpscript "nlp-go" "3145728" "131072" "8192" "2048" "4608" "true"
#cachebin "genpdf" "3145728" "131072" "262144" "4096" "5120"

# Save the generated *.bin files
#tar -zcvf nvbin.backup $( find -name "*.bin" )

# save the nvcache
#tar -zcvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore nvcache
# tar -zxvf nvcache.backup -C ~/.nv/ComputeCache/ .

# restore *.bin
# tar -zxvf nvbin.backup
