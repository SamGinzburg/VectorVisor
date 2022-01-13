import argparse
import re
import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import numpy as np

plt.rc('axes', axisbelow=True)
plt.grid(c='lightgrey')


sysname = "VectorVisor"

parser = argparse.ArgumentParser(description='generate graphs')
parser.add_argument("--input_dir", required=True)
parser.add_argument("--gpu", required=True)
parser.add_argument("--cpu", required=True)

args = vars(parser.parse_args())

input_dir = args['input_dir']
print (input_dir)

gpu_type = args['gpu']
print (gpu_type)

cpu_type = args['cpu']
print (cpu_type)

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
    plt.figure(figsize=(7, 3))
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
    plt.xticks(x_axis/2, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings'))
    ax = plt.gca()
    ax.set_yticks([1, 1.5, 2, 3, 5, 10, 20, 25, 50])
    ax.get_yaxis().set_major_formatter(matplotlib.ticker.FuncFormatter(lambda x, p: "{num}x".format(num=x)))

    # Perf/$ improvement

    if gpu_type == "a10g":
        gpu_price = 1.006
    else:
        gpu_price = 0.526

    if cpu_type == "intel":
        cpu_price = 0.17
    else:
        cpu_price = 0.154

    for idx in range(len(perf_gpu_wasm)):
      label_x = idx*2
      label_y = perf_gpu_wasm[idx]
      ax.annotate('{val:0.2f}'.format(val=(perf_gpu_wasm[idx]) / (gpu_price / cpu_price)), xy=(label_x, label_y), xytext=(label_x, label_y+3),ha='center', weight='bold')

    for idx in range(len(perf_gpu_x86)):
        label_x = idx*2 + 0.5
        label_y = perf_gpu_x86[idx]
        ax.annotate('{val:0.2f}'.format(val=(perf_gpu_x86[idx]) / (gpu_price / cpu_price)), xy=(label_x, label_y), xytext=(label_x, label_y+3),ha='center', weight='bold')

    # breakeven point for performance per dollar
    # ((x)/y) / (0.526 / 0.17) > 1
    # T4 -> 3.09412
    # A10G -> 6.53
    if gpu_type == "a10g":
        l1 = plt.axhline(y=6.53, color='r', linestyle='-')
    else:
        l1 = plt.axhline(y=3.09, color='r', linestyle='-')

    ax.legend([l1, p1, p2], ['Throughput/$ Breakeven Threshold', '{sys} vs. WebAssembly'.format(sys=sysname), '{sys} vs. x86-64'.format(sys=sysname)])
    plt.grid()
    plt.savefig(input_dir+"/{name}.png".format(name=figname))
    plt.clf()

def latency_breakdown(device_exe_time, buffer_time, vmm_overhead, queue_submit, net_latency, name, scale=30):
    N = 9
    plt.figure(figsize=(14, 6))
    
    plt.rc('xtick', labelsize=12)
    plt.rc('ytick', labelsize=12)
    plt.rc('axes', titlesize=16)
    plt.rc('axes', labelsize=16)
    
    total = []
    for idx in range(N):
        total.append(device_exe_time[idx]+vmm_overhead[idx]+queue_submit[idx]+buffer_time[idx]+net_latency[idx])

    ind = np.arange(N)    # the x locations for the groups
    width = 0.35       # the width of the bars: can also be len(x) sequence
    from operator import add
    combined = list( map(add, queue_submit, buffer_time) )
    combined = list( map(add, combined, net_latency) )

    p1 = plt.bar(ind, device_exe_time, width)
    for idx in range(N):
        p1[idx].set_color('lightgray')

    p2 = plt.bar(ind, vmm_overhead, width,
                bottom=np.asarray(device_exe_time))
    for idx in range(N):
        p2[idx].set_color('black')

    p3 = plt.bar(ind, combined, width,
                bottom=np.asarray(device_exe_time)+np.asarray(vmm_overhead))
    for idx in range(N):
        p3[idx].set_color('green')

    bench_names = ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings')
    print ("Latency breakdown: {x}".format(x=name))
    print ("on-device exe frac")
    for idx in range(N):
        print ("{b}: {val}".format(val=device_exe_time[idx] / total[idx], b=bench_names[idx]))
    print ("vmm frac")
    for idx in range(N):
        print ("{b}: {val}".format(val=vmm_overhead[idx] / total[idx], b=bench_names[idx]))



    plt.xlabel('Benchmark')
    plt.ylabel('Average Latency (s)')
    if gpu_type == "a10g":
        plt.title('GPU (NVIDIA A10G) Latency Breakdown')
    else:
        plt.title('GPU (NVIDIA T4) Latency Breakdown')

    plt.xticks(ind, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings'))
    plt.yticks(np.arange(0, 36, 5))
    plt.legend((p3[0], p2[0], p1[0]), ('Other', 'VMM Overhead', 'On Device Execution Time'))

    #plt.grid()
    plt.grid(zorder=-50)
    plt.savefig(input_dir+"/{name}_latency_breakdown.eps".format(name=name))
    plt.savefig(input_dir+"/{name}_latency_breakdown.png".format(name=name))
    plt.clf()

def latency_throughput(gpu_latency, gpu_throughput, cpu_x86_latency, cpu_x86_throughput, cpu_wasm_latency, cpu_wasm_throughput):
    #plt.figure(figsize=(6, 4))
    benchmarks = [0, 2, 4]

    fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(7, 3))
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
    plt.savefig(input_dir+"/latency_throughput.eps")
    plt.clf()

def plot_compile_times():
    #plt.figure(figsize=(14, 6))

    ind = np.arange(9)    # the x locations for the groups
    width = 0.175
    ind2 = ind + 0.4
    ind_ticks = (ind + ind2) / 2
    print (ind)
    

    plt.rc('xtick', labelsize=16)
    plt.rc('ytick', labelsize=16)
    plt.rc('axes', titlesize=20)
    plt.rc('axes', labelsize=20)

    compile_times_1 = [12.18, 16.05, 25.65, 20.62, 32.2, 61.85, 17.02, 13.25, 18.15]
    compile_times_4 = [7.65, 12.77, 16.3, 28.78, 10, 22.23, 8.38, 11.02, 9.97]

    plt.title('GPU (RTX 2080 Ti) Compile Times')

    plt.yticks(ind_ticks, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings'))
    plt.xticks(np.arange(0, 80, 15))
    plt.ylabel('Benchmark')
    plt.xlabel('Compile Time (min)')
    p1 = plt.barh(ind, compile_times_1, width)
    for idx in range(9):
        p1[idx].set_color('green')

    p2 = plt.barh(ind2, compile_times_4, width)
    for idx in range(9):
        p2[idx].set_color('black')

    plt.legend()
    plt.legend((p2[0], p1[0]), ('Interleave = 4', 'Interleave = 1'))

    plt.grid(zorder=-50)

    plt.savefig(input_dir+"/compile_times.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/compile_times.png", bbox_inches='tight')

    plt.clf()

def plot_batch_times():
    #plt.figure(figsize=(14, 6))

    ind = np.arange(9)    # the x locations for the groups
    width = 0.175
    ind2 = ind + 0.4
    ind_ticks = (ind + ind2) / 2
    print (ind)
    

    plt.rc('xtick', labelsize=16)
    plt.rc('ytick', labelsize=16)
    plt.rc('axes', titlesize=20)
    plt.rc('axes', labelsize=20)

    times_scrypt = [39.04, 75.23, 88, 129.113, 149.65, 150.18]
    times_imageblur_bmp = [182.910000, 355.826667, 397.866667, 437.120000, 512.133333, 600.423333]
    times_phash_modified = [118.286667, 223.453333, 272.266667, 312.053333, 329.066667, 443.133333]

    batch_sizes = [64, 128, 256, 512, 1024, 2048]

    plt.title('Batch Size vs. RPS (NVIDIA RTX 2080 Ti)')
    p1 = plt.plot(batch_sizes, times_scrypt, marker='*', markersize=12)
    p2 = plt.plot(batch_sizes, times_imageblur_bmp, marker='s', markersize=8)
    p3 = plt.plot(batch_sizes, times_phash_modified, marker='D', markersize=8)
    plt.legend((p2[0], p3[0], p1[0]), ('Blur-Bmp', 'PHash-Modified', 'Scrypt'))
    plt.xlabel('Batch Size')
    plt.ylabel('RPS')

    plt.grid(zorder=-50)
    plt.xscale('log')
    plt.xticks(batch_sizes, batch_sizes)

    plt.savefig(input_dir+"/batch.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/batch.png", bbox_inches='tight')

    plt.clf()

# scrypt
scrypt_gpu = parse_file("gpu_bench_scrypt")
scrypt_cpu_wasm = parse_file("cpu_bench_scrypt")
scrypt_cpu_x86 = parse_file("cpu_x86_bench_scrypt")

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

# Strings
strings_gpu = parse_file("gpu_bench_nlp")
strings_cpu_wasm = parse_file("cpu_bench_nlp")
strings_cpu_x86 = parse_file("cpu_x86_bench_nlp")

vmcount = [4096, 4096, 3072, 3072, 3072, 3072, 4096, 3072, 3072]
gpu_list = [scrypt_gpu, pbkdf2_gpu, imageblur_gpu, imageblur_bmp_gpu, imagehash_gpu, imagehash_modified_gpu, histogram_gpu, lz4_gpu, strings_gpu]
cpu_wasm_list = [scrypt_cpu_wasm, pbkdf2_cpu_wasm, imageblur_cpu_wasm, imageblur_bmp_cpu_wasm, imagehash_cpu_wasm, imagehash_modified_cpu_wasm, histogram_cpu_wasm, lz4_cpu_wasm, strings_cpu_wasm]
cpu_x86_list = [scrypt_cpu_x86, pbkdf2_cpu_x86, imageblur_cpu_x86, imageblur_bmp_cpu_x86, imagehash_cpu_x86, imagehash_modified_cpu_x86, histogram_cpu_x86, lz4_cpu_x86, strings_cpu_x86]
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
# Each CPU instance has 4 cores, so can process 4 requests concurrently
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

latency_breakdown(gpu_device_exe, gpu_buffer_time, gpu_vmm_overhead, gpu_qsubmit, gpu_net_latency, "gpu", scale=60)

cpu_device_exe = []
cpu_buffer_time = []
cpu_qsubmit = []
cpu_vmm_overhead = []
cpu_net_latency = []
for d, v in zip(cpu_x86_list, vmcount):
    cpu_device_exe.append(d['on_dev_exe_time'] / (10 ** 9))
    #cpu_buffer_time.append(d['buffer_time'] / (10 ** 9))
    cpu_buffer_time.append(0)
    cpu_qsubmit.append(d['queue_submit_time'] / (10 ** 9))
    # vmm overhead = device_time - queue_submit_time - buffer_time - exe_time
    cpu_vmm_overhead.append((d['device_time'] - d['queue_submit_time'] - d['buffer_time'] - d['on_dev_exe_time']) / (10 ** 9))
    cpu_net_latency.append((d['latency'] - d['device_time']) / (10 ** 9))


latency_breakdown(cpu_device_exe, cpu_buffer_time, cpu_vmm_overhead, cpu_qsubmit, cpu_net_latency, "cpu_x86", scale=1)

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

plot_compile_times()

plot_batch_times()