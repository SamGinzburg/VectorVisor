# this is a helper script to invoke a batch of requests
import requests
import json
from time import time

test = {
    "name": "John",
    "age": 43,
    "phones": [1, 2, 3, 4, 5]
}

sample_req = {
    "req_id": 0,
    "req": "{\"name\": \"TEST\", \"age\": 43,\"phones\": [\"+44 1234567\", \"+44 2345678\"]}"
}

sample_req = {
    "req_id": 0,
    "req": json.dumps(test),
}

req_list = []
for x in range(256):
    req_list.append(sample_req)

t0 = time()
r = requests.get('http://localhost:8000/batch_submit/',
    json={"requests": req_list})
t1 = time()

print (r.status_code)
print (r.json())

print ('Request took: %f seconds' %(t1-t0))