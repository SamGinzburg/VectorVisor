# this is a helper script to invoke a batch of requests
import requests
import json
from time import time
from multiprocessing import Pool
import sys
import random
import string
import numpy as np

def send_request_batch(req_list_ip_tuple):
    req_list = req_list_ip_tuple[0]
    port = req_list_ip_tuple[1]
    num_batches_to_run = req_list_ip_tuple[2]
    # do 100 batches
    e2e_times = []
    all_device_times = []
    average_on_device_times = []
    average_queue_times = []
    average_queue_submit_count = []
    average_num_unique_fns_called = []
    for x in range(num_batches_to_run):
        t0 = time()
        r = requests.get('http://localhost:{}/batch_submit/'.format(port), json={"requests": req_list})
        t1 = time()
        on_device_times = []
        device_queue_times = []
        queue_submit_count = []
        num_unique_fns_called = []
        for resp in r.json()['requests']:
            #print (r.json()['requests'][resp])
            on_device_times.append(r.json()['requests'][resp]['on_device_execution_time_ns'])
            device_queue_times.append(r.json()['requests'][resp]['device_queue_overhead_time_ns'])
            queue_submit_count.append(r.json()['requests'][resp]['queue_submit_count'])
            num_unique_fns_called.append(r.json()['requests'][resp]['num_unique_fns_called'])


        e2e_times.append(t1-t0)
        average_on_device_times.append(sum(on_device_times) / len(on_device_times))
        average_queue_times.append(sum(device_queue_times) / len(device_queue_times))
        average_queue_submit_count.append(sum(queue_submit_count) / len(queue_submit_count))
        average_num_unique_fns_called.append(sum(num_unique_fns_called) / len(num_unique_fns_called))
        all_device_times.extend(on_device_times)
    return (e2e_times, average_on_device_times, average_queue_times, average_queue_submit_count, average_num_unique_fns_called, all_device_times)

if __name__ == '__main__':
    # batch size

    if len(sys.argv) >= 1:
        try:
            BATCH_SIZE=int(sys.argv[1])
        except Exception as e:
            BATCH_SIZE=64
    else:
        BATCH_SIZE=64

    if len(sys.argv) >= 2:
        PORT=int(sys.argv[2])
    else:
        PORT=8000

    if len(sys.argv) >= 3:
        NUM_VMM=int(sys.argv[3])
    else:
        NUM_VMM=1

    if len(sys.argv) >= 4:
        NUM_BATCHES_TO_RUN=int(sys.argv[4])
    else:
        NUM_BATCHES_TO_RUN=10

    req_submit_list = []
    # for each VMM we are targeting, create a batch
    for vmm_idx in range(NUM_VMM):
        # create a batch
        req_list = []
        for x in range(BATCH_SIZE):
            # generate random string
            random_str = ''.join(random.choice(string.ascii_uppercase + string.digits) for _ in range(random.randint(4096*2,4096*2)))

            # what we are sending the serverless function
            payload = {
                "text": random_str,
            }

            # some formatting stuff for the runtime
            sample_req = {
                "req_id": 0,
                "req": json.dumps(payload),
            }
            req_list.append(sample_req)
        req_submit_list.append((req_list, PORT + vmm_idx, NUM_BATCHES_TO_RUN))


    # we can use this to ping multiple VMMs in parallel
    p = Pool(5)
    times = p.map(send_request_batch, req_submit_list)


    print ("Batch size for each req: ", BATCH_SIZE)
    print ("Total number of batches: ", NUM_BATCHES_TO_RUN)
    total_rps = 0
    max_time = 0
    for vmm, idx in zip(times, range(len(times))):
        e2e_times = vmm[0]
        on_device_time = vmm[1]
        queue = vmm[2]
        average_queue_submit_count = vmm[3]
        num_unique_fns_called = vmm[4]
        total_on_dev_times = vmm[5]
        total_rps +=  (BATCH_SIZE * NUM_BATCHES_TO_RUN) / sum(e2e_times)
        max_time = max(max_time, sum(e2e_times))
        print ("VMM: ", idx)
        print ("Total E2E (s): ", sum(e2e_times))
        print ("Requests Per Second: ", (BATCH_SIZE * NUM_BATCHES_TO_RUN) / sum(e2e_times))
        print ("Average E2E (s): ", sum(e2e_times) / len(e2e_times))
        print ("Average on device time (ns): ", sum(on_device_time) / len(on_device_time))
        print ("Stddev of device times (ns): ", np.std(total_on_dev_times))
        print ("Average queue submit overhead (ns): ", sum(queue) / len(queue))
        print ("Average # queue submits: ", sum(average_queue_submit_count) / len(average_queue_submit_count))
        print ("Average # num_unique_fns_called: ", sum(num_unique_fns_called) / len(num_unique_fns_called))
    print ("Total sum RPS: ", (BATCH_SIZE * NUM_BATCHES_TO_RUN * NUM_VMM) / max_time)
