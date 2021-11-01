import argparse
import re
import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import numpy as np

parser = argparse.ArgumentParser(description='Process some integers.')
parser.add_argument("--input_dir", required=True)
args = vars(parser.parse_args())

input_dir = args['input_dir']
print (input_dir)

def parse_file(f_name):
    ret = dict()
    with open (input_dir+"/{name}.txt".format(name=f_name), "r") as myfile:
        try:
            data = myfile.read().replace("\'", "\"")
            rps = float(re.search(r'Total\sRPS:\s(.*?)\\n', data).group(1))
            on_dev_exe_time = float(re.search(r'On\sdevice\sexecution\stime:\s(.*?)\\n', data).group(1))
            latency = float(re.search(r'Average\srequest\slatency:\s(.*?)\\n', data).group(1))
            queue_submit_time = float(re.search(r'queue\ssubmit\stime:\s(.*?)\\n', data).group(1))
            buffer_time = float(re.search(r'Request\sQueue\sTime:\s(.*?)\\n', data).group(1))
            device_time = float(re.search(r'Device\sTime:\s(.*?)\\n', data).group(1))
            ret['rps'] = rps
            ret['on_dev_exe_time'] = on_dev_exe_time
            ret['latency'] = latency
            ret['queue_submit_time'] = queue_submit_time
            ret['buffer_time'] = buffer_time
            ret['device_time'] = device_time
        except Exception:
            print ("{n} was not parsed properly".format(n=f_name))
    return ret

def plot_bars(gpu_latency, cpu_wasm_latency, cpu_x86_latency, figname):
    plt.figure(figsize=(16, 4))
    perf_gpu = np.asarray(gpu_latency)
    perf_cpu_wasm = np.asarray(cpu_wasm_latency)
    perf_cpu_x86 = np.asarray(cpu_x86_latency)

    perf_gpu_wasm = perf_gpu / perf_cpu_wasm
    perf_gpu_x86 = perf_gpu / perf_cpu_x86

    N = len(gpu_latency)
    ind1 = np.arange(N) * 2
    ind2 = np.arange(N) * 2 + 0.5

    x_axis = np.arange(N) * 4 + 0.5
    x_axis.sort(kind='mergesort')

    width = 0.35      # the width of the bars: can also be len(x) sequence

    p1 = plt.bar(ind1, perf_gpu_wasm, width)
    p2 = plt.bar(ind2, perf_gpu_x86, width)
    plt.yscale(value='log')

    plt.ylabel('Log Norm Throughput')
    plt.title('GPU (g4dn.xlarge) vs. CPU Application Throughput (c5.xlarge)')
    plt.xticks(x_axis/2, ('Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4'))
    ax = plt.gca()
    ax.set_yticks([1, 1.5, 2, 3, 5, 10, 20, 25, 50])
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.FuncFormatter(lambda x, p: "{num}x".format(num=x)))

    # Perf/$ improvement
    for idx in range(len(perf_gpu_wasm)):
      label_x = idx*2
      label_y = perf_gpu_wasm[idx]
      ax.annotate('{val:0.2f}'.format(val=(perf_gpu_wasm[idx]) / (0.526 / 0.17)), xy=(label_x, label_y), xytext=(label_x, label_y+3),ha='center', weight='bold')

    for idx in range(len(perf_gpu_x86)):
      label_x = idx*2 + 0.5
      label_y = perf_gpu_x86[idx]
      ax.annotate('{val:0.2f}'.format(val=(perf_gpu_x86[idx]) / (0.526 / 0.17)), xy=(label_x, label_y), xytext=(label_x, label_y+3),ha='center', weight='bold')

    # breakeven point for performance per dollar
    # ((x)/y) / (0.526 / 0.17) > 1
    l1 = plt.axhline(y=3.09412, color='r', linestyle='-')

    ax.legend([l1, p1, p2], ['Throughput/$ Breakeven Threshold', 'WebAssembly', 'x86-64'])
    plt.grid()
    plt.savefig(input_dir+"/{name}.png".format(name=figname))
    plt.clf()

def latency_breakdown(device_exe_time, buffer_time, vmm_overhead, queue_submit, net_latency):
    N = 7

    total = []
    for idx in range(N):
        total.append(device_exe_time[idx]+vmm_overhead[idx]+queue_submit[idx]+buffer_time[idx]+net_latency[idx])

    ind = np.arange(N)    # the x locations for the groups
    width = 0.35       # the width of the bars: can also be len(x) sequence

    p1 = plt.bar(ind, device_exe_time, width)

    p2 = plt.bar(ind, queue_submit, width,
                bottom=device_exe_time)

    p3 = plt.bar(ind, vmm_overhead, width,
                bottom=np.asarray(device_exe_time)+np.asarray(queue_submit))

    p4 = plt.bar(ind, buffer_time, width,
                bottom=np.asarray(device_exe_time)+np.asarray(queue_submit)+np.asarray(vmm_overhead))

    p5 = plt.bar(ind, net_latency, width,
                bottom=np.asarray(device_exe_time)+np.asarray(queue_submit)+np.asarray(vmm_overhead)+np.asarray(buffer_time))     

    plt.xlabel('Benchmark')
    plt.ylabel('Average Latency (s)')
    plt.title('GPU (g4dn.xlarge) Latency Breakdown')
    plt.xticks(ind, ('Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4'))
    plt.yticks(np.arange(0, 30, 2))
    plt.legend((p1[0], p2[0], p3[0], p4[0], p5[0]), ('On Device Execution Time', 'Device Queueing Overhead', 'VMM Overhead', 'Buffer Time', 'Network'))

    plt.grid()
    plt.savefig("latency_breakdown.png")
    plt.clf()

def latency_throughput(gpu_latency, gpu_throughput, cpu_x86_latency, cpu_x86_throughput, cpu_wasm_latency, cpu_wasm_throughput):
    #plt.figure(figsize=(6, 4))
    benchmarks = [0, 2, 4]

    fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(12, 4))
    fig.suptitle('GPU (g4dn.xlarge) vs. CPU Application Latency (c5.xlarge)')
    for ax in axes.flat:
        ax.set_xlabel('Requests Per Second')
        ax.set_ylabel('Latency (s)')

    axes[0].set_title('Pbkdf2')
    axes[1].set_title('Blur-Bmp')
    axes[2].set_title('PHash-Modified')

    for bench, bench_idx in zip(benchmarks, range(len(benchmarks))):
        ax = axes[bench_idx]
        ax.scatter(gpu_throughput[bench], gpu_latency[bench], label='GPU')
        ax.scatter(cpu_x86_throughput[bench], cpu_x86_latency[bench], label='CPU x86-64')
        ax.scatter(cpu_wasm_throughput[bench], cpu_wasm_latency[bench], label='CPU WASM')
        ax.set_ylim(0, 60)
        ax.set_xlim(0, 1300)
        ax.grid(True)
    plt.legend()
    plt.savefig("latency_throughput.png")
    plt.clf()

# pbkdf2
pbkdf2_gpu = parse_file("gpu_bench_pbkdf2")
pbkdf2_cpu_wasm = parse_file("cpu_bench_pbkdf2")
pbkdf2_cpu_x86 = parse_file("cpu_x86_bench_pbkdf2")

# imageblur
imageblur_gpu = parse_file("gpu_bench_imageblur")
imageblur_cpu_wasm = parse_file("cpu_bench_imageblur")
imageblur_cpu_x86 = parse_file("cpu_x86_bench_imageblur")

# imageblur-bmp
imageblur_bmp_gpu = parse_file("gpu_bench_imageblur_bmp")
imageblur_bmp_cpu_wasm = parse_file("cpu_bench_imageblur_bmp")
imageblur_bmp_cpu_x86 = parse_file("cpu_x86_bench_imageblur_bmp")

# phash
imagehash_gpu = parse_file("gpu_bench_imagehash")
imagehash_cpu_wasm = parse_file("cpu_bench_imagehash")
imagehash_cpu_x86 = parse_file("cpu_x86_bench_imagehash")

# phash-modified
imagehash_modified_gpu = parse_file("gpu_bench_imagehash_modified")
imagehash_modified_cpu_wasm = parse_file("cpu_bench_imagehash_modified")
imagehash_modified_cpu_x86 = parse_file("cpu_x86_bench_imagehash_modified")

# histogram
histogram_gpu = parse_file("gpu_bench_average")
histogram_cpu_wasm = parse_file("cpu_bench_average")
histogram_cpu_x86 = parse_file("cpu_x86_bench_average")

# lz4
lz4_gpu = parse_file("gpu_bench_lz4")
lz4_cpu_wasm = parse_file("cpu_bench_lz4")
lz4_cpu_x86 = parse_file("cpu_x86_bench_lz4")

vmcount = [4096, 3072, 3072, 3072, 3072, 4096, 3072]
gpu_list = [pbkdf2_gpu, imageblur_gpu, imageblur_bmp_gpu, imagehash_gpu, imagehash_modified_gpu, histogram_gpu, lz4_gpu]
cpu_wasm_list = [pbkdf2_cpu_wasm, imageblur_cpu_wasm, imageblur_bmp_cpu_wasm, imagehash_cpu_wasm, imagehash_modified_cpu_wasm, histogram_cpu_wasm, lz4_cpu_wasm]
cpu_x86_list = [pbkdf2_cpu_x86, imageblur_cpu_x86, imageblur_bmp_cpu_x86, imagehash_cpu_x86, imagehash_modified_cpu_x86, histogram_cpu_x86, lz4_cpu_x86]
gpu_rps = []
cpu_wasm_rps = []
cpu_x86_rps = []
for d in gpu_list:
    gpu_rps.append(d['rps'])
for d in cpu_wasm_list:
    cpu_wasm_rps.append(d['rps'])
for d in cpu_x86_list:
    cpu_x86_rps.append(d['rps'])

# First, plot E2E performance 
plot_bars(gpu_rps, cpu_wasm_rps, cpu_x86_rps, "e2e_rps")

# Next, remove network time, measure only on device time
# We compute the RPS from only on device execution time
gpu_rps_device = []
cpu_wasm_rps_device = []
cpu_x86_rps_device = []
for d, v in zip(gpu_list, vmcount):
    # include buffer time in GPU measurement but not CPU
    new_rps = v / (d['device_time'] / (10 ** 9))
    gpu_rps_device.append(new_rps)
# Each CPU instance has 4 cores, so can process 4 requests per second
for d, v in zip(cpu_wasm_list, vmcount):
    new_rps = 4 / ((d['device_time'] - d['buffer_time']) / (10 ** 9))
    cpu_wasm_rps_device.append(new_rps)
for d, v in zip(cpu_x86_list, vmcount):
    new_rps = 4 / ((d['device_time'] - d['buffer_time']) / (10 ** 9))
    cpu_x86_rps_device.append(new_rps)

plot_bars(gpu_rps_device, cpu_wasm_rps_device, cpu_x86_rps_device, "e2e_device_time_only")


# plot latency breakdown
gpu_device_exe = []
gpu_buffer_time = []
gpu_qsubmit = []
gpu_vmm_overhead = []
gpu_net_latency = []
for d, v in zip(gpu_list, vmcount):
    gpu_device_exe.append(d['on_dev_exe_time'] / (10 ** 9))
    gpu_buffer_time.append(d['buffer_time'] / (10 ** 9))
    gpu_qsubmit.append(d['queue_submit_time'] / (10 ** 9))
    # vmm overhead = device_time - queue_submit_time - buffer_time - exe_time
    gpu_vmm_overhead.append((d['device_time'] - d['queue_submit_time'] - d['buffer_time'] - d['on_dev_exe_time']) / (10 ** 9))
    gpu_net_latency.append((d['latency'] - d['device_time']) / (10 ** 9))

latency_breakdown(gpu_device_exe, gpu_buffer_time, gpu_vmm_overhead, gpu_qsubmit, gpu_net_latency)

gpu_latency = []
cpu_x86_latency = []
cpu_wasm_latency = []
for d, v in zip(gpu_list, vmcount):
    gpu_latency.append(d['latency'] / (10 ** 9))
for d, v in zip(cpu_wasm_list, vmcount):
    cpu_wasm_latency.append(d['latency'] / (10 ** 9))
for d, v in zip(cpu_x86_list, vmcount):
    cpu_x86_latency.append(d['latency'] / (10 ** 9))


# plot latency/throughput
latency_throughput(gpu_latency, gpu_rps, cpu_x86_latency, cpu_x86_rps, cpu_wasm_latency, cpu_wasm_rps)