import boto3
import time
import os
from datetime import date, datetime
import re
import argparse

parser = argparse.ArgumentParser(description='run benchmarks')
parser.add_argument("--gpu", required=True)
parser.add_argument("--cpu", required=True)
parser.add_argument("--interleave", required=True)
parser.add_argument("--membench", required=False)
parser.add_argument("--breakdown", required=False)
parser.add_argument("--dir", required=False)
parser.add_argument("--skip-membench", required=False)
parser.add_argument("--skip-cpu", required=False)
parser.add_argument("--run-profile", required=False)
parser.add_argument("--ami", required=True)
parser.add_argument("--cpuami", required=True)

args = vars(parser.parse_args())

ami = args['ami']
cpuami = args['cpuami']
gpu = args['gpu']
cpu = args['cpu']
interleave = args['interleave']
membench = args['membench']
breakdown = args['breakdown']
outdir = args['dir']
skip_membench = args['skip_membench']
skip_cpu = args['skip_cpu']
run_profile = args['run_profile']

if skip_cpu == None:
    skip_cpu = False

print ("ami: ", ami)
print ("cpuami: ", cpuami)
print ("gpu: ", gpu)
print ("cpu: ", cpu)
print ("interleave: ", interleave)
print ("membench: ", membench)
print ("run latency breakdown: ", breakdown)
print ("dir: ", outdir)
print ("skip-membench: ", skip_membench)
print ("run-profile: ", run_profile)

if run_profile:
    run_profile = "-profile"
else:
    run_profile = ""

if gpu == "t4":
    run_a10g = False
else:
    run_a10g = True

if cpu == "intel":
    run_amd = False
else:
    run_amd = True

if breakdown:
    run_latency_breakdown = True
else:
    run_latency_breakdown = False

if membench:
    run_only_membench = True
else:
    run_only_membench = False


# Benchmark constants
# target rps is really just the number of concurrent invokers
# this affects the *possible* max RPS and bandwidth/mem/cpu consumption of the invoker
target_rps = 3072
target_rps_cpu = 1024
TIMEOUT_MINUTES = 60 * 4
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

if gpu == "a10g":
    maxdemospace = 0
    local_group_size = 64
    nvflag = "true"
    somaxconn = "65535"
elif gpu == "t4":
    maxdemospace = 0
    local_group_size = 64
    nvflag = "true"
    somaxconn = "65535"
elif gpu == "amd":
    maxdemospace = 0
    local_group_size = 64
    nvflag = "false"
    somaxconn = "65535"

today = datetime.now()

if outdir is None:
    temp_dir = today.strftime("%d_%m_%Y_%H_%M_%S_bench_results_{gpu}_{cpu}_{interleave}/".format(gpu=gpu, cpu=cpu, interleave=interleave))
else:
    temp_dir = outdir

if os.path.isdir(temp_dir):
    print ("Temp dir: {d} exists already".format(d=temp_dir))
else:
    os.mkdir(temp_dir, 0o755)

region = "us-east-1"
ec2 = boto3.resource('ec2', region_name=region)
ec2_client = boto3.client('ec2', region_name=region)

if run_only_membench:
    userdata_ubuntu = """#cloud-config
    runcmd:
     - whoami
     - sudo su
     - sudo whoami
     - sysctl -w net.ipv4.tcp_max_syn_backlog=3240000
     - sysctl -w net.core.netdev_max_backlog=16384
     - sysctl -w net.core.somaxconn={somaxconn}
     - sysctl -w net.core.wmem_default=67108864
     - sysctl -w net.core.rmem_default=67108864
     - sysctl -w net.ipv4.tcp_rmem="4096 131072 67108864"
     - sysctl -w net.ipv4.tcp_wmem="4096 131072 67108864"
     - sysctl -w net.ipv4.tcp_mem="268435456 268435456 268435456"
     - sysctl -p
     - export HOME=/root
     - export CUDA_CACHE_MAXSIZE=4294967296
     - export CUDA_CACHE_PATH=~/.nv/ComputeCache/
     - cd /vv/VectorVisor/
     - git pull
     - ~/.cargo/bin/cargo build --release
""".format(opt=OPT_LEVEL, snip_args=WASM_SNIP_ARGS, snip_custom=WASM_SNIP_CUSTOM, somaxconn=somaxconn)
else:
    userdata_ubuntu = """#cloud-config
    runcmd:
     - whoami
     - sudo su
     - sudo whoami
     - sysctl -w net.ipv4.tcp_max_syn_backlog=3240000
     - sysctl -w net.core.netdev_max_backlog=16384
     - sysctl -w net.core.somaxconn={somaxconn}
     - sysctl -w net.core.wmem_default=67108864
     - sysctl -w net.core.rmem_default=67108864
     - sysctl -w net.ipv4.tcp_rmem="4096 131072 67108864"
     - sysctl -w net.ipv4.tcp_wmem="4096 131072 67108864"
     - sysctl -w net.ipv4.tcp_mem="268435456 268435456 268435456"
     - sysctl -p
     - export HOME=/root
     - export CUDA_CACHE_MAXSIZE=4294967296
     - export CUDA_CACHE_PATH=~/.nv/ComputeCache/
     - cd /vv/VectorVisor/
     - git pull
     - ~/.cargo/bin/cargo build --release
""".format(opt=OPT_LEVEL, snip_args=WASM_SNIP_ARGS, snip_custom=WASM_SNIP_CUSTOM, somaxconn=somaxconn)

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

    if gpu == "a10g":
        vmcount = 6144
        prefix = ""
    elif gpu == "t4":
        vmcount = 4096
        prefix = ""
    elif gpu == "amd":
        vmcount = 2048
        prefix = ""

    run_scrypt_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/scrypt/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/scrypt.log &
    """

    run_scrypt_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/scrypt-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /vv/scrypt.log &
    """.format(fastreply=fastreply, interleave=interleave, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_scrypt_command_wasmtime, "scrypt_cpu", cpu_bench_instance[0].id)

    run_scrypt_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}scrypt-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=false --serverless=true --vmcount={vmcount} --vmgroups=1 --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --lgroup={lgroup} --nvidia={nv} &> /vv/scrypt.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=999, maxloc=maxloc*10, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag)

    run_command(run_scrypt_command, "scrypt_gpu", gpu_instance[0].id)

    if not run_latency_breakdown:
        vmcount=vmcount*2

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

    cd /vv/VectorVisor/benchmarks/scrypt/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/scrypt/run_scrypt.go {addr} 8000 {target_rps} 1 {duration} {hashes}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount, duration=benchmark_duration, hashes=256)
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_scrypt_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))

        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/scrypt/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/scrypt/run_scrypt.go {addr} 8000 {target_rps} 1 {duration} {hashes}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration, hashes=256)
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"cpu_bench_scrypt_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_scrypt_command_x86, "scrypt_cpu_x86", cpu_bench_instance[0].id)
        
        command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"cpu_x86_bench_scrypt_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)




def run_pbkdf2_bench():
    # Now we can set up the next benchmark (pbkdf2)
    if gpu == "a10g":
        vmcount = 6144
        prefix = ""
    elif gpu == "t4":
        vmcount = 4096
        prefix = ""
    elif gpu == "amd":
        vmcount = 2048
        prefix = ""

    run_pbkdf2_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/pbkdf2/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/pbkdf2.log &
    """

    run_pbkdf2_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/pbkdf2-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /vv/pbkdf2.log &
    """.format(fastreply=fastreply, interleave=interleave, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_pbkdf2_command_wasmtime, "pbkdf2_cpu", cpu_bench_instance[0].id)

    run_pbkdf2_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}pbkdf2-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=16384 --partition=false --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=100 --lgroup={lgroup} --nvidia={nv} &> /vv/pbkdf2.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=999, maxloc=maxloc*10, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag)

    run_command(run_pbkdf2_command, "pbkdf2_gpu", gpu_instance[0].id)
    
    if not run_latency_breakdown:
        vmcount = vmcount*2

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

    cd /vv/VectorVisor/benchmarks/pbkdf2/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount, duration=benchmark_duration)
    
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_pbkdf2_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/pbkdf2/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"cpu_bench_pbkdf2_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_pbkdf2_command_x86, "pbkdf2_cpu_x86", cpu_bench_instance[0].id)
        
        command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"cpu_x86_bench_pbkdf2_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    # we need to kill the running VV instance first
    cleanup()

def run_lz4_bench():
    if gpu == "a10g":
        vmcount = 4096
        prefix = ""
    elif gpu == "t4":
        vmcount = 3072
        prefix = ""
    elif gpu == "amd":
        vmcount = 1536
        prefix = ""

    run_json_lz4_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/json-compression/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/json-compression.log &
    """.format(fastreply=fastreply)

    run_json_lz4_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/json-compression-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=524288 --partition=false --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /vv/json-compression.log &
    """.format(fastreply=fastreply, interleave=interleave, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_json_lz4_command_wasmtime, "run_json_lz4_command_wasmtime", cpu_bench_instance[0].id)

    run_json_lz4_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}json-compression-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=204800 --partition=false --serverless=true --vmcount={vmcount} --vmgroups=1 --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=100 --lgroup={lgroup} --nvidia={nv} &> /vv/json-compression.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag)

    run_command(run_json_lz4_command, "run_json_lz4_command", gpu_instance[0].id)

    # Now set up the invoker
    if not run_latency_breakdown:
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

    cd /vv/VectorVisor/benchmarks/json-compression/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/json-compression/run_lz4.go {addr} 8000 {target_rps} 1 {duration} /vv/VectorVisor/benchmarks/json-compression/smaller_tweets.txt {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=vmcount, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_lz4_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/json-compression/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/json-compression/run_lz4.go {addr} 8000 {target_rps} 1 {duration} /vv/VectorVisor/benchmarks/json-compression/smaller_tweets.txt {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=1000, target_rps=target_rps_cpu, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_bench_lz4_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_json_lz4_command_x86, "run_json_lz4_command_x86", cpu_bench_instance[0].id)

        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)

        print (output)
        # save output
        with open(temp_dir+"cpu_x86_bench_lz4_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))

def run_genpdf_bench():
    if gpu == "a10g":
        vmcount = 4096
        prefix=""
    elif gpu == "t4":
        vmcount = 3072
        prefix=""
    elif gpu == "amd":
        vmcount = 1536
        prefix=""

    run_genpdf_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/rust-pdfwriter/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/rust-pdfwriter.log &
    """.format(fastreply=fastreply)

    run_genpdf_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/rust-pdfwriter-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=131072 --partition=false --serverless=true --vmcount={vmcount} --wasmtime=true --fastreply={fastreply} &> /vv/rust-pdfwriter.log &
    """.format(fastreply=fastreply, interleave=interleave, vmcount=vmcount, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_genpdf_command_wasmtime, "run_genpdf_command_wasmtime", cpu_bench_instance[0].id)

    run_genpdf_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}rust-pdfwriter-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=131072 --partition=false --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=200 --lgroup={lgroup} --nvidia={nv} &> /vv/rust-pdfwriter.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag)

    run_command(run_genpdf_command, "run_rust-pdfwriter_command", gpu_instance[0].id)

    # Now set up the invoker
    if not run_latency_breakdown:
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

    cd /vv/VectorVisor/benchmarks/rust-pdfwriter/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/rust-pdfwriter/run_genpdf.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_genpdf_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/rust-pdfwriter/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/rust-pdfwriter/run_genpdf.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps_cpu, duration=benchmark_duration)
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_bench_genpdf_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_genpdf_command_x86, "run_genpdf_command_x86", cpu_bench_instance[0].id)
        
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_x86_bench_genpdf_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


def run_average_bench():
    if gpu == "a10g":
        vmcount = 5120
        prefix = ""
    elif gpu == "t4":
        vmcount = 4096
        prefix = ""
    elif gpu == "amd":
        vmcount = 2048
        prefix = ""

    run_average_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/average/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/average.log &
    """.format(fastreply=fastreply)

    run_average_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/average-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=262144 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /vv/average.log &
    """.format(fastreply=fastreply, interleave=interleave, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_average_command_wasmtime, "run_average_command_wasmtime", cpu_bench_instance[0].id)

    run_average_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}average-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=262144 --partition=false --serverless=true --vmcount={vmcount} --wasmtime=false  --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=100 --lgroup={lgroup} --nvidia={nv} &> /vv/average.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, \
               maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag)

    run_command(run_average_command, "run_average_command", gpu_instance[0].id)

    # Now set up the invoker
    if not run_latency_breakdown:
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

    cd /vv/VectorVisor/benchmarks/average/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/average/run_average_bench.go {addr} 8000 {target_rps} 1 {duration} {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=20, target_rps=vmcount, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_average_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/average/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/average/run_average_bench.go {addr} 8000 {target_rps} 1 {duration} {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=20, target_rps=target_rps_cpu, duration=benchmark_duration)
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_bench_average_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()
    for idx in range(NUM_REPEAT):
        run_command(run_average_command_x86, "run_average_command_x86", cpu_bench_instance[0].id)
        
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_x86_bench_average_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


def run_image_hash_bench(run_modified = False):
    if gpu == "a10g":
        vmcount = 4096
        prefix = ""
    elif gpu == "t4":
        vmcount = 3072
        prefix = ""
    elif gpu == "amd":
        vmcount = 1536
        prefix = ""
    
    imagehash_path = "/vv/VectorVisor/benchmarks/{prefix}imagehash".format(prefix=prefix)
    hcallsize = "65536"
    if run_modified:
        imagehash_path = "/vv/VectorVisor/benchmarks/{prefix}imagehash-modified".format(prefix=prefix)
        hcallsize = "266240"

    run_image_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done
    
    cd {imagehash_path}
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/imagehash.log &
    """.format(fastreply=fastreply, imagehash_path=imagehash_path)

    run_image_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input {imagehash_path}-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize=294912 --partition=true --serverless=true --vmcount=3072 --wasmtime=true --fastreply={fastreply} &> /vv/imagehash.log &
    """.format(fastreply=fastreply, imagehash_path=imagehash_path, interleave=interleave, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_image_command_wasmtime, "run_imagehash_command_wasmtime", cpu_bench_instance[0].id)

    run_image_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input {imagehash_path}-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=4194304 --stack=131072 --hcallsize={hc} --partition=false --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --rt=100 --lgroup={lgroup} --nvidia={nv} &> /vv/imagehash.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, imagehash_path=imagehash_path, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, prefix=prefix, run_profile=run_profile, nv=nvflag, hc=hcallsize)

    run_command(run_image_command, "run_imagehash_gpu_command", gpu_instance[0].id)

    # Now set up the invoker

    if not run_latency_breakdown:
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

    cd {imagehash_path}/

    /usr/local/go/bin/go run run_image_hash.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=vmcount, imagehash_path=imagehash_path, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        if run_modified:
            with open(temp_dir+"gpu_bench_imagehash_modified_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"gpu_bench_imagehash_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))

        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd {imagehash_path}/

    /usr/local/go/bin/go run run_image_hash.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=1000, target_rps=target_rps_cpu, imagehash_path=imagehash_path, duration=benchmark_duration)
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        if run_modified:
            with open(temp_dir+"cpu_bench_imagehash_modified_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"cpu_bench_imagehash_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))

        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_image_command_x86, "run_imagehash_command_x86", cpu_bench_instance[0].id)

        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        if run_modified:
            with open(temp_dir+"cpu_x86_bench_imagehash_modified_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"cpu_x86_bench_imagehash_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))

        time.sleep(SLEEP_TIME)

    if run_modified:
        run_cuda_command = """#!/bin/bash
            sudo su
            ulimit -n 65536
            x=$(cloud-init status)
            until [ "$x" == "status: done" ]; do
            sleep 10
            x=$(cloud-init status)
            done

            cd {bin_path}
            cd kernel
            make
            cd ..
            ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/imagehash_cuda.log &
            """.format(bin_path="/vv/VectorVisor/benchmarks/cuda-blockhash/")
            
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

            cd {exe_path}/

            /usr/local/go/bin/go run run_image_hash.go {addr} 8000 {target_rps} 1 {duration}
            """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=256, exe_path=imagehash_path, duration=benchmark_duration)

        run_command(run_cuda_command, "run_imagehash_cuda_gpu_command", gpu_instance[0].id)

        for idx in range(NUM_REPEAT): 
            command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

            time.sleep(20)

            # Block until benchmark is complete
            output = block_on_command(command_id, invoker_instance[0].id)
            print (output)

            # save output
            with open(temp_dir+"gpu_cuda_bench_imagehash_bmp_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
            time.sleep(SLEEP_TIME)

    cleanup()

def run_image_blur_bench(run_bmp = False):
    if gpu == "a10g":
        vmcount = 4096
        prefix = ""
    elif gpu == "t4":
        vmcount = 3072
        prefix = ""
    elif gpu == "amd":
        vmcount = 1536
        prefix = ""

    if not run_bmp:
        bin_path = "/vv/VectorVisor/benchmarks/{prefix}imageblur-opt-{interleave}{run_profile}.wasm".format(interleave=interleave, prefix=prefix, run_profile=run_profile)
        exe_path = "/vv/VectorVisor/benchmarks/imageblur/"
        hcallsize = "65536"
    else:
        bin_path = "/vv/VectorVisor/benchmarks/{prefix}imageblur-bmp-opt-{interleave}{run_profile}.wasm".format(interleave=interleave, prefix=prefix, run_profile=run_profile)
        exe_path = "/vv/VectorVisor/benchmarks/imageblur-bmp/"
        hcallsize = "225280"

    run_image_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd {bin_path}
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/imageblur.log &
    """.format(fastreply=fastreply, bin_path=exe_path)

    run_image_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input {bin_path} --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=225280 --partition=true --serverless=true --vmcount=3072 --wasmtime=true --fastreply={fastreply} &> /vv/imageblur.log &
    """.format(fastreply=fastreply, bin_path=bin_path)

    if not skip_cpu:
        run_command(run_image_command_wasmtime, "run_imageblur_command_wasmtime", cpu_bench_instance[0].id)

    run_image_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    /vv/VectorVisor/target/release/vectorvisor --input {bin_path}.bin --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize={hc} --partition=false --serverless=true --vmcount={vmcount} --vmgroups=1 --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --lgroup={lgroup} --nvidia={nv} &> /vv/imageblur.log &
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, bin_path=bin_path, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag, hc=hcallsize)

    run_command(run_image_command, "run_imageblur_gpu_command", gpu_instance[0].id)

    # Now set up the invoker
    if not run_latency_breakdown:
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

    cd {exe_path}

    /usr/local/go/bin/go run run_image_blur.go {addr} 8000 {target_rps} 1 {duration}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=vmcount, exe_path=exe_path, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        if not run_bmp:
            with open(temp_dir+"gpu_bench_imageblur_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"gpu_bench_imageblur_bmp_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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
    
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        if not run_bmp:
            with open(temp_dir+"cpu_bench_imageblur_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"cpu_bench_imageblur_bmp_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()
    for idx in range(NUM_REPEAT):
        run_command(run_image_command_x86, "run_imageblur_command_x86", cpu_bench_instance[0].id)

        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        if not run_bmp:
            with open(temp_dir+"cpu_x86_bench_imageblur_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        else:
            with open(temp_dir+"cpu_x86_bench_imageblur_bmp_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
        time.sleep(SLEEP_TIME)
    cleanup()

    if run_bmp:
        run_cuda_command = """#!/bin/bash
            sudo su
            ulimit -n 65536
            x=$(cloud-init status)
            until [ "$x" == "status: done" ]; do
            sleep 10
            x=$(cloud-init status)
            done

            cd {bin_path}
            cd kernel
            make
            cd ..
            ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/imageblur_cuda.log &
            """.format(bin_path="/vv/VectorVisor/benchmarks/cuda-blur/")
            
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
            """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=256, exe_path=exe_path, duration=benchmark_duration)

        run_command(run_cuda_command, "run_imageblur_cuda_gpu_command", gpu_instance[0].id)

        for idx in range(NUM_REPEAT): 
            command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

            time.sleep(20)

            # Block until benchmark is complete
            output = block_on_command(command_id, invoker_instance[0].id)
            print (output)

            # save output
            with open(temp_dir+"gpu_cuda_bench_imageblur_bmp_{idx}.txt".format(idx=idx), "w") as text_file:
                text_file.write(str(output))
            time.sleep(SLEEP_TIME)

    cleanup()

def run_nlp_count_bench(lang):
    if gpu == "a10g":
        vmcount = 6144
        prefix = ""
    elif gpu == "t4":
        vmcount = 4096
        prefix = ""
    elif gpu == "amd":
        vmcount = 2048
        prefix = ""

    if lang == "rust":
        path = "nlp-count-vectorizer"
    elif lang == "go":
        path = "nlp-go"
    elif lang == "assemblyscript":
        path = "nlp-assemblyscript"

    print ("Running nlp lang: ", lang)
    print ("nlp path: ", path)

    run_nlp_command_x86 = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/nlp-count-vectorizer/
    ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /vv/nlp-count-vectorizer.log &
    """.format(fastreply=fastreply)

    run_nlp_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{path}-opt-{interleave}{run_profile}.wasm --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=8192 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --fastreply={fastreply} &> /vv/nlp-count-vectorizer.log &
    """.format(fastreply=fastreply, interleave=interleave, path=path, run_profile=run_profile)

    if not skip_cpu:
        run_command(run_nlp_command_wasmtime, "run_nlp_command_wasmtime", cpu_bench_instance[0].id)

    run_nlp_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    export CUDA_CACHE_MAXSIZE=4294967296
    export CUDA_CACHE_PATH=~/.nv/ComputeCache/
    export PATH=~/.cargo/bin:$PATH
    export PATH=/vv/binaryen-version_109/bin:$PATH

    cd /vv/VectorVisor/benchmarks/

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{prefix}{path}-opt-{interleave}{run_profile}.wasm.bin --ip=0.0.0.0 --heap=3145728 --stack=131072 --hcallsize=8192 --partition=false --rt=0 --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --lgroup={lgroup} --nvidia={nv} &> /vv/nlp-count-vectorizer.log &
    """.format(lgroup=1, prefix=prefix, cflags=CFLAGS, interleave=interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, run_profile=run_profile, path=path, nv=nvflag)

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

    cd /vv/VectorVisor/benchmarks/nlp-count-vectorizer/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 {duration} /vv/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=25, target_rps=vmcount, duration=benchmark_duration)

    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)

        # save output
        with open(temp_dir+"gpu_bench_{path}_{idx}.txt".format(idx=idx, path=path), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)

    if skip_cpu:
        return

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

    cd /vv/VectorVisor/benchmarks/nlp-count-vectorizer/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 {duration} /vv/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=25, target_rps=target_rps_cpu, duration=benchmark_duration)
    
    for idx in range(NUM_REPEAT):
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_bench_{path}_{idx}.txt".format(idx=idx, path=path), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


    cleanup()

    for idx in range(NUM_REPEAT):
        run_command(run_nlp_command_x86, "run_nlp_command_x86", cpu_bench_instance[0].id)
        
        command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

        time.sleep(20)

        # Block until benchmark is complete
        output = block_on_command(command_id, invoker_instance[0].id)
        print (output)
        # save output
        with open(temp_dir+"cpu_x86_bench_nlp_{idx}.txt".format(idx=idx), "w") as text_file:
            text_file.write(str(output))
        time.sleep(SLEEP_TIME)


def run_membench(membench_interleave=4):
    if gpu == "a10g":
        vmcount = 6144
    elif gpu == "t4":
        vmcount = 4096
    elif gpu == "amd":
        vmcount = 2048

    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/examples/mem/memloop.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --nvidia={nv} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag)

    for idx in range(50):
        try:
            command_id = run_command(run_membench_command, "run_membench", gpu_instance[0].id)
            time.sleep(2)
            # Block until benchmark is complete
            output = block_on_command(command_id, gpu_instance[0].id)['StandardOutputContent']
            output = output.replace("\'", "\"")
            print ("Output:")
            print (output)
            output = float(re.search(r'kernel_exec_time:\s(.*?)\n', output).group(1))
            print (output)
            # save output
            with open(temp_dir+"gpu_membench_{interleave}.txt" \
                    .format(interleave=membench_interleave), "a") as text_file:
                text_file.write(str(output) + "\n")
        except Exception:
            continue

    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/examples/mem/memloop_unroll.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --nvidia={nv} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag)

    for idx in range(50):
        try:
            command_id = run_command(run_membench_command, "run_membench_unroll", gpu_instance[0].id)
            time.sleep(2)
            # Block until benchmark is complete
            output = block_on_command(command_id, gpu_instance[0].id)['StandardOutputContent']
            output = output.replace("\'", "\"")
            output = float(re.search(r'kernel_exec_time:\s(.*?)\n', output).group(1))   
            print (output)
            # save output
            with open(temp_dir+"gpu_membench_unroll_{interleave}.txt" \
                    .format(interleave=membench_interleave), "a") as text_file:
                text_file.write(str(output) + "\n")
        except Exception:
            continue


    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/examples/mem/memloop64.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --nvidia={nv} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag)

    for idx in range(50):
        try:
            command_id = run_command(run_membench_command, "run_membench64", gpu_instance[0].id)
            time.sleep(2)
            # Block until benchmark is complete
            output = block_on_command(command_id, gpu_instance[0].id)['StandardOutputContent']
            output = output.replace("\'", "\"")
            output = float(re.search(r'kernel_exec_time:\s(.*?)\n', output).group(1))   
            print (output)
            # save output
            with open(temp_dir+"gpu_membench64_{interleave}.txt" \
                    .format(interleave=membench_interleave), "a") as text_file:
                text_file.write(str(output) + "\n")
        except Exception:
            continue


    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/examples/mem/memloop64_unroll.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --nvidia={nv} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag)

    for idx in range(50):
        try:
            command_id = run_command(run_membench_command, "run_membench64_unroll", gpu_instance[0].id)
            time.sleep(3)
            # Block until benchmark is complete
            output = block_on_command(command_id, gpu_instance[0].id)['StandardOutputContent']
            output = output.replace("\'", "\"")
            output = float(re.search(r'kernel_exec_time:\s(.*?)\n', output).group(1)) 
            print (output)
            # save output
            with open(temp_dir+"gpu_membench64_unroll_{interleave}.txt" \
                    .format(interleave=membench_interleave), "a") as text_file:
                text_file.write(str(output)+"\n")
        except Exception:
            continue


    # Run optimized memcpy
    run_membench_command = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/examples/mem/bulkmemloop.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize=1024 --partition=false --serverless=true --volatile=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --nvidia={nv} &> test.log && tail -n 30 test.log
    """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, nv=nvflag)

    for idx in range(50):
        try:
            command_id = run_command(run_membench_command, "run_membench64_unroll", gpu_instance[0].id)
            time.sleep(3)
            # Block until benchmark is complete
            output = block_on_command(command_id, gpu_instance[0].id)['StandardOutputContent']
            output = output.replace("\'", "\"")
            output = float(re.search(r'kernel_exec_time:\s(.*?)\n', output).group(1)) 
            print (output)
            # save output
            with open(temp_dir+"gpu_bulkmem_{interleave}.txt" \
                    .format(interleave=membench_interleave), "a") as text_file:
                text_file.write(str(output)+"\n")
        except Exception:
            continue



def run_syscall_bench(hcall_sizes, membench_interleave=4):
    if gpu == "a10g":
        vmcount = 6144
    elif gpu == "t4":
        vmcount = 4096
    elif gpu == "amd":
        vmcount = 2048

    for repeat in range(10):
        for hcall_size in hcall_sizes:
            print ("Running bench for hcall_size: ", hcall_size)
            run_syscall_command = """#!/bin/bash
            sudo su
            ulimit -n 65536
            x=$(cloud-init status)
            until [ "$x" == "status: done" ]; do
            sleep 10
            x=$(cloud-init status)
            done

            /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/syscallbench/serverless.wat --ip=0.0.0.0 --heap=3145728 --stack=1024 --hcallsize={hcall} --partition=false --serverless=true --vmcount={vmcount} --interleave={interleave} --pinput={is_pretty} --fastreply={fastreply} --maxdemospace={maxdemo} --lgroup={lgroup} --rt=25 --nvidia={nv} &> /vv/syscall.log
            """.format(lgroup=local_group_size, cflags=CFLAGS, interleave=membench_interleave, is_pretty=is_pretty, fastreply=fastreply, maxdemo=maxdemospace, maxfuncs=maxfuncs, maxloc=maxloc, vmcount=vmcount, hcall=hcall_size+4, nv=nvflag)
            run_command(run_syscall_command, "run_syscall", gpu_instance[0].id)

            # Now run the invoker..
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

            cd /vv/VectorVisor/benchmarks/syscallbench/

            /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/syscallbench/run_syscalls.go {addr} 8000 {target_rps} 1 {duration} {input_size}
            """.format(addr=gpu_instance[0].private_dns_name, input_size=int(hcall_size/1024), target_rps=vmcount, duration=60)

            # save the result...
            command_id = run_command(run_invoker, "run invoker", invoker_instance[0].id)

            time.sleep(10)

            # Block until benchmark is complete
            output = block_on_command(command_id, invoker_instance[0].id)
            print (output)
            # save output
            with open(temp_dir+"gpu_syscallbench_{hcall}_{repeat}.txt".format(hcall=hcall_size, repeat=repeat), "w") as text_file:
                text_file.write(str(output))

            time.sleep(10)

            cleanup()

            time.sleep(10)



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

gpu_ami = ami

if gpu == "a10g":
    gpuinstance = "g5.xlarge"
elif gpu == "t4":
    gpuinstance = "g4dn.xlarge"
elif gpu == "amd":
    gpuinstance = "g4ad.xlarge"
else:
    print ("Err, did not select valid GPU for testing")

Placement={
    'AvailabilityZone': 'us-east-1b',
}

gpu_instance = ec2.create_instances(ImageId=gpu_ami,
                                InstanceType=gpuinstance,
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata_ubuntu,
                                Placement=Placement,
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
    #cpu_ami = 'ami-09a83b91fc98e860f'
    cpu_ami = cpuami
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
                                Placement=Placement,
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
                                Placement=Placement,
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

if skip_membench is None:
    vals = [2**x for x in range(12,19)]
    run_syscall_bench(vals) # 4096 --> 256KiB

    cleanup()

    run_membench(membench_interleave=1)

    cleanup()

    run_membench(membench_interleave=4)

    cleanup()

    run_membench(membench_interleave=8)

    cleanup()

if run_only_membench and skip_membench is None:
    ec2.instances.filter(InstanceIds = instance_id_list).terminate()
    exit()

if gpu != "amd":
    run_genpdf_bench()
    cleanup()

run_image_blur_bench(run_bmp = False)

cleanup()

run_image_blur_bench(run_bmp = True)

cleanup()

# run image hash bench
run_image_hash_bench(run_modified = False)

cleanup()

run_image_hash_bench(run_modified = True)

cleanup()

# run scrypt bench
run_scrypt_bench()

cleanup()

# run lz4 bench
run_lz4_bench()

cleanup()

# run average bench
run_average_bench()

cleanup()

if gpu != "amd":
    run_nlp_count_bench("go")
    cleanup()
    run_nlp_count_bench("assemblyscript")
    cleanup()
    run_nlp_count_bench("rust")
    cleanup()
    # run pbkdf2 bench
    run_pbkdf2_bench()
    cleanup()

# clean up all instances at end
ec2.instances.filter(InstanceIds = instance_id_list).terminate()
