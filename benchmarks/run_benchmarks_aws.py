import boto3
import time
import os
from datetime import date, datetime

# Benchmark constants
# target rps is really just the number of concurrent invokers
# this affects the *possible* max RPS and bandwidth/mem/cpu consumption of the invoker
target_rps = 3072
target_rps_cpu = 1024
TIMEOUT_MINUTES = 120
interleave = 8
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
benchmark_duration = 300
run_a10g = False
run_amd = True

if run_a10g:
    maxdemospace = 0
    local_group_size = 16
else:
    maxdemospace = 0
    local_group_size = 64

today = datetime.now()
temp_dir = today.strftime("%d_%m_%Y_%H_%M_%S_bench_results/")

if os.path.isdir(temp_dir):
    print ("Temp dir: {d} exists already".format(d=temp_dir))
else:
    os.mkdir(temp_dir, 0o755)

region = "us-east-1"
ec2 = boto3.resource('ec2', region_name=region)
ec2_client = boto3.client('ec2', region_name=region)

userdata = """#cloud-config
    runcmd:
     - /home/ec2-user/sudo npm run prod
     - cd /tmp
     - curl https://amazon-ssm-%s.s3.amazonaws.com/latest/linux_amd64/amazon-ssm-agent.rpm -o amazon-ssm-agent.rpm
     - yum install -y amazon-ssm-agent.rpm
     - yum install -y git
     - yum install -y htop
     - yum install -y gcc
     - yum install -y golang
     - yum install -y curl
     - yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
     - yum update -y
     - yum install -y ocl*
     - curl https://sh.rustup.rs -sSf | sh -s -- -y
     - ~/.cargo/bin/rustup target add wasm32-wasi
     - git clone https://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:x-oauth-basic@github.com/SamGinzburg/VectorVisor.git
     - cd /tmp/VectorVisor/
     - ~/.cargo/bin/cargo build --release
     - cd benchmarks/
     - cd json-compression-lz4/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd json-compression/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd average/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd pbkdf2/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd nlp-count-vectorizer/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd imageblur/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd imagehash/
     - ~/.cargo/bin/cargo build --release
""" % region


userdata_ubuntu = """#cloud-config
    runcmd:
     - whoami
     - sudo su
     - sudo whoami
     - export HOME=/root
     - export CUDA_CACHE_MAXSIZE=4294967296
     - export CUDA_CACHE_PATH=~/.nv/ComputeCache/
     - cd /tmp
     - sudo apt update
     - sudo apt install -y git
     - sudo apt install -y git-lfs
     - sudo apt install -y htop
     - sudo apt install -y gcc
     - sudo apt install -y curl
     - sudo apt install -y clinfo
     - wget https://golang.org/dl/go1.17.1.linux-amd64.tar.gz
     - rm -rf /usr/local/go && tar -C /usr/local -xzf go1.17.1.linux-amd64.tar.gz
     - sudo curl https://sh.rustup.rs -sSf | sh -s -- -y
     - . $HOME/.cargo/env
     - sudo ~/.cargo/bin/rustup target add wasm32-wasi
     - git clone https://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:x-oauth-basic@github.com/SamGinzburg/VectorVisor.git
     - wget https://github.com/WebAssembly/binaryen/releases/download/version_109/binaryen-version_109-x86_64-linux.tar.gz
     - tar -xzvf binaryen-version_109-x86_64-linux.tar.gz
     - cargo install wasm-snip
     - cd /tmp/VectorVisor/
     - sudo ~/.cargo/bin/cargo build --release
     - cd benchmarks/
     - mkdir -p ~/.nv/ComputeCache/
     - cd json-compression-lz4/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/json-compression.wasm {snip_args} -o target/wasm32-wasi/release/json-compression.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/json-compression.wasm {opt} -c -o target/wasm32-wasi/release/json-compression-opt.wasm
     - cd ..
     - cd json-compression/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/json-compression.wasm {snip_args} -o target/wasm32-wasi/release/json-compression.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/json-compression.wasm {opt} -c -o target/wasm32-wasi/release/json-compression-opt.wasm
     - cd ..
     - cd average/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/average.wasm {snip_args} -o target/wasm32-wasi/release/average.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/average.wasm {opt} -c -o target/wasm32-wasi/release/average-opt.wasm
     - cd ..
     - cd pbkdf2/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/pbkdf2.wasm {snip_args} -o target/wasm32-wasi/release/pbkdf2.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/pbkdf2.wasm {opt} -c -o target/wasm32-wasi/release/pbkdf2-opt.wasm
     - cd ..
     - cd scrypt/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/scrypt.wasm {snip_args} -o target/wasm32-wasi/release/scrypt.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/scrypt.wasm {opt} -c -o target/wasm32-wasi/release/scrypt-opt.wasm
     - cd ..
     - cd nlp-count-vectorizer/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/nlp-count-vectorizer.wasm {snip_args} -o target/wasm32-wasi/release/nlp-count-vectorizer.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/nlp-count-vectorizer.wasm {opt} -c -o target/wasm32-wasi/release/nlp-count-vectorizer-opt.wasm
     - cd ..
     - cd imageblur/
     - sudo ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/imageblur.wasm {snip_args} --snip-rust-fmt-code -o target/wasm32-wasi/release/imageblur.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/imageblur.wasm {opt} -c -o target/wasm32-wasi/release/imageblur-opt.wasm
     - cd ..
     - cd imageblur-bmp/
     - ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/imageblur-bmp.wasm {snip_args} --snip-rust-fmt-code -o target/wasm32-wasi/release/imageblur-bmp.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/imageblur-bmp.wasm {opt} -c -o target/wasm32-wasi/release/imageblur-opt.wasm
     - cd ..
     - cd imagehash/
     - sudo ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/imagehash.wasm {snip_args} --snip-rust-fmt-code -o target/wasm32-wasi/release/imagehash.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/imagehash.wasm {opt} -c -o target/wasm32-wasi/release/imagehash-opt.wasm
     - cd ..
     - cd imagehash-modified/
     - sudo ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/imagehash-modified.wasm {snip_args} --snip-rust-fmt-code -o target/wasm32-wasi/release/imagehash-modified.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/imagehash-modified.wasm {opt} -c -o target/wasm32-wasi/release/imagehash-opt.wasm
     - cd ..
     - cd genpdf/
     - sudo ~/.cargo/bin/cargo build --release
     - ~/.cargo/bin/wasm-snip target/wasm32-wasi/release/genpdf.wasm {snip_args} -o target/wasm32-wasi/release/genpdf.wasm -p {snip_custom}
     - /tmp/binaryen-version_109/bin/wasm-opt target/wasm32-wasi/release/genpdf.wasm {opt} -c -o target/wasm32-wasi/release/genpdf-opt.wasm
""".format(opt=OPT_LEVEL, snip_args=WASM_SNIP_ARGS, snip_custom=WASM_SNIP_CUSTOM)


def run_command(command, command_name, instance_id):
    while True:
        try:
            response = ssm_client.send_command(
                    InstanceIds=[instance_id],
                    DocumentName="AWS-RunShellScript",
                    Parameters={'commands': [command, ], 'executionTimeout': [str(60*TIMEOUT_MINUTES)]})
            print ("Command response: {resp}".format(resp=response))
            break
        except Exception as err:
            print ("Failed to send {command_name} command, with error: {e}".format(command_name=command_name, e=err))
            time.sleep(10)

    command_id = response['Command']['CommandId']

    print ("running SSM command ID to run {command_name}: {id}".format(command_name=command_name, id=command_id))
    return command_id

def block_on_command(command_id, instance_id):
    while True:
        output = ssm_client.get_command_invocation(
            CommandId=command_id,
            InstanceId=str(instance_id),
            )
        if output['Status'] == 'InProgress':
            print ("Command is still running...")
            time.sleep(10)
        else:
            print ("Command has completed with status: " + str(output['Status']))
            return output

# call between benchmarks
def cleanup():
    terminate_gpu = """#!/bin/bash
    sudo su
    curl -X GET {addr}:8000/terminate
    curl -X GET {addr_cpu}:8000/terminate
    """.format(addr=gpu_instance[0].private_dns_name, addr_cpu=cpu_bench_instance[0].private_dns_name)
    command_id = run_command(terminate_gpu, "run invoker for gpu", invoker_instance[0].id)
    time.sleep(10)
    output = block_on_command(command_id, invoker_instance[0].id)
    time.sleep(10)

def run_scrypt_bench():
    # Now we can set up the next benchmark (scrypt)

    if run_a10g:
        vmcount = 6144
    else:
        vmcount = 4096

    run_scrypt_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/scrypt/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/scrypt.log &
    """

    run_scrypt_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/scrypt/target/wasm32-wasi/release/scrypt-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/scrypt.log &
    """.format(fastreply=fastreply)

    run_command(run_scrypt_command_wasmtime, "scrypt_cpu", cpu_bench_instance[0].id)

    run_scrypt_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/scrypt/target/wasm32-wasi/release/scrypt-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --partitions={maxfuncs} --maxloc={maxloc} --serverless=true --vmcount={vmcount} --vmgroups=1 --maxdup=3 --lgroup={lgroup} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> /tmp/scrypt.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=999, maxloc=maxloc*10, vmcount=vmcount)

    run_command(run_scrypt_command, "scrypt_gpu", gpu_instance[0].id)

    # now run the invoker(s) for pbkdf2
    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/scrypt/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/scrypt/run_scrypt.go {addr} 8000 {target_rps} 1 {duration} {hashes}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount*2, duration=benchmark_duration, hashes=256)

    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_scrypt.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_cpu = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/scrypt/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/scrypt/run_scrypt.go {addr} 8000 {target_rps} 1 {duration} {hashes}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration, hashes=256)

    command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"cpu_bench_scrypt.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()

    run_command(run_scrypt_command_x86, "scrypt_cpu_x86", cpu_bench_instance[0].id)
    
    command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"cpu_x86_bench_scrypt.txt", "w") as text_file:
        text_file.write(str(output))



def run_pbkdf2_bench():
    # Now we can set up the next benchmark (pbkdf2)
    if run_a10g:
        vmcount = 6144
    else:
        vmcount = 4096

    run_pbkdf2_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/pbkdf2/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/pbkdf2.log &
    """

    run_pbkdf2_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/pbkdf2/target/wasm32-wasi/release/pbkdf2-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/pbkdf2.log &
    """.format(fastreply=fastreply)

    run_command(run_pbkdf2_command_wasmtime, "pbkdf2_cpu", cpu_bench_instance[0].id)

    run_pbkdf2_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/pbkdf2/target/wasm32-wasi/release/pbkdf2-opt.wasm --maxdup=0 --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=16384 --partition=false --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=200 &> /tmp/pbkdf2.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=999, maxloc=maxloc*10, vmcount=vmcount)

    run_command(run_pbkdf2_command, "pbkdf2_gpu", gpu_instance[0].id)

    # now run the invoker(s) for pbkdf2
    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/pbkdf2/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount*2, duration=benchmark_duration)

    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_cpu = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/pbkdf2/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration)

    command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"cpu_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()

    run_command(run_pbkdf2_command_x86, "pbkdf2_cpu_x86", cpu_bench_instance[0].id)
    
    command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"cpu_x86_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))

    # we need to kill the running VV instance first
    cleanup()

    run_invoker_hashcat = """#!/bin/bash
	    sudo su
	    ulimit -n 65536
	    apt install -y hashcat
	    hashcat -b -m 9200
	    """

    command_id = run_command(run_invoker_hashcat, "run hashcat", gpu_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)
    with open(temp_dir+"hashcat_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))


def run_lz4_bench():
    if run_a10g:
        vmcount = 4608
    else:
        vmcount = 3072

    run_json_lz4_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/json-compression-lz4/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/json-compression.log &
    """.format(fastreply=fastreply)

    run_json_lz4_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/json-compression-lz4/target/wasm32-wasi/release/json-compression-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=524288 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/json-compression.log &
    """.format(fastreply=fastreply)

    run_command(run_json_lz4_command_wasmtime, "run_json_lz4_command_wasmtime", cpu_bench_instance[0].id)

    run_json_lz4_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/json-compression-lz4/target/wasm32-wasi/release/json-compression-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount={vmcount} --vmgroups=1 --maxdup=3 --partitions={maxfuncs} --maxloc={maxloc} --lgroup={lgroup} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --rt=200 &> /tmp/json-compression.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_json_lz4_command, "run_json_lz4_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/json-compression-lz4/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/json-compression-lz4/run_lz4.go {addr} 8000 {target_rps} 1 {duration} /tmp/VectorVisor/benchmarks/json-compression/smaller_tweets.txt {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=2000, target_rps=vmcount*2, duration=benchmark_duration)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_lz4.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/json-compression-lz4/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/json-compression-lz4/run_lz4.go {addr} 8000 {target_rps} 1 {duration} /tmp/VectorVisor/benchmarks/json-compression-lz4/smaller_tweets.txt {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=2000, target_rps=target_rps_cpu, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_bench_lz4.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()

    run_command(run_json_lz4_command_x86, "run_json_lz4_command_x86", cpu_bench_instance[0].id)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)

    print (output)
    # save output
    with open(temp_dir+"cpu_x86_bench_lz4.txt", "w") as text_file:
        text_file.write(str(output))

def run_genpdf_bench():
    if run_a10g:
        vmcount = 5120
    else:
        vmcount = 4096

    run_genpdf_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/genpdf/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/genpdf.log &
    """.format(fastreply=fastreply)

    run_genpdf_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/genpdf/target/wasm32-wasi/release/genpdf-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/genpdf.log &
    """.format(fastreply=fastreply)

    run_command(run_genpdf_command_wasmtime, "run_average_command_wasmtime", cpu_bench_instance[0].id)

    run_genpdf_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/genpdf/target/wasm32-wasi/release/genpdf-opt.wasm --ip=0.0.0.0 --heap=3000000 --stack=131072 --hcallsize=200000 --partition=true --serverless=true --vmcount={vmcount} --wasmtime=false --maxdup=2 --lgroup={lgroup} --partitions={maxfuncs} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --rt=200 &> /tmp/genpdf.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_genpdf_command, "run_average_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/genpdf/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/genpdf/run_genpdf.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount*2, duration=benchmark_duration)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_genpdf.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/genpdf/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/genpdf/run_genpdf.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_bench_genpdf.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()

    run_command(run_genpdf_command_x86, "run_genpdf_command_x86", cpu_bench_instance[0].id)
    
    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_x86_bench_genpdf.txt", "w") as text_file:
        text_file.write(str(output))

def run_average_bench():
    if run_a10g:
        vmcount = 5120
    else:
        vmcount = 4096

    run_average_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/average/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/average.log &
    """.format(fastreply=fastreply)

    run_average_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/average/target/wasm32-wasi/release/average-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/average.log &
    """.format(fastreply=fastreply)

    run_command(run_average_command_wasmtime, "run_average_command_wasmtime", cpu_bench_instance[0].id)

    run_average_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/average/target/wasm32-wasi/release/average-opt.wasm --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount={vmcount} --wasmtime=false --maxdup=3 --lgroup={lgroup} --partitions={maxfuncs} --maxloc={maxloc} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --rt=200 &> /tmp/average.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_average_command, "run_average_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/average/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/average/run_average_bench.go {addr} 8000 {target_rps} 1 {duration} {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=20, target_rps=vmcount*2, duration=benchmark_duration)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_average.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/average/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/average/run_average_bench.go {addr} 8000 {target_rps} 1 {duration} {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=20, target_rps=target_rps_cpu, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_bench_average.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()

    run_command(run_average_command_x86, "run_average_command_x86", cpu_bench_instance[0].id)
    
    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_x86_bench_average.txt", "w") as text_file:
        text_file.write(str(output))

def run_image_hash_bench(run_modified = False):
    if run_a10g:
        vmcount = 4096
    else:
        vmcount = 3072
    
    imagehash_path = "/tmp/VectorVisor/benchmarks/imagehash/"
    if run_modified:
        imagehash_path = "/tmp/VectorVisor/benchmarks/imagehash-modified/"

    run_image_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done
    
    cd {imagehash_path}
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/imagehash.log &
    """.format(fastreply=fastreply, imagehash_path=imagehash_path)

    run_image_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input {imagehash_path}target/wasm32-wasi/release/imagehash-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount=3072 --wasmtime=true --fastreply={fastreply} &> /tmp/imagehash.log &
    """.format(fastreply=fastreply, imagehash_path=imagehash_path)

    run_command(run_image_command_wasmtime, "run_imagehash_command_wasmtime", cpu_bench_instance[0].id)

    run_image_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input {imagehash_path}target/wasm32-wasi/release/imagehash-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount={vmcount} --vmgroups=1 --maxdup=2 --partitions={maxfuncs} --maxloc={maxloc} --lgroup={lgroup} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --rt=200 &> /tmp/imagehash.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, imagehash_path=imagehash_path, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_image_command, "run_imagehash_gpu_command", gpu_instance[0].id)

    # Now set up the invoker

    if not run_modified:
        vmcount = vmcount*2

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {imagehash_path}

    /usr/local/go/bin/go run run_image_hash.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=vmcount, imagehash_path=imagehash_path, duration=benchmark_duration)

    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    if run_modified:
        with open(temp_dir+"gpu_bench_imagehash_modified.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"gpu_bench_imagehash.txt", "w") as text_file:
            text_file.write(str(output))

    run_command(run_image_command_wasmtime, "run_imagehash_command_x86", cpu_bench_instance[0].id)

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {imagehash_path}

    /usr/local/go/bin/go run run_image_hash.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=1000, target_rps=target_rps_cpu, imagehash_path=imagehash_path, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    if run_modified:
        with open(temp_dir+"cpu_bench_imagehash_modified.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"cpu_bench_imagehash.txt", "w") as text_file:
            text_file.write(str(output))

    cleanup()

    run_command(run_image_command_x86, "run_imagehash_command_x86", cpu_bench_instance[0].id)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    if run_modified:
        with open(temp_dir+"cpu_x86_bench_imagehash_modified.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"cpu_x86_bench_imagehash.txt", "w") as text_file:
            text_file.write(str(output))

def run_image_blur_bench(run_bmp = False):
    if run_a10g:
        vmcount = 4096
    else:
        vmcount = 3072

    if not run_bmp:
        bin_path = "/tmp/VectorVisor/benchmarks/imageblur/target/wasm32-wasi/release/imageblur-opt.wasm"
        exe_path = "/tmp/VectorVisor/benchmarks/imageblur/"
    else:
        bin_path = "/tmp/VectorVisor/benchmarks/imageblur-bmp/target/wasm32-wasi/release/imageblur-opt.wasm"
        exe_path = "/tmp/VectorVisor/benchmarks/imageblur-bmp/"

    run_image_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {bin_path}
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/imageblur.log &
    """.format(fastreply=fastreply, bin_path=exe_path)

    run_image_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input {bin_path} --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=409600 --partition=true --serverless=true --vmcount=3072 --wasmtime=true --fastreply={fastreply} &> /tmp/imageblur.log &
    """.format(fastreply=fastreply, bin_path=bin_path)

    run_command(run_image_command_wasmtime, "run_imageblur_command_wasmtime", cpu_bench_instance[0].id)

    run_image_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input {bin_path} --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=409600 --partition=true --serverless=true --vmcount={vmcount} --vmgroups=1 --maxdup=2 --disablefastcalls=false --partitions={maxfuncs} --maxloc={maxloc} --lgroup={lgroup} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> /tmp/imageblur.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, bin_path=bin_path, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_image_command, "run_imageblur_gpu_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {exe_path}

    /usr/local/go/bin/go run run_image_blur.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=vmcount*2, exe_path=exe_path, duration=benchmark_duration)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    if not run_bmp:
        with open(temp_dir+"gpu_bench_imageblur.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"gpu_bench_imageblur_bmp.txt", "w") as text_file:
            text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {exe_path}

    /usr/local/go/bin/go run run_image_blur.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=1000, target_rps=target_rps_cpu, exe_path=exe_path, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    if not run_bmp:
        with open(temp_dir+"cpu_bench_imageblur.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"cpu_bench_imageblur_bmp.txt", "w") as text_file:
            text_file.write(str(output))


    cleanup()

    run_command(run_image_command_x86, "run_imageblur_command_x86", cpu_bench_instance[0].id)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    if not run_bmp:
        with open(temp_dir+"cpu_x86_bench_imageblur.txt", "w") as text_file:
            text_file.write(str(output))
    else:
        with open(temp_dir+"cpu_x86_bench_imageblur_bmp.txt", "w") as text_file:
            text_file.write(str(output))

def run_nlp_count_bench():
    if run_a10g:
        vmcount = 4608
    else:
        vmcount = 3072

    run_nlp_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/nlp-count-vectorizer.log &
    """.format(fastreply=fastreply)

    run_nlp_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/target/wasm32-wasi/release/nlp-count-vectorizer-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=524288 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /tmp/nlp-count-vectorizer.log &
    """.format(fastreply=fastreply)

    run_command(run_nlp_command_wasmtime, "run_nlp_command_wasmtime", cpu_bench_instance[0].id)

    run_nlp_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/target/wasm32-wasi/release/nlp-count-vectorizer-opt.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=524288 --partition=true --serverless=true --vmcount={vmcount} --vmgroups=1 --maxdup=3 --disablefastcalls=false --lgroup={lgroup} --partitions={maxfuncs} --maxloc={maxloc} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> /tmp/nlp-count-vectorizer.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    run_command(run_nlp_command, "run_nlp_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 {duration} /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=500, target_rps=vmcount, duration=benchmark_duration)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_bench_nlp.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/

    /usr/local/go/bin/go run /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 {duration} /tmp/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=500, target_rps=target_rps_cpu, duration=benchmark_duration)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_bench_nlp.txt", "w") as text_file:
        text_file.write(str(output))

    cleanup()
    run_command(run_nlp_command_x86, "run_nlp_command_x86", cpu_bench_instance[0].id)
    
    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open(temp_dir+"cpu_x86_bench_nlp.txt", "w") as text_file:
        text_file.write(str(output))

def run_membench(membench_interleave=4):
    if run_a10g:
        vmcount = 6144
    else:
        vmcount = 4096

    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/examples/mem/memloop.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    command_id = run_command(run_membench_command, "run_membench", gpu_instance[0].id)

    time.sleep(5)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_membench_{interleave}.txt".format(interleave=membench_interleave), "w") as text_file:
        text_file.write(str(output))

    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/examples/mem/memloop_unroll.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    command_id = run_command(run_membench_command, "run_membench_unroll", gpu_instance[0].id)

    time.sleep(5)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_membench_unroll_{interleave}.txt".format(interleave=membench_interleave), "w") as text_file:
        text_file.write(str(output))


    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/examples/mem/memloop64.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    command_id = run_command(run_membench_command, "run_membench64", gpu_instance[0].id)

    time.sleep(5)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_membench64_{interleave}.txt".format(interleave=membench_interleave), "w") as text_file:
        text_file.write(str(output))

    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/VectorVisor/target/release/vectorvisor --input /tmp/VectorVisor/examples/mem/memloop64_unroll.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --cflags={cflags} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount)

    command_id = run_command(run_membench_command, "run_membench64_unroll", gpu_instance[0].id)

    time.sleep(5)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    # save output
    with open(temp_dir+"gpu_membench64_unroll_{interleave}.txt".format(interleave=membench_interleave), "w") as text_file:
        text_file.write(str(output))


"""
Create VMs for the test
1 GPU VM, 1 CPU VM, and 1 VM for issuing requests

g4dn.xlarge  => 1 T4, 16 GiB memory,  4 vCPU, $0.526 / hr
g4dn.2xlarge => 1 T4, 32 GiB memory, 8 vCPU, $0.752 / hr
g4dn.4xlarge => 1 T4, 64 GiB memory, 16 vCPU, $1.204 / hr
p3.2xlarge   => 1 V100, 16 GiB memory, 8 vCPU, $3.06 / hr
g5.xlarge    => 1 A10G, 24GiB memory, 4 vCPU, $1.006 / hr

"""
# AMIs specific to us-east-2
# ami-01463836f7041cd10  ==> OpenCL 3.0 driver (470.57.02)
# ami-00339339e800db52e  ==> OpenCL 1.2 driver (460.X)
# ami-0748c95fd9dd9f42a  ==> OpenCL 1.2 driver (450.X)


"""
us-east-2 AMI: ami-01463836f7041cd10
us-east-1 AMI: ami-094c089c38ed069f2 
"""

if region == "us-east-1":
    #gpu_ami = 'ami-09a83b91fc98e860f'
    gpu_ami = 'ami-02e8976fea9b1f568'
elif region == "us-east-2":
    gpu_ami = 'ami-01463836f7041cd10'

if run_a10g:
    gpuinstance = "g5.xlarge"
else:
    gpuinstance = "g4dn.xlarge"


gpu_instance = ec2.create_instances(ImageId=gpu_ami,
                                InstanceType=gpuinstance,
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata_ubuntu,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

# cpu wasmtime instance
# c5.xlarge (0.17), c5a.xlarge (0.154)

"""
us-east-2 AMI: ami-028dbc12531690cf4
us-east-1 AMI: ami-083654bd07b5da81d
"""

if region == "us-east-1":
    cpu_ami = 'ami-09a83b91fc98e860f'
elif region == "us-east-2":
    cpu_ami = 'ami-028dbc12531690cf4'

if run_amd:
    cpu_vm = "c5a.xlarge"
else:
    cpu_vm = "c5.xlarge"

cpu_bench_instance = ec2.create_instances(ImageId=cpu_ami,
                                InstanceType=cpu_vm,
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata_ubuntu,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })


# t2.2xlarge = 8 vCPUs, $0.37/hr
# c5.4xlarge = 16 vCPUs, $0.68/hr
invoker_instance = ec2.create_instances(ImageId=cpu_ami,
                                InstanceType="c5a.8xlarge",
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata_ubuntu,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

print ("Started: " + str(invoker_instance) + " with id: " + str(invoker_instance[0].id))
print ("Started: " + str(gpu_instance) + " with id: " + str(gpu_instance[0].id))
print ("Started: " + str(cpu_bench_instance) + " with id: " + str(cpu_bench_instance[0].id))

instance_id_list = [invoker_instance[0].id, gpu_instance[0].id, cpu_bench_instance[0].id]
print ("Instance id list: ", instance_id_list)

print ("now waiting...")
invoker_instance[0].wait_until_running()
gpu_instance[0].wait_until_running()
cpu_bench_instance[0].wait_until_running()
print ("Instances are now running")

invoker_instance[0].load()
gpu_instance[0].load()
cpu_bench_instance[0].load()

print("CPU instance private addr: ", invoker_instance[0].private_dns_name)
print("GPU instance private addr: ", gpu_instance[0].private_dns_name)
print("CPU bench instance private addr: ", cpu_bench_instance[0].private_dns_name)


# Wait until initialization is complete
while True:
    resp = ec2_client.describe_instance_status(InstanceIds=instance_id_list)
    done_waiting = True
    for status in resp['InstanceStatuses']:
        if status['InstanceStatus']['Status'] != 'ok':
            done_waiting = False
    if done_waiting:
        break
    else:
        print ("Still waiting on allocated VMs to finish waiting...")
        time.sleep(10)

ssm_client = boto3.client('ssm', region_name=region)

run_membench(membench_interleave=1)

cleanup()

run_membench(membench_interleave=4)

cleanup()

run_membench(membench_interleave=8)

cleanup()

# run image hash bench
run_image_hash_bench(run_modified = False)

cleanup()

# run image hash bench

run_image_hash_bench(run_modified = True)

cleanup()

# run scrypt bench
run_scrypt_bench()

cleanup()

# run lz4 bench
run_lz4_bench()

cleanup()

# run NLP bench
run_nlp_count_bench()

cleanup()

# run average bench
run_average_bench()

cleanup()

# run image blue bench
run_image_blur_bench(run_bmp = True)

cleanup()

run_image_blur_bench(run_bmp = False)

cleanup()

#run_genpdf_bench()
#cleanup()

# run pbkdf2 bench
# pbkdf2 needs to be run last because it also installs hashcat / pocl to benchmark against at the end
run_pbkdf2_bench()

cleanup()

# clean up all instances at end
ec2.instances.filter(InstanceIds = instance_id_list).terminate()
