import boto3
import time


ec2 = boto3.resource('ec2')

region = "us-east-2"
userdata = """#cloud-config
    runcmd:
     - /home/ec2-user/sudo npm run prod
     - cd /tmp
     - curl https://amazon-ssm-%s.s3.amazonaws.com/latest/linux_amd64/amazon-ssm-agent.rpm -o amazon-ssm-agent.rpm
     - yum install -y amazon-ssm-agent.rpm
""" % region   



"""
Create VMs for the test
1 GPU VM, 1 CPU VM, and 1 VM for issuing requests
"""
# ImageID = Ubuntu Server 18.04 LTS
# Specific to us-east-2
#instance = ec2.create_instances(ImageId='ami-0b9064170e32bde34', InstanceType="g4dn.xlarge", MinCount=1, MaxCount=1, UserData=userdata)
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


print ("now waiting...")
instance[0].wait_until_running()
print ("Instance is now running")

ssm_client = boto3.client('ssm')

response = ssm_client.send_command(
            InstanceIds=[instance[0].id],
            DocumentName="AWS-RunShellScript",
            Parameters={'commands': ['ls -lah']}, )

command_id = response['Command']['CommandId']

print ("SSM command ID: " + str(command_id))

# Needs to be done for each instance
output = ssm_client.get_command_invocation(
      CommandId=command_id,
      InstanceId=str(instance[0].id),
    )

print (output)
