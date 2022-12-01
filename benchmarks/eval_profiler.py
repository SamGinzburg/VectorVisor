import time
import os
from datetime import date, datetime
import re
import argparse
import subprocess
import os
import re

gpu = "N/A" 

print ("gpu: ", gpu)

if gpu == "t4":
    run_a10g = False
else:
    run_a10g = True

# Benchmark constants
# target rps is really just the number of concurrent invokers
# this affects the *possible* max RPS and bandwidth/mem/cpu consumption of the invoker
vmcount=3072
target_rps = 3072
target_rps_cpu = 1024
TIMEOUT_MINUTES = 60 * 12
#local_group_size = 999999
is_pretty = "true"
fastreply = "true"
CFLAGS="-cl-nv-verbose"
OPT_LEVEL="-O1 -g"
WASM_SNIP_ARGS="--snip-rust-panicking-code"
WASM_SNIP_CUSTOM="rust_oom __rg_oom"
maxfuncs = 50
maxloc = 2000000
#maxfuncs = 999
#maxloc = 20000000
benchmark_duration = 600
SLEEP_TIME=120
NUM_REPEAT=1
interleave=4


def run_cmd(cmd, block=False):
    process = subprocess.Popen(["bash", "-c", cmd], stdout=subprocess.PIPE)
    if block:
        output, error = process.communicate()
        print (output)
        return output
    else:
        return ""

def run_profile_generic(bench_name, params=""):
    run_command_wasmtime = """#!/bin/bash
      
    vectorvisor --input {name}-opt-instrument.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=1310720 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --profile=true
    """.format(interleave=interleave, name=bench_name)

    run_cmd(run_command_wasmtime) 

    # now run the invoker(s) for pbkdf2
    run_invoker = """#!/bin/bash
    cd {name}/
    go run run_*.go {addr} 8000 {target_rps} 1 {duration} {params}
    """.format(addr="localhost", target_rps=256, duration=300, name=bench_name, params=params)

    time.sleep(20)

    # Block until benchmark is complete
    run_cmd(run_invoker, block=True) 


    run_invoker = """#!/bin/bash
    vv-profiler --input {name}-opt-4.wasm --output {name}-opt-profile.wasm --profile={name}-opt-instrument.wasm.profile
    wasm-opt -O1 -g {name}-opt-profile.wasm -o {name}-opt-profile.wasm
    cp {name}-opt-profile.wasm {name}-opt-4-profile.wasm
    cp {name}-opt-profile.wasm {name}-opt-8-profile.wasm

    # profile the optimized binary as well, should be set up for the next invocation
    vv-profiler --input {name}-opt-profile.wasm --output {name}-opt-instrument.wasm
    """.format(target_rps=vmcount, duration=60, hashes=256, name=bench_name)

    time.sleep(20)

    # Block until benchmark is complete
    run_cmd(run_invoker, block=True)

def get_profile(bench_name, slowcall_map, indirect_map):
    run_invoker = """#!/bin/bash
    cat {name}-opt-instrument.wasm.slowcalls
    """.format(target_rps=vmcount, duration=60, hashes=256, name=bench_name)

    # Block until benchmark is complete
    output = run_cmd(run_invoker, block=True)
    slowcalls = re.search(r"slowcalls:\ (\d+)", str(output)).groups(0)[0]
    indirect = re.search(r"indirect:\ (\d+)", str(output)).groups(0)[0]
    print (bench_name, slowcalls, indirect)
    slowcall_map[bench_name] = slowcalls
    indirect_map[bench_name] = indirect

block_until_done = """#!/bin/bash
./local_cached_bin.sh
""".format(gpu=gpu)

def dump_dict(d, d_name):
    with open('{name}.txt'.format(name=d_name), 'w') as f:
        for k, v in d.items():
            f.write(str(k) + "\t" + str(v) + "\n")

run_cmd(block_until_done, block=True)

# Now generate the profiling data
# For each benchmark we need to:
# 1) Generate an instrumented binary
# 2) Run VV-wasm with the instrumented binary w/some workload
# 3) Use the generated profile to emit an optimized WASM binary
run_profile_generic("rust-pdfwriter")
run_profile_generic("average", params="20")
run_profile_generic("imageblur")
run_profile_generic("imageblur-bmp")
run_profile_generic("imagehash")
run_profile_generic("imagehash-modified")
run_profile_generic("json-compression", params="smaller_tweets.txt 2000")
run_profile_generic("scrypt", params="256")
run_profile_generic("pbkdf2")
run_profile_generic("nlp-count-vectorizer", params="smaller_tweets.txt 500")

# Collect profiling results
slowcalls = dict()
indirect = dict()
get_profile("rust-pdfwriter", slowcalls, indirect)
get_profile("average", slowcalls, indirect)
get_profile("imageblur", slowcalls, indirect)
get_profile("imageblur-bmp", slowcalls, indirect)
get_profile("imagehash", slowcalls, indirect)
get_profile("imagehash-modified", slowcalls, indirect)
get_profile("json-compression", slowcalls, indirect)
get_profile("scrypt", slowcalls, indirect)
get_profile("pbkdf2", slowcalls, indirect)
get_profile("nlp-count-vectorizer", slowcalls, indirect)

dump_dict(slowcalls, "slowcalls")
dump_dict(indirect, "indirect")

# Now that we have generated optimized binaries, we will profile those too
run_profile_generic("rust-pdfwriter")
run_profile_generic("average", params="20")
run_profile_generic("imageblur")
run_profile_generic("imageblur-bmp")
run_profile_generic("imagehash")
run_profile_generic("imagehash-modified")
run_profile_generic("json-compression", params="smaller_tweets.txt 2000")
run_profile_generic("scrypt", params="256")
run_profile_generic("pbkdf2")
run_profile_generic("nlp-count-vectorizer", params="smaller_tweets.txt 500")

slowcalls = dict()
indirect = dict()
get_profile("rust-pdfwriter", slowcalls, indirect)
get_profile("average", slowcalls, indirect)
get_profile("imageblur", slowcalls, indirect)
get_profile("imageblur-bmp", slowcalls, indirect)
get_profile("imagehash", slowcalls, indirect)
get_profile("imagehash-modified", slowcalls, indirect)
get_profile("json-compression", slowcalls, indirect)
get_profile("scrypt", slowcalls, indirect)
get_profile("pbkdf2", slowcalls, indirect)
get_profile("nlp-count-vectorizer", slowcalls, indirect)

dump_dict(slowcalls, "slowcalls-opt")
dump_dict(indirect, "indirect-opt")
