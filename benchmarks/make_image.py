import boto3
import time
import os
from datetime import date, datetime
import re
import argparse

parser = argparse.ArgumentParser(description='run benchmarks')
parser.add_argument("--gpu", required=True)
parser.add_argument("--awsarn", required=True)

args = vars(parser.parse_args())

gpu = args['gpu']

awsarn = args['awsarn']

print ("gpu: ", gpu)
print ("running with AWS ARN: ", awsarn)


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
TIMEOUT_MINUTES = 60 * 24
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

if run_a10g:
    maxdemospace = 0
    local_group_size = 16
else:
    maxdemospace = 0
    local_group_size = 64

region = "us-east-1"
ec2 = boto3.resource('ec2', region_name=region)
ec2_client = boto3.client('ec2', region_name=region)

userdata_ubuntu = """#cloud-config
    runcmd:
     - whoami
     - sudo su
     - sudo whoami
     - export HOME=/root
     - export CUDA_CACHE_MAXSIZE=4294967296
     - export CUDA_CACHE_PATH=~/.nv/ComputeCache/
     - sysctl -w net.ipv4.tcp_max_syn_backlog=65536
     - sysctl -w net.core.somaxconn=8192
     - mkdir -p /vv/
     - cd /vv/
     - sudo apt update
     - sudo apt install -y git
     - sudo apt install -y git-lfs
     - sudo apt install -y htop
     - sudo apt install -y gcc
     - sudo apt install -y curl
     - sudo apt install -y clinfo
     - sudo apt install -y nvidia-driver-525-server
     - sudo apt install -y opencl-dev
     - wget https://golang.org/dl/go1.17.1.linux-amd64.tar.gz
     - rm -rf /usr/local/go && tar -C /usr/local -xzf go1.17.1.linux-amd64.tar.gz
     - sudo curl https://sh.rustup.rs -sSf | sh -s -- -y
     - sudo ~/.cargo/bin/rustup default 1.65-x86_64-unknown-linux-gnu
     - . $HOME/.cargo/env
     - sudo ~/.cargo/bin/rustup target add wasm32-wasi
     - git clone https://github.com/SamGinzburg/VectorVisor
     - wget https://github.com/WebAssembly/binaryen/releases/download/version_109/binaryen-version_109-x86_64-linux.tar.gz
     - tar -xzvf binaryen-version_109-x86_64-linux.tar.gz
     - cargo install --git https://github.com/SamGinzburg/wasm-snip.git
     - cd /vv/VectorVisor/
     - sudo ~/.cargo/bin/cargo build --release
     - cd benchmarks/
     - mkdir -p ~/.nv/ComputeCache/
     - export PATH=/vv/binaryen-version_109/bin:$PATH
     - sudo ~/.cargo/bin/cargo install --git https://github.com/SamGinzburg/vv-pgo-instrument
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

def run_profile_generic(bench_name, params="", testdir=""):
    print ("Running: ", bench_name)
    if testdir=="":
        testdir = bench_name

    run_command_wasmtime = """#!/bin/bash
    sudo su
    ulimit -n 65536
    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done
    
    export PATH=/vv/binaryen-version_109/bin:$PATH
    export PATH=~/.cargo/bin:$PATH

    /vv/VectorVisor/target/release/vectorvisor --input /vv/VectorVisor/benchmarks/{name}-opt-instrument.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=1310720 --partition=true --serverless=true --vmcount=4096 --wasmtime=true --profile=true &
    """.format(interleave=interleave, name=bench_name)

    run_command(run_command_wasmtime, "rustpdfwriter_cpu", gpu_instance[0].id)

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
    export PATH=/vv/binaryen-version_109/bin:$PATH
    export PATH=~/.cargo/bin:$PATH

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/{name}/

    /usr/local/go/bin/go run /vv/VectorVisor/benchmarks/{name}/run_*.go {addr} 8000 {target_rps} 1 {duration} {params}
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=256, duration=300, name=testdir, params=params)
    command_id = run_command(run_invoker, "run invoker for gpu", gpu_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    run_invoker = """#!/bin/bash
    sudo su
    ulimit -n 65536
    mkdir -p ~/gocache/
    mkdir -p ~/gopath/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export GOPATH=~/gopath/
    export XDG_CACHE_HOME=~/xdg/
    export PATH=/vv/binaryen-version_109/bin:$PATH
    export PATH=~/.cargo/bin:$PATH

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    cd /vv/VectorVisor/benchmarks/
    vv-profiler --input /vv/VectorVisor/benchmarks/{name}-opt-4.wasm --output /vv/VectorVisor/benchmarks/{name}-opt-profile.wasm --profile=/vv/VectorVisor/benchmarks/{name}-opt-instrument.wasm.profile
    wasm-opt -all -O3 -g -c /vv/VectorVisor/benchmarks/{name}-opt-profile.wasm -o /vv/VectorVisor/benchmarks/{name}-opt-profile.wasm
    cp /vv/VectorVisor/benchmarks/{name}-opt-profile.wasm /vv/VectorVisor/benchmarks/{name}-opt-4-profile.wasm
    cp /vv/VectorVisor/benchmarks/{name}-opt-profile.wasm /vv/VectorVisor/benchmarks/{name}-opt-8-profile.wasm
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=vmcount, duration=60, hashes=256, name=bench_name)
    command_id = run_command(run_invoker, "run invoker for gpu", gpu_instance[0].id)

    time.sleep(20)
    output = block_on_command(command_id, gpu_instance[0].id)
    print (output)

    # Block until benchmark is complete

    time.sleep(SLEEP_TIME)

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
    #gpu_ami = 'ami-02e8976fea9b1f568'
    gpu_ami = 'ami-0b13950be17f7d1b5'
elif region == "us-east-2":
    gpu_ami = 'ami-01463836f7041cd10'

if run_a10g:
    gpuinstance = "g5.2xlarge"
    #gpuinstance = "g4dn.2xlarge"
else:
    gpuinstance = "g4dn.2xlarge"


gpu_instance = ec2.create_instances(ImageId=gpu_ami,
                                InstanceType=gpuinstance,
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata_ubuntu,
                                IamInstanceProfile={
                                    'Arn': awsarn,
                                })


print ("Started: " + str(gpu_instance) + " with id: " + str(gpu_instance[0].id))

instance_id_list = [gpu_instance[0].id]
print ("Instance id list: ", instance_id_list)

print ("now waiting...")
gpu_instance[0].wait_until_running()
print ("Instances are now running")

gpu_instance[0].load()

print("GPU instance private addr: ", gpu_instance[0].private_dns_name)

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

block_until_done = """#!/bin/bash
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
./{gpu}_save_cached_bin.sh
""".format(gpu=gpu)

command_id = run_command(block_until_done, "precompile GPU binaries", gpu_instance[0].id)
time.sleep(20)

# Block until benchmark is complete
output = block_on_command(command_id, gpu_instance[0].id)
print (output)

time.sleep(120)

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
run_profile_generic("json-compression", params="/vv/VectorVisor/benchmarks/json-compression/smaller_tweets.txt 2000")
run_profile_generic("scrypt", params="256")
run_profile_generic("pbkdf2")
run_profile_generic("nlp-count-vectorizer", params="/vv/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt 500")
run_profile_generic("nlp-assemblyscript", params="/vv/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt 500")
# pass in custom test dir, go projects can't include multiple  
run_profile_generic("nlp-go", testdir="nlp-count-vectorizer", params="/vv/VectorVisor/benchmarks/nlp-count-vectorizer/smaller_tweets.txt 500")

block_until_done = """#!/bin/bash
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
./{gpu}_compile_opt.sh
""".format(gpu=gpu)

command_id = run_command(block_until_done, "precompile GPU binaries", gpu_instance[0].id)
time.sleep(20)

# Block until benchmark is complete
output = block_on_command(command_id, gpu_instance[0].id)
print (output)

time.sleep(120)

# now build the AMI
image = ec2_client.create_image(InstanceId=gpu_instance[0].id, NoReboot=True, Name="vectorvisor-bench-image-{gpu}".format(gpu=gpu))
print ("Finished image creation!")
print (image)

time.sleep(120)

# clean up all instances at end
ec2.instances.filter(InstanceIds = instance_id_list).terminate()
