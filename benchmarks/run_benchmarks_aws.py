import boto3
import time


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
     - yum install -y go
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
     - cd average/
     - ~/.cargo/bin/cargo build --release
     - cd ..
     - cd pbkdf2/
     - ~/.cargo/bin/cargo build --release
""" % region



"""
Create VMs for the test
1 GPU VM, 1 CPU VM, and 1 VM for issuing requests
"""
# ImageID = Ubuntu Server 18.04 LTS
# Specific to us-east-2
gpu_instance = ec2.create_instances(ImageId='ami-0414f41139d36fb50',
                                InstanceType="g4dn.xlarge",
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

instance = ec2.create_instances(ImageId='ami-0277b52859bac6f4b',
                                InstanceType="t2.medium",
                                MinCount=1,
                                MaxCount=1,
                                UserData=userdata,
                                IamInstanceProfile={
                                    'Arn': 'arn:aws:iam::573062721377:instance-profile/ec2-ssm',
                                    #'Name': "ec2-ssm"
                                })

print ("Started: " + str(instance) + " with id: " + str(instance[0].id))
print ("Started: " + str(gpu_instance) + " with id: " + str(gpu_instance[0].id))

print ("now waiting...")
instance[0].wait_until_running()
gpu_instance[0].wait_until_running()
print ("Instances are now running")

instance[0].load()
gpu_instance[0].load()

print("CPU instance private addr: ", instance.private_dns_name)
print("GPU instance private addr: ", gpu_instance.private_dns_name)


# Wait until initialization is complete
while True:
    resp = ec2_client.describe_instance_status(InstanceIds=[instance[0].id])
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

run_json_lz4_command = """#!/bin/bash
sudo su

x=$(cloud-init status)
until [ "$x" == "status: done" ]; do
  sleep 10
  x=$(cloud-init status)
done

/tmp/wasm2opencl/target/release/wasm2opencl --input /tmp/wasm2opencl/benchmarks/json-compression-lz4/target/wasm32-wasi/release/json-compression.wasm --heap=3145728 --stack=262144 --hcallsize=131072 --partition=true --serverless=true --vmcount=4096
"""

while True:
    try:
        response = ssm_client.send_command(
                InstanceIds=[gpu_instance[0].id],
                DocumentName="AWS-RunShellScript",
                Parameters={'commands': [run_json_lz4_command, ]}, )
        break
    except:
        print ("Failed to send [run_json_lz4_command] command, retrying...")
        time.sleep(10)

command_id = response['Command']['CommandId']

print ("running SSM command ID to run json-compression-lz4 benchmark: " + str(command_id))

# Now set up the invoker

run_invoker = """#!/bin/bash
sudo su

x=$(cloud-init status)
until [ "$x" == "status: done" ]; do
  sleep 10
  x=$(cloud-init status)
done

go run /tmp/wasm2opencl/benchmarks/json-compression-lz4/run_json_lz4_bench.go {addr} 8000 4096 1 60
""".format(addr=gpu_instance.private_dns_name)

while True:
    try:
        response = ssm_client.send_command(
                InstanceIds=[instance[0].id],
                DocumentName="AWS-RunShellScript",
                Parameters={'commands': [run_invoker, ]}, )
        break
    except:
        print ("Failed to send [invoker json lz4] command, retrying...")
        time.sleep(10)

command_id = response['Command']['CommandId']

print ("running SSM command ID to run the invoker for the json-lz4 benchmark: " + str(command_id))

time.sleep(20)

# Needs to be done for each instance
while True:
    output = ssm_client.get_command_invocation(
          CommandId=command_id,
          InstanceId=str(instance[0].id),
        )
    if output['Status'] == 'InProgress':
        print ("Command is still running...")
        time.sleep(10)
    else:
        print ("Command has completed with status: " + str(output['Status']))
        print (output)
        break

