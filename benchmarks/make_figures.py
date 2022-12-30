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
parser.add_argument("--input", required=True)
args = vars(parser.parse_args())

input_dir = args['input']
print (input_dir)

def parse_file(dir_name, f_name, ret, parse_syscall=False):
    try:
        if parse_syscall:
            temp = ""
        else:
            temp = "_0"
        with open (dir_name+"/{name}{temp}.txt".format(name=f_name, temp=temp), "r") as myfile:
            try:
                temp = dict()
                data = myfile.read().replace("\'", "\"")
                rps = float(re.search(r'Total\sRPS:\s(.*?)\\n', data).group(1))
                on_dev_exe_time = float(re.search(r'On\sdevice\sexecution\stime:\s(.*?)\\n', data).group(1))
                latency = float(re.search(r'Average\srequest\slatency:\s(.*?)\\n', data).group(1))
                queue_submit_time = float(re.search(r'queue\ssubmit\stime:\s(.*?)\\n', data).group(1))
                buffer_time = float(re.search(r'Request\sQueue\sTime:\s(.*?)\\n', data).group(1))
                device_time = float(re.search(r'Device\sTime:\s(.*?)\\n', data).group(1))
                overhead_time = float(re.search(r'overhead:\s(.*?)\\n', data).group(1))
                compile_time = float(re.search(r'compile\stime:\s(.*?)\\n', data).group(1))
                temp['name'] = f_name
                temp['rps'] = rps
                temp['on_dev_exe_time'] = on_dev_exe_time
                temp['latency'] = latency
                temp['queue_submit_time'] = queue_submit_time
                temp['buffer_time'] = buffer_time
                temp['device_time'] = device_time
                temp['overhead'] = overhead_time
                temp['compile_time'] = compile_time
                if parse_syscall:
                    ret[f_name] = device_time
                else:
                    ret[f_name] = temp
            except Exception:
                #print ("{n} was not parsed properly".format(n=f_name))
                temp = dict()
                temp['rps'] = -0.0
                ret[f_name] = temp
    except Exception:
        #print ("{n}.txt does not exist, skipping".format(n=dir_name+"/"+f_name))
        temp = dict()
        temp['rps'] = -0.0
        ret[f_name] = temp

def parse_membench(dir_name, f_name, ret):
    try:
        with open (dir_name+"/{name}.txt".format(name=f_name), "r") as myfile:
            try:
                data = myfile.read().replace("\'", "\"")
                data = np.array(list(map(lambda x: np.double(x), filter(lambda x: x != '', data.split("\n")))))
                ret[f_name] = data
            except Exception:
                print ("{n} was not parsed properly".format(n=f_name))
    except Exception:
        print ("{n}.txt does not exist, skipping".format(n=dir_name+"/"+f_name))

# parse results
def parse_dir(dir_name):
    ret = dict()
    ret['gpu'] = dict() 
    ret['wasm'] = dict()
    ret['x86'] = dict()
    ret['cuda'] = dict()
    ret['membench'] = dict()
    ret['syscalls'] = dict()

    # scrypt 
    parse_file(dir_name, "gpu_bench_scrypt", ret['gpu'])
    parse_file(dir_name, "cpu_bench_scrypt", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_scrypt", ret['x86'])

    # pbkdf2
    parse_file(dir_name, "gpu_bench_pbkdf2", ret['gpu'])
    parse_file(dir_name, "cpu_bench_pbkdf2", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_pbkdf2", ret['x86'])

    # imageblur
    parse_file(dir_name, "gpu_bench_imageblur", ret['gpu'])
    parse_file(dir_name, "cpu_bench_imageblur", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_imageblur", ret['x86'])

    # imageblur-bmp
    parse_file(dir_name, "gpu_bench_imageblur_bmp", ret['gpu'])
    parse_file(dir_name, "cpu_bench_imageblur_bmp", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_imageblur_bmp", ret['x86'])
    parse_file(dir_name, "gpu_cuda_bench_imageblur_bmp", ret['cuda'])

    # phash
    parse_file(dir_name, "gpu_bench_imagehash", ret['gpu'])
    parse_file(dir_name, "cpu_bench_imagehash", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_imagehash", ret['x86'])

    # phash-modified
    parse_file(dir_name, "gpu_bench_imagehash_modified", ret['gpu'])
    parse_file(dir_name, "cpu_bench_imagehash_modified", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_imagehash_modified", ret['x86'])
    parse_file(dir_name, "gpu_cuda_bench_imagehash_bmp", ret['cuda'])

    # genpdf
    parse_file(dir_name, "gpu_bench_genpdf", ret['gpu'])
    parse_file(dir_name, "cpu_bench_genpdf", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_genpdf", ret['x86'])

    # histogram
    parse_file(dir_name, "gpu_bench_average", ret['gpu'])
    parse_file(dir_name, "cpu_bench_average", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_average", ret['x86'])

    # lz4
    parse_file(dir_name, "gpu_bench_lz4", ret['gpu'])
    parse_file(dir_name, "cpu_bench_lz4", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_lz4", ret['x86'])

    # Strings
    parse_file(dir_name, "gpu_bench_nlp-count-vectorizer", ret['gpu'])
    parse_file(dir_name, "cpu_bench_nlp-count-vectorizer", ret['wasm'])
    parse_file(dir_name, "cpu_x86_bench_nlp", ret['x86'])
    parse_file(dir_name, "gpu_bench_nlp-go", ret['gpu'])
    parse_file(dir_name, "cpu_bench_nlp-go", ret['wasm'])
    parse_file(dir_name, "gpu_bench_nlp-assemblyscript", ret['gpu'])
    parse_file(dir_name, "cpu_bench_nlp-assemblyscript", ret['wasm'])

    # membench...
    parse_membench(dir_name, "gpu_bulkmem_1", ret['membench'])
    parse_membench(dir_name, "gpu_bulkmem_4", ret['membench'])
    parse_membench(dir_name, "gpu_bulkmem_8", ret['membench'])

    parse_membench(dir_name, "gpu_membench_1", ret['membench'])
    parse_membench(dir_name, "gpu_membench_4", ret['membench'])
    parse_membench(dir_name, "gpu_membench_8", ret['membench'])

    parse_membench(dir_name, "gpu_membench_unroll_1", ret['membench'])
    parse_membench(dir_name, "gpu_membench_unroll_4", ret['membench'])
    parse_membench(dir_name, "gpu_membench_unroll_8", ret['membench'])

    parse_membench(dir_name, "gpu_membench64_1", ret['membench'])
    parse_membench(dir_name, "gpu_membench64_4", ret['membench'])
    parse_membench(dir_name, "gpu_membench64_8", ret['membench'])

    parse_membench(dir_name, "gpu_membench64_unroll_1", ret['membench'])
    parse_membench(dir_name, "gpu_membench64_unroll_4", ret['membench'])
    parse_membench(dir_name, "gpu_membench64_unroll_8", ret['membench'])

    print (dir_name)
    print (ret['membench'])

    # syscalls...
    for call in [2**x for x in range(12,19)]:
        parse_file(dir_name, "gpu_syscallbench_{size}".format(size=call), ret['syscalls'], parse_syscall=True)

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
    #plt.xticks(x_axis/2, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings', 'Genpdf'))
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

def latency_breakdown(device_exe_time, buffer_time, vmm_overhead, queue_submit, overhead, net_latency, name, scale=30):
    N = len(device_exe_time)
    plt.figure(figsize=(7, 4))
    
    plt.rc('xtick', labelsize=10)
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

    p1 = plt.bar(ind, overhead, width)
    for idx in range(N):
        p1[idx].set_color('blue')

    p2 = plt.bar(ind, device_exe_time, width,
                bottom=np.asarray(overhead))
    for idx in range(N):
        p2[idx].set_color('lightgray')

    p3 = plt.bar(ind, vmm_overhead, width,
                bottom=np.asarray(device_exe_time))
    for idx in range(N):
        p3[idx].set_color('black')

    p4 = plt.bar(ind, combined, width,
                bottom=np.asarray(device_exe_time)+np.asarray(vmm_overhead))
    for idx in range(N):
        p4[idx].set_color('green')

    #bench_names = ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings', 'Genpdf')
    bench_names = ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings')

    print ("Latency breakdown: {x}".format(x=name))
    print ("on-device exe frac")
    for idx in range(N):
        print ("{b}: {val}".format(val=device_exe_time[idx] / total[idx], b=bench_names[idx]))
    print ("vmm frac")
    for idx in range(N):
        print ("{b}: {val}".format(val=vmm_overhead[idx] / total[idx], b=bench_names[idx]))



    #plt.xlabel('Benchmark')
    plt.ylabel('Average Latency (s)')
    if gpu_type == "a10g":
        plt.title('GPU (NVIDIA A10G) Latency Breakdown')
    else:
        plt.title('GPU (NVIDIA T4) Latency Breakdown')

    #plt.xticks(ind, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings', 'Genpdf'))
    plt.xticks(ind, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings'), rotation=50)

    plt.yticks(np.arange(0, 60, 5))
    plt.legend((p4[0], p3[0], p2[0], p1[0]), ('Other', 'VMM Overhead', 'On Device Execution Time', 'Continuations Overhead'))

    #plt.grid()
    plt.grid(zorder=-50)
    plt.gcf().subplots_adjust(bottom=0.25)
    plt.savefig(input_dir+"/{name}_latency_breakdown.eps".format(name=name))
    plt.savefig(input_dir+"/{name}_latency_breakdown.png".format(name=name))
    plt.clf()
def plot_syscalls():
    #plt.figure(figsize=(14, 6))

    ind = np.arange(len(range(12,19))) * 10
    width = 2
    spacing = 2.25
    ind2 = ind + spacing
    ind3 = ind2 + spacing
    ind4 = ind3 + spacing
    ind5 = ind4 + spacing
    ind6 = ind5 + spacing
    ind_ticks = (ind + ind3) / 2
    
    plt.rc('xtick', labelsize=18)
    plt.rc('ytick', labelsize=18)
    plt.rc('axes', titlesize=18)
    plt.rc('axes', labelsize=18)

    t4 = []
    a10g = []
    v520 = []

    def add_interleave(device_str, avg_list, batch=4096):
        for hcall in [2**x for x in range(12,19)]:
            device_time = results[device_str]['syscalls']['gpu_syscallbench_{x}'.format(x=hcall)]
            bandwidth = batch * hcall * 2 / (device_time / 10**3)
            #bandwidth = (rps * hcall * 2) / 10**6
            avg_list.append(bandwidth)


    add_interleave("t4_membench", t4)
    add_interleave("a10g_membench", a10g, batch=6144)
    add_interleave("v520_membench", v520, batch=2048)

    print (t4)
    print (a10g)
    print (v520)

    fig, axes = plt.subplots(nrows=1, ncols=1, figsize=(16, 5))
    #fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(12.75, 5))
    fig.tight_layout()

    plt.xticks(ind2, ["{}".format(int((2**x) / 1024)) for x in range(12,19)])
    #axes[0].set_ylim(0, 650)

    """
    axes[0].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[0].set_yticks(np.arange(0, 800, 50))
    axes[1].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[1].set_yticks(np.arange(0, 800, 50))
    """
    
    #axes[1].yaxis.set_visible(False)
    #plt.subplots_adjust(wspace=0.05)

    plt.ylabel('Bandwidth (MB/s)')
    plt.xlabel('Copy Size (KiB)')
    #axes[0].set_xlabel('Memory Benchmarks')
    #axes[1].set_ylabel('Memory Bandwidth (GB/s)')
    #axes[1].set_xlabel('Memory Benchmarks')
    plt.title('Syscall Bandwidth (Device Transfer Overhead)')

    colors = plt.cm.viridis(np.linspace(0, 1, 12))

    t4 = plt.bar(ind, t4, width, color=colors[0], capsize=6, label='T4')
    a10g = plt.bar(ind2, a10g, width, color=colors[4], capsize=6, label='A10G')
    v520 = plt.bar(ind3, v520, width, color=colors[8], capsize=6, label='v520')

    plt.grid(zorder=-50)
    plt.legend(prop={'size': 18})

    plt.savefig(input_dir+"/syscalls.eps", bbox_inches='tight')

    plt.clf()


def plot_memory_bandwidth():
    #plt.figure(figsize=(14, 6))

    ind = np.arange(5) * 10
    width = 2
    spacing = 2.25
    ind2 = ind + spacing
    ind3 = ind2 + spacing
    ind4 = ind3 + spacing
    ind5 = ind4 + spacing
    ind6 = ind5 + spacing

    ind_ticks = (ind + ind3) / 2
    
    plt.rc('xtick', labelsize=18)
    plt.rc('ytick', labelsize=18)
    plt.rc('axes', titlesize=18)
    plt.rc('axes', labelsize=18)

    nvidia_t4_1 = []
    nvidia_t4_4 = []
    nvidia_t4_8 = []
    nvidia_t4_1_std = []
    nvidia_t4_4_std = []
    nvidia_t4_8_std = []

    nvidia_a10g_1 = []
    nvidia_a10g_4 = []
    nvidia_a10g_8 = []
    nvidia_a10g_1_std = []
    nvidia_a10g_4_std = []
    nvidia_a10g_8_std = []

    v520_1 = []
    v520_4 = []
    v520_8 = []
    v520_1_std = []
    v520_4_std = []
    v520_8_std = []

    def add_interleave(device_str, interleave, avg_list, std_list, batch=4096):
        vals = list(map(lambda x: batch * 1024*1024*2 / x, \
                        results[device_str]['membench']['gpu_membench_{x}'.format(x=interleave)]))
        avg = np.average(vals)
        std = np.std(vals)

        avg_list.append(avg)
        std_list.append(std)

        vals = list(map(lambda x: batch * 1024*1024*2 / x, \
                        results[device_str]['membench']['gpu_membench_unroll_{x}'.format(x=interleave)]))
        avg = np.average(vals)
        std = np.std(vals)

        avg_list.append(avg)
        std_list.append(std)

        vals = list(map(lambda x: batch * 1024*1024*2 / x, \
                        results[device_str]['membench']['gpu_membench64_{x}'.format(x=interleave)]))
        avg = np.average(vals)
        std = np.std(vals)

        avg_list.append(avg)
        std_list.append(std)

        vals = list(map(lambda x: batch * 1024*1024*2 / x, \
                        results[device_str]['membench']['gpu_membench64_unroll_{x}'.format(x=interleave)]))
        avg = np.average(vals)
        std = np.std(vals)

        avg_list.append(avg)
        std_list.append(std)

        vals = list(map(lambda x: batch * 1024*1024*2 / x, \
                        results[device_str]['membench']['gpu_bulkmem_{x}'.format(x=interleave)]))
        avg = np.average(vals)
        std = np.std(vals)
        
        avg_list.append(avg)
        std_list.append(std)


    add_interleave("t4_membench", 1, nvidia_t4_1, nvidia_t4_1_std)
    add_interleave("t4_membench", 4, nvidia_t4_4, nvidia_t4_4_std)
    add_interleave("t4_membench", 8, nvidia_t4_8, nvidia_t4_8_std)

    add_interleave("a10g_membench", 1, nvidia_a10g_1, nvidia_a10g_1_std, batch=6144)
    add_interleave("a10g_membench", 4, nvidia_a10g_4, nvidia_a10g_4_std, batch=6144)
    add_interleave("a10g_membench", 8, nvidia_a10g_8, nvidia_a10g_8_std, batch=6144)

    add_interleave("v520_membench", 1, v520_1, v520_1_std, batch=2048)
    add_interleave("v520_membench", 4, v520_4, v520_4_std, batch=2048)
    add_interleave("v520_membench", 8, v520_8, v520_8_std, batch=2048)

    fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(16, 5))
    #fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(12.75, 5))
    fig.tight_layout()

    axes[0].set_xticks(ind_ticks)
    axes[1].set_xticks(ind_ticks)
    axes[2].set_xticks(ind_ticks)
    axes[0].set_xticklabels(('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll', 'Memory.copy'), rotation=20)
    axes[1].set_xticklabels(('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll', 'Memory.copy'), rotation=20)
    axes[2].set_xticklabels(('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll', 'Memory.copy'), rotation=20)
    
    axes[0].set_ylim(0, 650)
    axes[1].set_ylim(0, 650)
    axes[2].set_ylim(0, 650)

    """
    axes[0].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[0].set_yticks(np.arange(0, 800, 50))
    axes[1].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[1].set_yticks(np.arange(0, 800, 50))
    """
    
    #axes[1].yaxis.set_visible(False)
    #plt.subplots_adjust(wspace=0.05)

    axes[0].set_ylabel('Memory Bandwidth (GB/s)')
    #axes[0].set_xlabel('Memory Benchmarks')
    #axes[1].set_ylabel('Memory Bandwidth (GB/s)')
    #axes[1].set_xlabel('Memory Benchmarks')
    axes[0].set_title('NVIDIA T4')
    axes[1].set_title('NVIDIA A10G')
    axes[2].set_title('AMD v520')

    colors = plt.cm.viridis(np.linspace(0, 1, 12))

    nvidia_t4_1 = axes[0].bar(ind, nvidia_t4_1, width, color=colors[0], yerr=nvidia_t4_1_std, capsize=6)
    nvidia_t4_4 = axes[0].bar(ind2, nvidia_t4_4, width, color=colors[4], yerr=nvidia_t4_4_std, capsize=6)
    nvidia_t4_8 = axes[0].bar(ind3, nvidia_t4_8, width, color=colors[8], yerr=nvidia_t4_8_std, capsize=6)
    nvidia_a10g_1 = axes[1].bar(ind, nvidia_a10g_1, width, color=colors[0], yerr=nvidia_a10g_1_std, capsize=6)
    nvidia_a10g_4 = axes[1].bar(ind2, nvidia_a10g_4, width, color=colors[4], yerr=nvidia_a10g_4_std, capsize=6)
    nvidia_a10g_8 = axes[1].bar(ind3, nvidia_a10g_8, width, color=colors[8], yerr=nvidia_a10g_8_std, capsize=6)
    v520_1 = axes[2].bar(ind, v520_1, width, color=colors[0], yerr=v520_1_std, capsize=6)
    v520_4 = axes[2].bar(ind2, v520_4, width, color=colors[4], yerr=v520_4_std, capsize=6)
    v520_8 = axes[2].bar(ind3, v520_8, width, color=colors[8], yerr=v520_8_std, capsize=6)


    t4_line = axes[0].axhline(y=320, color='b', linestyle='-')
    t4_line_approx = axes[0].axhline(y=220.16, color='black', linestyle='dashed')

    a10g_line = axes[1].axhline(y=600, color='b', linestyle='-')

    v520_line = axes[2].axhline(y=512, color='b', linestyle='-')

    axes[0].grid(zorder=-50)
    axes[1].grid(zorder=-50)
    axes[2].grid(zorder=-50)

    axes[0].legend((nvidia_t4_1[0], nvidia_t4_4[0], nvidia_t4_8[0], t4_line, t4_line_approx),
               ('Interleave = 1 Byte', 'Interleave = 4 Bytes', 'Interleave = 8 Bytes','Theoretical Max Bandwidth', 'Prev. Measured Max Bandwidth'),
               prop={'size': 18})

    """
    axes[1].legend((nvidia_a10g_1[0], nvidia_a10g_4[0], nvidia_a10g_8[0], a10g_line),
               ('Interleave = 1 Byte', 'Interleave = 4 Bytes', 'Interleave = 8 Bytes','Theoretical Max Bandwidth'),
               loc = "upper left", bbox_to_anchor=(0,0.9), prop={'size': 18})

    axes[2].legend((v520_1[0], v520_4[0], v520_8[0], v520_line),
               ('Interleave = 1 Byte', 'Interleave = 4 Bytes', 'Interleave = 8 Bytes','Theoretical Max Bandwidth'),
               prop={'size': 18})
    """

    plt.savefig(input_dir+"/memory_bandwidth.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/memory_bandwidth.png", bbox_inches='tight')

    plt.clf()

def plot_breakdowns():
    #plt.figure(figsize=(14, 6))
    ind = np.arange(len(range(12,19))) * 10
    width = 2
    spacing = 2.25
    ind2 = ind + spacing
    ind3 = ind2 + spacing
    ind4 = ind3 + spacing
    ind5 = ind4 + spacing
    ind6 = ind5 + spacing
    ind_ticks = (ind + ind3) / 2
    
    plt.rc('xtick', labelsize=18)
    plt.rc('ytick', labelsize=18)
    plt.rc('axes', titlesize=18)
    plt.rc('axes', labelsize=18)

    """
    provide breakdowns
    --> continuations overhead
    --> device exe = exe - continuations overhead
    --> Network latency = E2E latency - device time
    --> VMM overhead = Device time - device exe
    """
    t4_cont = []
    t4_exe = []
    t4_net = []
    t4_vmm = []
    a10g_cont = []
    a10g_exe = []
    a10g_net = []
    a10g_vmm = []
    v520_cont = []
    v520_exe = []
    v520_net = []
    v520_vmm = []

    def add_continuations(device_str, overhead_lst, exe_list, net_lst, vmm_lst, batch=4096):
        print (results[device_str]['gpu'].values())
        try:
            overhead = [x['overhead'] for x in results[device_str]['gpu'].values()]
            exe = [x['on_dev_exe_time'] - x['overhead'] for x in results[device_str]['gpu'].values()]
            network = [x['latency'] - x['device_time'] for x in results[device_str]['gpu'].values()]
            vmm = [x['device_time'] - x['on_dev_exe_time'] for x in results[device_str]['gpu'].values()]
            overhead_lst.append(overhead)
            exe_list.append(exe)
            net_lst.append(network)
            vmm_lst.append(vmm)
        except Exception:
            overhead_lst.append(0)
            exe_list.append(0)
            net_lst.append(0)
            vmm_lst.append(0)

    add_continuations("t4_breakdown_4", t4_cont)
    add_continuations("a10g_breakdown_4", a10g_cont, batch=6144)
    add_continuations("v520_breakdown_4", v520_cont, batch=2048)

    print (t4_cont)

    N = len(t4_cont)


    fig, axes = plt.subplots(nrows=1, ncols=1, figsize=(16, 5))
    #fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(12.75, 5))
    fig.tight_layout()

    #plt.xticks(ind2, ["{}".format(int((2**x) / 1024)) for x in range(12,19)])
    #axes[0].set_ylim(0, 650)

    plt.ylabel('Bandwidth (MB/s)')
    plt.xlabel('Copy Size (KiB)')
    plt.title('Syscall Bandwidth (Device Transfer Overhead)')

    colors = plt.cm.viridis(np.linspace(0, 1, 12))  
    """
    t4 = plt.bar(ind, t4, width, color=colors[0], capsize=6, label='T4')
    a10g = plt.bar(ind2, a10g, width, color=colors[4], capsize=6, label='A10G')
    v520 = plt.bar(ind3, v520, width, color=colors[8], capsize=6, label='v520')
    """
    p1 = plt.bar(ind, t4_cont, width)
    for idx in range(N):
        p1[idx].set_color('blue')

    """
    p2 = plt.bar(ind, device_exe_time, width,
                bottom=np.asarray(overhead))
    for idx in range(N):
        p2[idx].set_color('lightgray')

    p3 = plt.bar(ind, vmm_overhead, width,
                bottom=np.asarray(device_exe_time))
    for idx in range(N):
        p3[idx].set_color('black')

    p4 = plt.bar(ind, combined, width,
                bottom=np.asarray(device_exe_time)+np.asarray(vmm_overhead))
    """

    plt.grid(zorder=-50)
    plt.legend(prop={'size': 18})

    plt.savefig(input_dir+"/breakdown.eps", bbox_inches='tight')

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

    # plt.title('GPU (RTX 2080 Ti) Compile Times')

    plt.yticks(ind_ticks, ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings'))
    plt.xticks(np.arange(0, 80, 15))
    plt.ylabel('Benchmark')
    plt.xlabel('Compile Time (min)')
    p1 = plt.barh(ind, compile_times_4, width)
    for idx in range(9):
        p1[idx].set_color('green')

    p2 = plt.barh(ind2, compile_times_1, width)
    for idx in range(9):
        p2[idx].set_color('black')

    plt.legend()
    plt.legend((p2[0], p1[0]), ('Interleave = 1', 'Interleave = 4'), prop={'size': 14})

    plt.grid(zorder=-50)

    plt.savefig(input_dir+"/compile_times.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/compile_times.png", bbox_inches='tight')

    plt.clf()

def plot_batch_times():
    plt.figure(figsize=(8, 4))

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

    # plt.title('Batch Size vs. RPS (NVIDIA RTX 2080 Ti)')
    p1 = plt.plot(batch_sizes, times_scrypt, marker='*', markersize=12)
    p2 = plt.plot(batch_sizes, times_imageblur_bmp, marker='s', markersize=8)
    p3 = plt.plot(batch_sizes, times_phash_modified, marker='D', markersize=8)
    plt.legend((p2[0], p3[0], p1[0]), ('Blur-Bmp', 'PHash-Modified', 'Scrypt'), prop={'size': 14})
    plt.xlabel('Batch Size')
    plt.ylabel('RPS')

    plt.grid(zorder=-50)
    plt.xscale('log')
    plt.xticks(batch_sizes, batch_sizes)

    plt.savefig(input_dir+"/batch.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/batch.png", bbox_inches='tight')

    plt.clf()

def plot_roofline(gpu_bench_rps, gpu_on_dev_exe, gpu_e2e, vmcount, is_gpu):
    plt.figure(figsize=(8, 6)) 
    # [scrypt_gpu, pbkdf2_gpu, imageblur_gpu, imageblur_bmp_gpu, imagehash_gpu, imagehash_modified_gpu, histogram_gpu, lz4_gpu, strings_gpu]
    #bench_names = ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings', 'Genpdf')
    bench_names = ('Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-Modified', 'Histogram', 'LZ4', 'Strings')
    input_sizes = np.array([80 * 256, 32, 94 * 1024, 184 * 1024, 73 * 1024, 184 * 1024, 20 * 1024 * 4, 200 * 1024, 64*1024])
    # Ratio of on_dev_exe to input size vs. RPS?
    print (gpu_e2e)
    if is_gpu:
        #intensity =  (np.array(gpu_on_dev_exe) * 1000) / (input_sizes * vmcount)
        intensity = np.array(gpu_bench_rps) / (input_sizes*vmcount)
    else:
        intensity =  (np.array(gpu_on_dev_exe) * 1000) / (input_sizes)

    #rps = (input_sizes * vmcount) / 1024 / 1024 / gpu_bench_rps
    #rps =  (vmcount*input_sizes)/1024/1024 / (1/(np.array(gpu_bench_rps) / vmcount))
    #rps = (1/(np.array(gpu_bench_rps) / vmcount))
    #rps = (gpu_on_dev_exe / (1/(np.array(gpu_bench_rps) / vmcount)))
    rps = np.array(gpu_bench_rps)
    print (rps)
    print (gpu_bench_rps)
    for idx in range(len(bench_names)):
        #if idx != 1:
        plt.scatter(intensity[idx], rps[idx], label=bench_names[idx])

    #plt.yticks(np.arange(0, 1.0, 0.2))
    #plt.xticks(np.arange(10e-5, 1e2, 1))
    #plt.xticks(np.arange(0, 1, 0.1))
    #plt.yticks(np.arange(0, 1.1, 0.1))

    plt.xscale(value='log')
    plt.legend()
    plt.grid(zorder=-50)
    plt.ylabel('RPS')
    plt.xlabel('Operational Intensity (RPS/byte)')
    if is_gpu:
        dev_name = "gpu"
    else:
        dev_name = "cpu"
    plt.savefig(input_dir+"/roofline_{dev}.eps".format(dev=dev_name), bbox_inches='tight')
    plt.savefig(input_dir+"/roofline_{dev}.png".format(dev=dev_name), bbox_inches='tight')
    plt.clf()

# dump the table...
def dump_row(system, platform, interleave, data, cpu=False):
    if system == "vv":
        temp = "\small \systemname{} & \small "
    elif system == "x86":
        temp = "\small CPU (x86-64) & \small "
    elif system == "wasm":
        temp = "\small CPU (WASM) & \small "
    else:
        # CUDA
        temp = "\small CUDA & \small "
    
    if platform == "t4":
        temp += "NVIDIA T4 & \small {interleave} & ".format(interleave=interleave)
    elif platform == "a10g":
        temp += "NVIDIA A10G & \small {interleave} & ".format(interleave=interleave)
    elif platform == "amd":
        temp += "AMD v520 & \small {interleave} & ".format(interleave=interleave)
    elif platform == "intel":
        temp += "Intel & \small N/A & "
    elif platform == "amdcpu":
        temp += "AMD & \small N/A & "

    if cpu:
        for value in data:
            if value == -0.00:
                temp += r"\small N/A & "
            else:
                temp += r"\small {:0.2f} & ".format(value)
        temp = temp[:-2]
        temp += r" \\"
    else:
        for value in data[:-3]:
            if value == -0.00:
                temp += r"\small N/A & "
            else:
                temp += r"\small {:0.2f} & ".format(value)

        # the last three items are all "Strings" results
        temp += r"\small {:0.2f} / {:0.2f} / {:0.2f} \\".format(data[-3], data[-2], data[-1])
    print (temp)

def dump_table(results, per_dollar=False):
    vals = list(map(lambda x: x['rps'], results['v520_profile_4']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.3785
    dump_row("vv", "amd", 4, vals)

    vals = list(map(lambda x: x['rps'], results['v520_profile_8']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.3785
    dump_row("vv", "amd", 8, vals)

    vals = list(map(lambda x: x['rps'], results['t4_4']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("vv", "t4", 4, vals)

    vals = list(map(lambda x: x['rps'], results['t4_8']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("vv", "t4", 8, vals)

    vals = list(map(lambda x: x['rps'], results['t4_profile_4']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("vv", "t4", 4, vals)

    vals = list(map(lambda x: x['rps'], results['t4_profile_8']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("vv", "t4", 8, vals)

    vals = list(map(lambda x: x['rps'], results['a10g_4']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.006
    dump_row("vv", "a10g", 4, vals)

    vals = list(map(lambda x: x['rps'], results['a10g_8']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.006
    dump_row("vv", "a10g", 8, vals)

    vals = list(map(lambda x: x['rps'], results['a10g_profile_4']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.006
    dump_row("vv", "a10g", 4, vals)

    vals = list(map(lambda x: x['rps'], results['a10g_profile_8']['gpu'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.006
    dump_row("vv", "a10g", 8, vals)

    # Dump CPU data
    vals = list(map(lambda x: x['rps'], results['t4_4']['x86'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.154
    dump_row("x86", "amdcpu", 4, vals, cpu=True)

    vals = list(map(lambda x: x['rps'], results['t4_8']['x86'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.17
    dump_row("x86", "intel", 4, vals, cpu=True)

    vals = list(map(lambda x: x['rps'], results['t4_4']['wasm'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.154
    dump_row("wasm", "amdcpu", 4, vals)

    vals = list(map(lambda x: x['rps'], results['t4_8']['wasm'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.17
    dump_row("wasm", "intel", 4, vals)


    vals = list(map(lambda x: x['rps'], results['t4_cuda']['cuda'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.526
    print ("t4 cuda", np.array(vals))
    """
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("cuda", "t4", 4, vals)
    """

    vals = list(map(lambda x: x['rps'], results['t4_cuda_2x']['cuda'].values()))
    if per_dollar:
        vals = np.array(vals) / 0.752
    print ("t4 cuda 2x", np.array(vals))

    vals = list(map(lambda x: x['rps'], results['a10g_cuda']['cuda'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.006
    print ("a10g cuda", np.array(vals))
    """
    if per_dollar:
        vals = np.array(vals) / 0.526
    dump_row("cuda", "t4", 4, vals)
    """

    vals = list(map(lambda x: x['rps'], results['a10g_cuda_2x']['cuda'].values()))
    if per_dollar:
        vals = np.array(vals) / 1.212
    print ("a10g cuda 2x", np.array(vals))


results = dict()
results['t4_cuda'] = parse_dir(input_dir+"t4_cuda")
results['t4_cuda_2x'] = parse_dir(input_dir+"t4_cuda_2x")
results['a10g_cuda'] = parse_dir(input_dir+"a10g_cuda")
results['a10g_cuda_2x'] = parse_dir(input_dir+"a10g_cuda_2x")
results['t4_4'] = parse_dir(input_dir+"t4_amd_4")
results['t4_profile_4'] = parse_dir(input_dir+"t4_amd_4_profile")
results['t4_breakdown_4'] = parse_dir(input_dir+"t4_amd_4_breakdown")
results['t4_8'] = parse_dir(input_dir+"t4_amd_8")
results['t4_profile_8'] = parse_dir(input_dir+"t4_amd_8_profile")
results['t4_breakdown_8'] = parse_dir(input_dir+"t4_amd_8_breakdown")
results['t4_membench'] = parse_dir(input_dir+"t4_membench")
results['a10g_4'] = parse_dir(input_dir+"a10g_intel_4")
results['a10g_profile_4'] = parse_dir(input_dir+"a10g_intel_4_profile")
results['a10g_breakdown_4'] = parse_dir(input_dir+"a10g_intel_4_breakdown")
results['a10g_8'] = parse_dir(input_dir+"a10g_intel_8")
results['a10g_profile_8'] = parse_dir(input_dir+"a10g_intel_8_profile")
results['a10g_breakdown_8'] = parse_dir(input_dir+"a10g_intel_8_breakdown")
results['a10g_membench'] = parse_dir(input_dir+"a10g_membench")
results['v520_profile_4'] = parse_dir(input_dir+"v520_4_profile")
results['v520_profile_8'] = parse_dir(input_dir+"v520_8_profile")
results['v520_breakdown_4'] = parse_dir(input_dir+"v520_4_breakdown")
results['v520_breakdown_8'] = parse_dir(input_dir+"v520_8_breakdown")
results['v520_membench'] = parse_dir(input_dir+"amd_membench")

# dump throughput table
print ("\nThroughput\n\n\n")
dump_table(results)
print ("\nThroughput/$\n\n\n")
dump_table(results, per_dollar=True)

# mem bandwidth figures
plot_memory_bandwidth()

# plot syscall perf
plot_syscalls()

# plot breakdowns
plot_breakdowns()