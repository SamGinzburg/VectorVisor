import boto3
import time

# Benchmark constants

target_rps = 5000

ec2 = boto3.resource('ec2')
ec2_client = boto3.client('ec2')

region = "us-east-2"
userdata = """#cloud-config
    runcmd:
     - /home/ec2-user/sudo npm run prod
     - cd /tmp
     - curl https://amazon-ssm-%s.s3.amazonaws.com/latest/linux_amd64/amazon-ssm-agent.rpm -o amazon-ssm-agent.rpm
     - yum install -y amazon-ssm-agent.rpm
     - yum install -y git
     - yum install -y gcc
     - yum install -y golang
     - yum install -y curl
     - yum install -y https://dl.fedoraproject.org/pub/epel/epel-release-latest-7.noarch.rpm
     - yum update -y
     - yum install -y ocl*
     - curl https://sh.rustup.rs -sSf | sh -s -- -y
     - ~/.cargo/bin/rustup target add wasm32-wasi
     - git clone https://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:x-oauth-basic@github.com/SamGinzburg/wasm2opencl.git
     - cd /tmp/wasm2opencl/
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
""" % region


def run_command(command, command_name, instance_id):
    while True:
        try:
            response = ssm_client.send_command(
                    InstanceIds=[instance_id],
                    DocumentName="AWS-RunShellScript",
                    Parameters={'commands': [command, ]}, )
            break
        except:
            print ("Failed to send [run_json_lz4_command] command, retrying...")
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

def run_pbkdf2_bench(run_x86):
    # Now we can set up the next benchmark (pbkdf2)
    if run_x86:
        run_pbkdf2_command_wasmtime = """#!/bin/bash
        sudo su

        x=$(cloud-init status)
        until [ "$x" == "status: done" ]; do
        sleep 10
        x=$(cloud-init status)
        done

        cd /tmp/wasm2opencl/benchmarks/pbkdf2/
        ~/.cargo/bin/cargo run --release --target x86_64-unknown-linux-gnu &> /tmp/pbkdf2.log &
        """
    else:
        run_pbkdf2_command_wasmtime = """#!/bin/bash
        sudo su

        x=$(cloud-init status)
        until [ "$x" == "status: done" ]; do
        sleep 10
        x=$(cloud-init status)
        done

        /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/pbkdf2/target/wasm32-wasi/release/pbkdf2.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --wasmtime=true &> /tmp/pbkdf2.log &
        """

    run_command(run_pbkdf2_command_wasmtime, "pbkdf2_cpu", cpu_bench_instance[0].id)

    run_pbkdf2_command = """#!/bin/bash
    sudo su

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/pbkdf2/target/wasm32-wasi/release/pbkdf2.wasm --ip=0.0.0.0 --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096 --vmgroups=1 --maxdup=2 &> /tmp/pbkdf2.log &
    """

    run_command(run_pbkdf2_command, "pbkdf2_gpu", gpu_instance[0].id)

    # now run the invoker(s) for pbkdf2
    run_invoker = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 120

    go run /tmp/wasm2opencl/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 120
    """.format(addr=gpu_instance[0].private_dns_name, target_rps=target_rps)

    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open("gpu_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_cpu = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 120

    go run /tmp/wasm2opencl/benchmarks/pbkdf2/run_pbkdf2.go {addr} 8000 {target_rps} 1 120
    """.format(addr=cpu_bench_instance[0].private_dns_name, target_rps=target_rps)

    command_id = run_command(run_invoker_cpu, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open("cpu_bench_pbkdf2.txt", "w") as text_file:
        text_file.write(str(output))

def run_lz4_bench():
    run_json_lz4_command_wasmtime = """#!/bin/bash
    sudo su

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/json-compression/target/wasm32-wasi/release/json-compression.wasm --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=524288 --partition=true --serverless=true --vmcount=3072 --wasmtime=true &> /tmp/json-compression.log &
    """

    run_command(run_json_lz4_command_wasmtime, "run_json_lz4_command_wasmtime", cpu_bench_instance[0].id)

    run_json_lz4_command = """#!/bin/bash
    sudo su

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/json-compression/target/wasm32-wasi/release/json-compression.wasm --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=524288 --partition=true --serverless=true --vmcount=3072 --vmgroups=1 --maxdup=2 &> /tmp/json-compression.log &
    """

    run_command(run_json_lz4_command, "run_json_lz4_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/json-compression/run_json_lz4.go {addr} 8000 {target_rps} 1 60 {input_size}

    go run /tmp/wasm2opencl/benchmarks/json-compression/run_json_lz4.go {addr} 8000 {target_rps} 1 60 {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=350, target_rps=target_rps)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open("gpu_bench_lz4.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/json-compression/run_json_lz4.go {addr} 8000 {target_rps} 1 60 {input_size}

    go run /tmp/wasm2opencl/benchmarks/json-compression/run_json_lz4.go {addr} 8000 {target_rps} 1 60 {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=350, target_rps=target_rps)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open("cpu_bench_lz4.txt", "w") as text_file:
        text_file.write(str(output))

def run_nlp_count_bench():
    run_nlp_command_wasmtime = """#!/bin/bash
    sudo su

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/target/wasm32-wasi/release/nlp-count-vectorizer.wasm --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=524288 --partition=true --serverless=true --vmcount=3072 --wasmtime=true &> /tmp/nlp-count-vectorizer.log &
    """

    run_command(run_nlp_command_wasmtime, "run_nlp_command_wasmtime", cpu_bench_instance[0].id)

    run_nlp_command = """#!/bin/bash
    sudo su

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    /tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/target/wasm32-wasi/release/nlp-count-vectorizer.wasm --ip=0.0.0.0 --heap=4194304 --stack=262144 --hcallsize=524288 --partition=true --serverless=true --vmcount=3072 --vmgroups=1 --maxdup=2 --disablefastcalls=true &> /tmp/nlp-count-vectorizer.log &
    """

    run_command(run_nlp_command, "run_nlp_command", gpu_instance[0].id)

    # Now set up the invoker

    run_invoker = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 60 /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}

    go run /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 60 /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=gpu_instance[0].private_dns_name, input_size=1000, target_rps=target_rps)


    command_id = run_command(run_invoker, "run invoker for gpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)

    # save output
    with open("gpu_bench_nlp.txt", "w") as text_file:
        text_file.write(str(output))

    run_invoker_wasmtime = """#!/bin/bash
    sudo su

    mkdir -p ~/gocache/
    mkdir -p ~/xdg/
    export GOCACHE=~/gocache/
    export XDG_CACHE_HOME=~/xdg/

    go env

    x=$(cloud-init status)
    until [ "$x" == "status: done" ]; do
    sleep 10
    x=$(cloud-init status)
    done

    go run /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 60 /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}

    go run /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/run_nlp.go {addr} 8000 {target_rps} 1 60 /tmp/wasm2opencl/benchmarks/nlp-count-vectorizer/smaller_tweets.txt {input_size}
    """.format(addr=cpu_bench_instance[0].private_dns_name, input_size=1000, target_rps=target_rps)

    command_id = run_command(run_invoker_wasmtime, "run invoker for cpu", invoker_instance[0].id)

    time.sleep(20)

    # Block until benchmark is complete
    output = block_on_command(command_id, invoker_instance[0].id)
    print (output)
    # save output
    with open("cpu_bench_nlp.txt", "w") as text_file:
        text_file.write(str(output))

# call between benchmarks
def cleanup():
    terminate_gpu = """#!/bin/bash
    sudo su
    curl -X GET {addr}:8000/terminate
    curl -X GET {addr_cpu}:8000/terminate
    """.format(addr=gpu_instance[0].private_dns_name, addr_cpu=cpu_bench_instance[0].private_dns_name)
    command_id = run_command(terminate_gpu, "run invoker for gpu", invoker_instance[0].id)
    time.sleep(2)
    output = block_on_command(command_id, invoker_instance[0].id)
    time.sleep(2)

"""
Create VMs for the test
1 GPU VM, 1 CPU VM, and 1 VM for issuing requests

g4dn.xlarge  => 1 T4, 16 GiB memory,  4 vCPU
g4dn.2xlarge => 1 T4, 32 GiB memory, 8 vCPU
g4dn.4xlarge => 1 T4, 64 GiB memory, 16 vCPU
p3.2xlarge   => 1 V100, 16 GiB memory, 8 vCPU

"""
# AMIs specific to us-east-2
gpu_instance = ec2.create_instances(ImageId='ami-0414f41139d36fb50',
                                InstanceType="g4dn.2xlarge", # $0.53 / hr
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

# cpu wasmtime instance
cpu_bench_instance = ec2.create_instances(ImageId='ami-0277b52859bac6f4b',
                                InstanceType="c5.xlarge", # $0.17 / hr
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

invoker_instance = ec2.create_instances(ImageId='ami-0277b52859bac6f4b',
                                InstanceType="t2.2xlarge",
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata,
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

ssm_client = boto3.client('ssm')

# run pbkdf2 bench
#run_pbkdf2_bench(True)

#cleanup()

# run lz4 bench
#run_lz4_bench()

#cleanup()

run_nlp_count_bench()

# clean up all instances at end
ec2.instances.filter(InstanceIds = instance_id_list).terminate()
