import argparse
import json
import re
import numpy as np
import matplotlib
import matplotlib.pyplot as plt
from numpy import concatenate, sort

parser = argparse.ArgumentParser(description='Process some integers.')
parser.add_argument("--input_dir", required=True)
args = vars(parser.parse_args())

input_dir = args['input_dir']
print (input_dir)

def parse_file(f_name):
    ret = dict()
    with open (input_dir+"/{name}.txt".format(name=f_name), "r") as myfile:
        data = myfile.read().replace("\'", "\"")
        rps = float(re.search(r'Total\sRPS:\s(.*?)\\n', data).group(1))
        on_dev_exe_time = float(re.search(r'On\sdevice\sexecution\stime:\s(.*?)\\n', data).group(1))
        latency = float(re.search(r'Average\srequest\slatency:\s(.*?)\\n', data).group(1))
        queue_submit_time = float(re.search(r'queue\ssubmit\stime:\s(.*?)\\n', data).group(1))
        buffer_time = float(re.search(r'Request\sQueue\sTime:\s(.*?)\\n', data).group(1))
        ret['rps'] = rps
        ret['on_dev_exe_time'] = on_dev_exe_time
        ret['latency'] = latency
        ret['queue_submit_time'] = queue_submit_time
        ret['buffer_time'] = buffer_time
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
cpu_x86_list = [pbkdf2_cpu_x86, imageblur_cpu_x86, imageblur_cpu_x86, imagehash_cpu_x86, imagehash_modified_cpu_x86, histogram_cpu_x86, lz4_cpu_x86]
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
    new_rps = v / (d['on_dev_exe_time'] / (10 ** 9))
    gpu_rps_device.append(new_rps)
for d, v in zip(cpu_wasm_list, vmcount):
    new_rps = v / (d['on_dev_exe_time'] / (10 ** 9))
    cpu_wasm_rps_device.append(new_rps)
for d, v in zip(cpu_x86_list, vmcount):
    new_rps = v / (d['on_dev_exe_time'] / (10 ** 9))
    cpu_x86_rps_device.append(new_rps)

plot_bars(gpu_rps_device, cpu_wasm_rps_device, cpu_x86_rps_device, "e2e_device_time_only")
