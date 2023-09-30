import argparse
from distutils.dep_util import newer_pairwise
import re
import numpy as np
import matplotlib
import matplotlib.pyplot as plt
import numpy as np
from mpl_toolkits.axes_grid1.inset_locator import zoomed_inset_axes, inset_axes
from mpl_toolkits.axes_grid1.inset_locator import mark_inset

plt.rc('axes', axisbelow=True)
plt.grid(c='lightgrey')

# Get rid of type 3 fonts
matplotlib.rcParams['pdf.fonttype'] = 42
matplotlib.rcParams['ps.fonttype'] = 42

sysname = "VectorVisor"

parser = argparse.ArgumentParser(description='generate graphs')
parser.add_argument("--input", required=True)
args = vars(parser.parse_args())

input_dir = args['input']
print (input_dir)

def parse_syscalls(dir_name, f_name, ret, parse_syscall=False):
    try:
        dev_time = []
        for idx in range(10):
            print (dir_name+"/{name}_{idx}.txt".format(name=f_name, idx=idx))
            with open (dir_name+"/{name}_{idx}.txt".format(name=f_name, idx=idx), "r") as myfile:
                data = myfile.read().replace("\'", "\"")
                rps = float(re.search(r'Total\sRPS:\s(.*?)\\n', data).group(1))
                on_dev_exe_time = float(re.search(r'On\sdevice\sexecution\stime:\s(.*?)\\n', data).group(1))
                latency = float(re.search(r'Average\srequest\slatency:\s(.*?)\\n', data).group(1))
                queue_submit_time = float(re.search(r'queue\ssubmit\stime:\s(.*?)\\n', data).group(1))
                buffer_time = float(re.search(r'Request\sQueue\sTime:\s(.*?)\\n', data).group(1))
                device_time = float(re.search(r'Device\sTime:\s(.*?)\\n', data).group(1))
                dev_time.append(device_time)
                overhead_time = float(re.search(r'overhead:\s(.*?)\\n', data).group(1))
                compile_time = float(re.search(r'compile\stime:\s(.*?)\\n', data).group(1))
        dev_time = np.array(dev_time)
        ret[f_name] = dict()
        ret[f_name]['device_time'] = dev_time
    except Exception as e:
        print (e)
        temp = dict()
        temp['rps'] = -0.0
        temp['on_dev_exe_time'] = 0
        temp['latency'] = 0
        temp['queue_submit_time'] = 0
        temp['buffer_time'] = 0
        temp['device_time'] = 0
        temp['overhead'] = 0
        temp['compile_time'] = 0
        ret[f_name] = temp

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
                temp['on_dev_exe_time'] = 0
                temp['latency'] = 0
                temp['queue_submit_time'] = 0
                temp['buffer_time'] = 0
                temp['device_time'] = 0
                temp['overhead'] = 0
                temp['compile_time'] = 0
                ret[f_name] = temp
    except Exception:
        #print ("{n}.txt does not exist, skipping".format(n=dir_name+"/"+f_name))
        temp = dict()
        temp['rps'] = -0.0
        temp['on_dev_exe_time'] = 0
        temp['latency'] = 0
        temp['queue_submit_time'] = 0
        temp['buffer_time'] = 0
        temp['device_time'] = 0
        temp['overhead'] = 0
        temp['compile_time'] = 0
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
        parse_syscalls(dir_name, "gpu_syscallbench_{size}".format(size=call), ret['syscalls'], parse_syscall=True)

    return ret

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

    plt.rc('xtick', labelsize=30)
    plt.rc('ytick', labelsize=36)
    plt.rc('axes', titlesize=36)
    plt.rc('axes', labelsize=36)

    t4 = []
    a10g = []
    v520 = []

    t4_std = []
    a10g_std = []
    v520_std = []

    def add_interleave(device_str, avg_list, std_list, batch=4096):
        for hcall in [2**x for x in range(12,19)]:
            device_time = results[device_str]['syscalls']['gpu_syscallbench_{x}'.format(x=hcall)]['device_time']
            bw = []
            for value in device_time:
                bandwidth = batch * hcall * 2 / (value / 10**3)
                bw.append(bandwidth)
            bw = np.array(bw)
            avg_list.append(np.average(bw))
            std_list.append(np.std(bw))

    add_interleave("t4_membench", t4, t4_std)
    add_interleave("a10g_membench", a10g, a10g_std, batch=6144)
    add_interleave("v520_membench", v520, v520_std, batch=2048)

    print (t4, t4_std)
    print (a10g, a10g_std)
    print (v520, v520_std)

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

    t4 = plt.bar(ind, t4, width, yerr=t4_std, color=colors[0], hatch='o.', capsize=6, label='T4')
    a10g = plt.bar(ind2, a10g, width, yerr=a10g_std, color=colors[4], hatch='/\\', capsize=6, label='A10G')
    v520 = plt.bar(ind3, v520, width, yerr=v520_std, color=colors[8], hatch='o', capsize=6, label='v520')

    plt.grid(zorder=-50, axis='y')
    plt.legend(prop={'size': 36})

    plt.savefig(input_dir+"/syscalls.eps", bbox_inches='tight')

    plt.clf()


def plot_memory_bandwidth():
    ind = np.arange(5) * 10
    width = 2
    spacing = 2.25
    ind2 = ind + spacing
    ind3 = ind2 + spacing
    ind4 = ind3 + spacing
    ind5 = ind4 + spacing
    ind6 = ind5 + spacing

    ind_ticks = (ind + ind3) / 2
    
    plt.rc('xtick', labelsize=30)
    plt.rc('ytick', labelsize=36)
    plt.rc('axes', titlesize=36)
    plt.rc('axes', labelsize=36)

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

    print ("a10g:", nvidia_a10g_8)

    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(16, 5))
    plt.subplots_adjust(wspace=0.025)

    #fig, axes = plt.subplots(nrows=1, ncols=3, figsize=(12.75, 5))
    fig.tight_layout()


    axes[0].set_yticks([0, 200, 400, 600])
    axes[0].set_xticks(ind_ticks)
    axes[1].set_xticks(ind_ticks)
    #axes[2].set_xticks(ind_ticks)
    axes[0].set_xticklabels(('4x1', '4x4', '8x1', '8x4', '128+ILP'), rotation=20)
    axes[1].set_xticklabels(('4x1', '4x4', '8x1', '8x4', '128+ILP'), rotation=20)
    #axes[2].set_xticklabels(('4x1', '4x4', '8x1', '8x4', '128+ILP'), rotation=20)
    
    axes[0].set_ylim(0, 650)
    axes[1].set_ylim(0, 650)
    #axes[2].set_ylim(0, 650)

    """
    axes[0].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[0].set_yticks(np.arange(0, 800, 50))
    axes[1].set_xticklabels(ind_ticks, ('Membench', 'Membench-Unroll', 'Membench64', 'Membench64-Unroll'))
    axes[1].set_yticks(np.arange(0, 800, 50))
    """
    
    #axes[1].yaxis.set_visible(False)
    #plt.subplots_adjust(wspace=0.05)
    #plt.xlabel('Memcpy Bytes x Unroll Count')
    axes[0].set_ylabel('Memory B.w. (GB/s)')
    #axes[0].set_xlabel('Memory Benchmarks')
    #axes[1].set_ylabel('Memory Bandwidth (GB/s)')
    #axes[1].set_xlabel('Memory Benchmarks')
    axes[0].set_title('NVIDIA T4')
    axes[1].set_title('NVIDIA A10G')
    #axes[1].set_yticks([])
    axes[1].get_yaxis().set_ticklabels([])
    plt.subplots_adjust(wspace=0.025)

    #axes[2].set_title('AMD v520')

    colors = plt.cm.viridis(np.linspace(0, 1, 12))

    nvidia_t4_1 = axes[0].bar(ind, nvidia_t4_1, width, color=colors[0], hatch='o.', yerr=nvidia_t4_1_std, capsize=6)
    nvidia_t4_4 = axes[0].bar(ind2, nvidia_t4_4, width, color=colors[4], hatch='/\\', yerr=nvidia_t4_4_std, capsize=6)
    nvidia_t4_8 = axes[0].bar(ind3, nvidia_t4_8, width, color=colors[8], hatch='o', yerr=nvidia_t4_8_std, capsize=6)
    nvidia_a10g_1 = axes[1].bar(ind, nvidia_a10g_1, width, color=colors[0], hatch='o.', yerr=nvidia_a10g_1_std, capsize=6)
    nvidia_a10g_4 = axes[1].bar(ind2, nvidia_a10g_4, width, color=colors[4], hatch='/\\', yerr=nvidia_a10g_4_std, capsize=6)
    nvidia_a10g_8 = axes[1].bar(ind3, nvidia_a10g_8, width, color=colors[8], hatch='o', yerr=nvidia_a10g_8_std, capsize=6)
    """
    v520_1 = axes[2].bar(ind, v520_1, width, color=colors[0], hatch='o.', yerr=v520_1_std, capsize=6)
    v520_4 = axes[2].bar(ind2, v520_4, width, color=colors[4], hatch='/\\', yerr=v520_4_std, capsize=6)
    v520_8 = axes[2].bar(ind3, v520_8, width, color=colors[8], hatch='o', yerr=v520_8_std, capsize=6)
    """

    t4_line = axes[0].axhline(y=320, color='b', linestyle='-')
    t4_line_approx = axes[0].axhline(y=220.16, color='black', linestyle='dashed')

    a10g_line = axes[1].axhline(y=600, color='b', linestyle='-')

    #v520_line = axes[2].axhline(y=512, color='b', linestyle='-')

    axes[0].grid(zorder=-50, axis='y')
    axes[1].grid(zorder=-50, axis='y')
    #axes[2].grid(zorder=-50)

    axes[0].legend((nvidia_t4_1[0], nvidia_t4_4[0], nvidia_t4_8[0], t4_line, t4_line_approx),
               ('Interleave = 1 Byte', 'Interleave = 4 Bytes', 'Interleave = 8 Bytes', 'Theoretical Max B.w.', 'Prev. Measured Max B.w.'),
               prop={'size': 32}, bbox_to_anchor=(0, -0.2, 2.025, 2), loc='upper center',
               ncol=2, mode="expand", borderaxespad=0.)

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

def plot_breakdowns(results):
#    plt.figure(figsize=(16, 6))
    ind = np.arange(12) * 16
    # cluster the groups of three together
    # 12 clusters...
    print (ind)
    width = 10
    spacing = 1.25
    ind2 = ind + spacing
    ind3 = ind2 + spacing
    ind4 = ind3 + spacing
    ind5 = ind4 + spacing
    ind6 = ind5 + spacing
    ind_ticks = ind
    
    plt.rc('xtick', labelsize=30)
    plt.rc('ytick', labelsize=36)
    plt.rc('axes', titlesize=36)
    plt.rc('axes', labelsize=36)

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
    t4_breakdown_cont = []
    t4_breakdown_exe = []
    t4_breakdown_net = []
    t4_breakdown_vmm = []

    v520_breakdown_cont = []
    v520_breakdown_exe = []
    v520_breakdown_net = []
    v520_breakdown_vmm = []

    def add_continuations(device_str, overhead_lst, exe_list, net_lst, vmm_lst, batch=4096):
        try:
            # select best performing result...
            new_device_str = []
            for bench in ['gpu_bench_scrypt',
                          'gpu_bench_pbkdf2',
                          'gpu_bench_imageblur',
                          'gpu_bench_imageblur_bmp',
                          'gpu_bench_imagehash',
                          'gpu_bench_imagehash_modified',
                          'gpu_bench_genpdf',
                          'gpu_bench_average',
                          'gpu_bench_lz4',
                          'gpu_bench_nlp-count-vectorizer',
                          'gpu_bench_nlp-go',
                          'gpu_bench_nlp-assemblyscript']:
                #print (results[device_str+"4"]['gpu'], bench)
                x = results[device_str+"4"]['gpu'][bench]
                y = results[device_str+"8"]['gpu'][bench]
                if x['rps'] > y['rps']:
                    new_device_str.append(device_str+"4")
                else:
                    new_device_str.append(device_str+"8")

            #new_device_str = [device_str+"4" if x['rps'] >= y['rps'] else device_str+"8" for x, y in zip(results[device_str+"4"]['gpu'].values(),
            #                                                                                                    results[device_str+"8"]['gpu'].values())]
            print (new_device_str)
            new_results = dict()
            benches = ['gpu_bench_scrypt',
                          'gpu_bench_pbkdf2',
                          'gpu_bench_imageblur',
                          'gpu_bench_imageblur_bmp',
                          'gpu_bench_imagehash',
                          'gpu_bench_imagehash_modified',
                          'gpu_bench_genpdf',
                          'gpu_bench_average',
                          'gpu_bench_lz4',
                          'gpu_bench_nlp-count-vectorizer',
                          'gpu_bench_nlp-go',
                          'gpu_bench_nlp-assemblyscript']
            best_performing = []
            for device, bench in zip(new_device_str, benches):
                new_results[device_str] = results[device]
                best_performing.append(results[device]['gpu'][bench])
            print (best_performing)

            overhead = np.array([x['overhead'] for x in best_performing]) / 10**9
            print ("overhead", overhead)
            exe = np.array([x['on_dev_exe_time'] - x['overhead'] for x in best_performing]) / 10**9
            # we use 2x as many requests as we have VMs, so device time really represents the sum of two requests on average
            # latency is similarly doubled
            network = np.array([x['latency'] - (x['device_time']) for x in best_performing]) / 10**9
            vmm = np.array([((x['device_time']) - (x['on_dev_exe_time']*2)) for x in best_performing]) / 10**9
            np.clip(vmm, 0, 99999999999, out=vmm)
            overhead_lst.extend(overhead)
            exe_list.extend(exe)
            net_lst.extend(network)
            vmm_lst.extend(vmm)
        except Exception as e:
            print (e)
            overhead_lst.append(0)
            exe_list.append(0)
            net_lst.append(0)
            vmm_lst.append(0)

    add_continuations("t4_", t4_cont, t4_exe, t4_net, t4_vmm)
    add_continuations("a10g_", t4_breakdown_cont, t4_breakdown_exe, t4_breakdown_net, t4_breakdown_vmm)
    #add_continuations("v520_profile_4", v520_breakdown_cont, v520_breakdown_exe, v520_breakdown_net, v520_breakdown_vmm)
    #print ("t4_vmm,", t4_vmm)

    # swap pbkdf2 and genpdf
    t4_vmm[6], t4_vmm[1] = t4_vmm[1], t4_vmm[6]
    t4_cont[6], t4_cont[1] = t4_cont[1], t4_cont[6]
    t4_exe[6], t4_exe[1] = t4_exe[1], t4_exe[6]
    t4_net[6], t4_net[1] = t4_net[1], t4_net[6]

    t4_breakdown_vmm[6], t4_breakdown_vmm[1] = t4_breakdown_vmm[1], t4_breakdown_vmm[6]
    t4_breakdown_cont[6], t4_breakdown_cont[1] = t4_breakdown_cont[1], t4_breakdown_cont[6]
    t4_breakdown_exe[6], t4_breakdown_exe[1] = t4_breakdown_exe[1], t4_breakdown_exe[6]
    t4_breakdown_net[6], t4_breakdown_net[1] = t4_breakdown_net[1], t4_breakdown_net[6]

    labels = ['Scrypt', 'Pbkdf2', 'Blur-Jpeg', 'Blur-Bmp', 'PHash', 'PHash-M.', 'Genpdf', 'Histogram', 'LZ4', 'Strings', 'Strings-Go', 'Strings-AScript']
    labels[6], labels[1] = labels[1], labels[6]
    
    t4_vmm = np.array(t4_vmm)
    t4_net = np.array(t4_net)
    t4_cont = np.array(t4_cont)
    t4_exe = np.array(t4_exe)
    print ("e2e: ", t4_exe+t4_net+t4_cont+t4_exe)

    t4_res = (t4_vmm + t4_net) / (t4_cont+t4_exe+t4_net+t4_vmm)
    t4_cont_frac = (t4_cont) / (t4_exe+t4_cont)

    print ("t4 vmm+net frac:")
    for label, idx in zip(labels, range(12)):
        print (label, t4_res[idx])
    print ("t4 cont frac:")
    for label, idx in zip(labels, range(12)):
        print (label, t4_cont_frac[idx], t4_cont[idx])
    print (t4_cont)

    print (t4_breakdown_exe)

    N = len(t4_cont)
    print (N)

    fig, axes = plt.subplots(nrows=1, ncols=2, figsize=(16, 5))
    fig.tight_layout()

    axes[0].set_xticks(ind_ticks, labels, rotation=75)
    axes[1].set_xticks(ind_ticks, labels, rotation=75)

    #plt.xticks(ind2, ["{}".format(int((2**x) / 1024)) for x in range(12,19)])
    #axes[0].set_ylim(0, 650)

    axes[0].set_ylabel('E2E Latency (s)')
    axes[0].set_ylim(0, 45)
    axes[1].set_ylim(0, 45)
    axes[1].get_yaxis().set_ticklabels([])

    #plt.xlabel('Copy Size (KiB)')
    axes[0].set_title('T4 Latency Breakdown')
    axes[1].set_title('A10G Latency Breakdown')
    plt.subplots_adjust(wspace=0.025)
    #plt.yscale('log')

    colors = plt.cm.viridis(np.linspace(0, 1, 13))  

    print (t4_exe)
    print (t4_vmm)

    p1 = axes[0].bar(ind, t4_cont, width, color='blue', hatch='.')

    p2 = axes[0].bar(ind, t4_exe, width, color='lightgray', hatch='/\\',
                bottom=np.asarray(t4_cont))

    p3 = axes[0].bar(ind, t4_vmm, width, color='black', hatch='o.',
                bottom=np.asarray(t4_exe)+np.asarray(t4_cont))

    p4 = axes[0].bar(ind, t4_net, width, color='green', hatch='o',
                bottom=np.asarray(t4_exe)+np.asarray(t4_cont)+np.asarray(t4_vmm))

    axins = inset_axes(axes[0], width=3, height=3, loc=1)
    mark_inset(axes[0], axins, loc1=2, loc2=4, fc="none", ec="black")
    axins.set_xlim([87.5,185])
    axins.set_ylim([0,4.075])
    axins.bar(ind, t4_breakdown_cont, width, color='blue', hatch='.')
    axins.bar(ind, t4_breakdown_exe, width, color='lightgray', hatch='/\\',
                bottom=np.asarray(t4_breakdown_cont))
    axins.bar(ind, t4_breakdown_vmm, width, color='black', hatch='o.',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont))
    axins.bar(ind, t4_breakdown_net, width, color='green', hatch='o',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont)+np.asarray(t4_breakdown_vmm))
    axins.set_xticks([])
    axins.set_yticks(np.arange(0, 4.1, 0.5), np.arange(0, 4.1, 0.5), size=24)

    # a10g

    p1 = axes[1].bar(ind, t4_breakdown_cont, width, color='blue', hatch='.')


    p2 = axes[1].bar(ind, t4_breakdown_exe, width, color='lightgray', hatch='/\\',
                bottom=np.asarray(t4_breakdown_cont))
 

    p3 = axes[1].bar(ind, t4_breakdown_vmm, width, color='black', hatch='o.',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont))

    p4 = axes[1].bar(ind, t4_breakdown_net, width, color='green', hatch='o',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont)+np.asarray(t4_breakdown_vmm))
    
    # plot subplot
    axins = inset_axes(axes[1], width=3, height=3, loc=1)
    mark_inset(axes[1], axins, loc1=2, loc2=4, fc="none", ec="black")
    axins.set_xlim([87.5,185])
    axins.set_ylim([0,4.075])
    axins.bar(ind, t4_breakdown_cont, width, color='blue', hatch='.')
    axins.bar(ind, t4_breakdown_exe, width, color='lightgray', hatch='/\\',
                bottom=np.asarray(t4_breakdown_cont))
    axins.bar(ind, t4_breakdown_vmm, width, color='black', hatch='o.',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont))
    axins.bar(ind, t4_breakdown_net, width, color='green', hatch='o',
                bottom=np.asarray(t4_breakdown_exe)+np.asarray(t4_breakdown_cont)+np.asarray(t4_breakdown_vmm))
    axins.set_xticks([])
    axins.set_yticks(np.arange(0, 4.1, 0.5), np.arange(0, 4.1, 0.5), size=24)

    axes[0].grid(zorder=-50, axis='y')
    axes[1].grid(zorder=-50, axis='y')
    plt.legend((p4[0], p3[0], p2[0], p1[0]),
    ('Network', 'VMM Overhead', 'On Device Execution Time', 'Continuations Overhead'),
    prop={'size': 32}, bbox_to_anchor=(-3.76, -2.3, 4.78, 4), loc='upper center',
                      ncol=2, mode="expand", borderaxespad=0.)

    plt.savefig(input_dir+"/breakdown.eps", bbox_inches='tight')
    plt.savefig(input_dir+"/breakdown.png", bbox_inches='tight')

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


def plot_bars_long(results, per_dollar=False):
    plt.clf()
    fig = plt.figure(figsize=(16, 6))
    plt.rc('xtick', labelsize=24)
    plt.rc('ytick', labelsize=24)
    plt.rc('axes', titlesize=20)
    plt.rc('axes', labelsize=20)
    plt.xticks(rotation=50)
    N = 12 
    ind = np.arange(N)*4    # the x locations for the groups
    width = 0.7
    print(plt.gcf())

    plt.grid(which='minor', color='lightgrey')
    # mapping of index to bench
    mapping = dict()
    mapping["Scrypt"] = 0
    mapping["Pbkdf2"] = 1
    mapping["Blur-Jpeg"] = 2
    mapping["Blur-Bmp"] = 3
    mapping["PHash"] = 4
    mapping["PHash-Mod"] = 5
    mapping["Bill-PDF"] = 6
    mapping["Histogram"] = 7
    mapping["LZ4"] = 8
    mapping["Strings-Rust"] = 9
    mapping["Strings-Go"] = 10
    mapping["Strings-AScript"] = 11

    # plot scrypt
    def get_best(bench, device_type, config, per_dollar=False):
        best_rps = 0
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_4'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
                elif config == "x86" or config == "wasm":
                    vals = np.array(vals) / 0.154
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_8'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
                elif config == "x86" or config == "wasm":
                    vals = np.array(vals) / 0.17
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_profile_4'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_profile_8'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
            best_rps = max(best_rps, vals)
        except:
            pass
        return best_rps

    row = 0
    col = 0
    print ("per-dollar:", per_dollar)
    t4_vals = []
    a10g_vals = []
    v520_vals = []
    x86_vals = []
    wasm_vals = []
    for bench in ["Scrypt", "Pbkdf2", "Blur-Jpeg", "Blur-Bmp", "PHash", "PHash-Mod", "Bill-PDF",
                  "Histogram", "LZ4", "Strings-Rust", "Strings-Go", "Strings-AScript"]:
        t4_best = get_best(bench, "t4", "gpu", per_dollar=per_dollar)
        a10g_best = get_best(bench, "a10g", "gpu", per_dollar=per_dollar)
        amd_best = get_best(bench,  "v520", "gpu", per_dollar=per_dollar)
        x86_best = get_best(bench, "t4", "x86", per_dollar=per_dollar)
        wasm_best = get_best(bench, "t4", "wasm", per_dollar=per_dollar)

        if x86_best > 0: 
            best_vals = np.array([t4_best, a10g_best, amd_best, x86_best, wasm_best]) / x86_best
        else:
            # Strings-Go/Strings-AScript
            # normalize to Strings-Rust x86 instead
            x86_best = get_best("Strings-Rust", "t4", "x86", per_dollar=per_dollar)
            best_vals = np.array([t4_best, a10g_best, amd_best, x86_best, wasm_best]) / x86_best
            #best_vals = np.array([t4_best, a10g_best, amd_best, 0, wasm_best]) / wasm_best

        t4_vals.append(best_vals[0])
        a10g_vals.append(best_vals[1])
        v520_vals.append(best_vals[2])
        x86_vals.append(best_vals[3])
        wasm_vals.append(best_vals[4])
    axes = plt.gca()
    plt.yscale('log')
    
    p1 = plt.bar(ind, t4_vals, width, label="T4", color="blue", hatch=".")
    p2 = plt.bar(ind+width, a10g_vals, width, label="A10G", color="lightgrey", hatch="/\\")
    p3 = plt.bar(ind+width*2, v520_vals, width, label="v520", color="black", hatch="o.")
    """
    p4 = plt.bar(ind+width*3, x86_vals, width, label="x86-64", color="green", hatch="o")
    """
    p4 = plt.bar(ind+width*3, wasm_vals, width, label="WASM", color="orange", hatch="x")
    labels = ["Scrypt", "Pbkdf2*", "Blur-Jpeg", "Blur-Bmp", "PHash", "PHash-Mod", "Bill-PDF*",
                  "Histogram", "LZ4", "Strings-Rust*", "Strings-Go*", "Strings-AScript*"]

    plt.xticks(ind+width*2, labels, rotation=30)
    plt.legend((p4[0], p3[0], p2[0], p1[0]),
               ('WASM', 'v520', 'A10G', 'T4'))

    if per_dollar:
        plt.ylabel("Normalized Throughput/$")
        plt.ylim(0.01, 100)
        plt.yticks([0.01, 0.1, 1, 10, 100])
        axes.set_yticklabels(["0.01x", "0.1x", "1x", "10x", "100x"])
    else:
        plt.ylabel("Normalized Throughput")
        plt.yticks([0.01, 0.1, 1, 10, 100])
        axes.set_yticklabels(["0.01x", "0.1x", "1x", "10x", "100x"])
        plt.ylim(0.01, 100)

    # plot x86
    baseline = plt.axhline(y=1, color='black', linestyle='dashed')
    plt.legend((baseline, p4[0], p3[0], p2[0], p1[0]),
               ('x86-64 baseline', 'WASM', 'v520', 'A10G', 'T4'), prop={'size': 24}, loc="upper center", ncols=5)

    print(plt.gcf())
    fig.tight_layout()
    print(plt.gcf())
    if per_dollar:
        plt.savefig(input_dir+"/barplots_per_dollar.eps", bbox_inches='tight')
        plt.savefig(input_dir+"/barplots_per_dollar.png", bbox_inches='tight')
    else:
        plt.savefig(input_dir+"/barplots.eps", bbox_inches='tight')
        plt.savefig(input_dir+"/barplots.png", bbox_inches='tight')
    plt.clf()

def plot_bars(results, per_dollar=False):
    plt.rc('xtick', labelsize=10)
    plt.rc('ytick', labelsize=10)
    plt.rc('axes', titlesize=10)
    plt.rc('axes', labelsize=10)
    plt.xticks(rotation=50)
    plt.legend()
    fig, axes = plt.subplots(2, 6, figsize=(9, 3))

    # mapping of index to bench
    mapping = dict()
    mapping["Scrypt"] = 0
    mapping["Pbkdf2"] = 1
    mapping["Blur-Jpeg"] = 2
    mapping["Blur-Bmp"] = 3
    mapping["PHash"] = 4
    mapping["PHash-Mod"] = 5
    mapping["Bill-PDF"] = 6
    mapping["Histogram"] = 7
    mapping["LZ4"] = 8
    mapping["Strings-Rust"] = 9
    mapping["Strings-Go"] = 10
    mapping["Strings-AScript"] = 11

    # plot scrypt
    def get_best(bench, device_type, config, per_dollar=False):
        best_rps = 0
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_4'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
                elif config == "x86" or config == "wasm":
                    vals = np.array(vals) / 0.154
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_8'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
                elif config == "x86" or config == "wasm":
                    vals = np.array(vals) / 0.17
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_profile_4'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
            best_rps = max(best_rps, vals)
        except:
            pass
        try:
            vals = list(map(lambda x: x['rps'], results['{device}_profile_8'.format(device=device_type)][config].values()))[mapping[bench]]
            if per_dollar:
                if config == "gpu" and device_type == "t4":
                    vals = np.array(vals) / 0.526
                elif config == "gpu" and device_type == "a10g":
                    vals = np.array(vals) / 1.006
                elif config == "gpu" and device_type == "v520":
                    vals = np.array(vals) / 0.3785
            best_rps = max(best_rps, vals)
        except:
            pass
        return best_rps

    row = 0
    col = 0
    print ("per-dollar:", per_dollar)
    for bench in ["Scrypt", "Pbkdf2", "Blur-Jpeg", "Blur-Bmp", "PHash", "PHash-Mod", "Bill-PDF",
                  "Histogram", "LZ4", "Strings-Rust", "Strings-Go", "Strings-AScript"]:
        t4_best = get_best(bench, "t4", "gpu", per_dollar=per_dollar)
        a10g_best = get_best(bench, "a10g", "gpu", per_dollar=per_dollar)
        amd_best = get_best(bench,  "v520", "gpu", per_dollar=per_dollar)
        x86_best = get_best(bench, "t4", "x86", per_dollar=per_dollar)
        wasm_best = get_best(bench, "t4", "wasm", per_dollar=per_dollar)

        missing_x86 = False
        if x86_best > 0: 
            best_vals = np.array([t4_best, a10g_best, amd_best, x86_best, wasm_best]) / x86_best
        else:
            # Strings-Go/Strings-AScript
            # normalize to Strings-Rust x86 instead
            x86_best = get_best("Strings-Rust", "t4", "x86", per_dollar=per_dollar)
            best_vals = np.array([t4_best, a10g_best, amd_best, x86_best, wasm_best]) / x86_best
            #best_vals = np.array([t4_best, a10g_best, amd_best, 0, wasm_best]) / wasm_best
            missing_x86 = True

        missing_amd = False
        if amd_best == 0:
            missing_amd = True
            #best_vals[2] = 1

        print (bench, best_vals)
        #axes[row, col].set_xticks(["T4", "A10G", "v520", "x86-64", "WASM"])

        labels = ["T4", "A10G", "v520", "x86-64", "WASM"]
        colors = ['blue', 'lightgrey', 'black', 'green', 'orange']
        hatches = ['.', '/\\', 'o.', 'o', 'x']
        #axes[row, col].set_yscale('log')
        #plt.yscale('log')
        bars = axes[row, col].bar(labels, best_vals, label=labels, color=colors, hatch=hatches)
        axes[row, col].set_xticklabels(labels, rotation=50)
        
        if missing_amd:
            axes[row, col].set_title(bench + "*")
        else:
            axes[row, col].set_title(bench)

        axes[row, col].set_xticks([])
        if per_dollar:
            axes[row, col].set_ylim(0, 2)
            axes[row, col].set_yticks([1, 2])
            axes[row, col].set_yticklabels(["1x", "2x"])
        else:
            #axes[row, col].set_ylim(0.1, 12)
            #plt.setp(axes[row, col].get_yminorticklabels(), visible=False)
            axes[row, col].set_yticks([1, 5, 10])
            axes[row, col].set_yticklabels(["1x", "5x", "10x"])
            axes[row, col].set_ylim(0, 12)

        # Strings-AScript
        #inset axes for non per_dollar
        if bench in ["Pbkdf2", "Blur-Jpeg", "PHash", 
                     "Histogram", "LZ4", "Strings-Rust", "Strings-Go", "Strings-AScript"] and not per_dollar:
            axins = inset_axes(axes[row, col], width=0.4, height=0.7, loc=1)
            mark_inset(axes[row, col], axins, loc1=1, loc2=2, fc="none", ec="black", linewidth=0.2)
            axins.set_xlim([-0.5,4.5])
            axins.set_ylim([0, 2.5])
            axins.set_yticks([1, 2])
            axins.set_yticklabels(["1x", "2x"])
            axins.bar(labels, best_vals, label=labels, color=colors, hatch=hatches)
            axins.set_xticks([])
            [x.set_linewidth(0.2) for x in axins.spines.values()]

        if bench in ["Strings-AScript"] and per_dollar: 
            axins = inset_axes(axes[row, col], width=0.4, height=0.3, loc=1)
            mark_inset(axes[row, col], axins, loc1=1, loc2=2, fc="none", ec="black", linewidth=0.2)
            axins.set_xlim([-0.5, 1.7])
            axins.set_ylim([0, 0.05])
            axins.set_yticks([0, 0.05])
            axins.set_yticklabels(["0x", "0.05x"])
            axins.bar(labels, best_vals, label=labels, color=colors, hatch=hatches)
            axins.set_xticks([])
            [x.set_linewidth(0.2) for x in axins.spines.values()]


        handles, labels = axes[row, col].get_legend_handles_labels()


        """
        if missing_amd and per_dollar:
            bar_height = bars[2].get_height()
            axes[row, col].text(x=bars[2].get_x() + bars[2].get_width() / 2, y=bar_height+1.2,
                                s="v520 N/A",
                                ha='center',
                                fontsize=8)
        elif missing_amd:
            bar_height = bars[2].get_height()
            axes[row, col].text(x=bars[2].get_x() + bars[2].get_width() / 2, y=bar_height+6,
                                s="v520 N/A",
                                ha='center',
                                fontsize=8)
        """
        col += 1
        if col >= 6:
            col = 0
            row += 1

    #axes[0, 0].set_ylabel("Normalized Throughput")
    #axes[1, 0].set_ylabel("Normalized Throughput")
    if per_dollar:
        fig.supylabel("Normalized Throughput/$")
    else:
        fig.supylabel("Normalized Throughput")

    #fig.legend(handles, labels, loc='upper center', ncol=5)
    fig.legend(handles, labels, loc='upper center', ncol=5, bbox_to_anchor=(0.5, 1.1, 0, 0))

    fig.tight_layout() 
    if per_dollar:
        plt.savefig(input_dir+"/barplots_per_dollar.eps", bbox_inches='tight')
    else:
        plt.savefig(input_dir+"/barplots.eps", bbox_inches='tight')
    plt.clf()

results = dict()
results['t4_cuda'] = parse_dir(input_dir+"t4_cuda")
results['t4_cuda_2x'] = parse_dir(input_dir+"t4_cuda_2x")
results['a10g_cuda'] = parse_dir(input_dir+"a10g_cuda")
results['a10g_cuda_2x'] = parse_dir(input_dir+"a10g_cuda_2x")
results['t4_4'] = parse_dir(input_dir+"t4_amd_4")
results['t4_profile_4'] = parse_dir(input_dir+"t4_amd_4_profile")
results['t4_breakdown_4'] = parse_dir(input_dir+"t4_amd_4_breakdown")
results['t4_breakdown_profile_4'] = parse_dir(input_dir+"t4_amd_4_breakdown_profile")
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
plot_breakdowns(results)

#plot_bars(results, per_dollar=False)
#plot_bars(results, per_dollar=True)

plot_bars_long(results, per_dollar=False)
plot_bars_long(results, per_dollar=True)

vals = list(map(lambda x: x, results['t4_4']['gpu'].keys()))

m = dict()
for bench, bench2 in zip(results['t4_4']['gpu'].keys(), results['t4_4']['x86'].keys()):
    m[bench] = bench2

for bench, res in results['t4_4']['gpu'].items():
    best_t4 = max(results['t4_4']['gpu'][bench]['rps'] / 0.526, results['t4_8']['gpu'][bench]['rps'] / 0.526)
    best_t4_2 = max(results['t4_profile_4']['gpu'][bench]['rps'] / 0.526, results['t4_profile_8']['gpu'][bench]['rps'] / 0.526)
    best_t4 = max(best_t4, best_t4_2)

    best_a10g = max(results['a10g_4']['gpu'][bench]['rps'] / 1.006, results['a10g_8']['gpu'][bench]['rps'] / 1.006)
    best_a10g_2 = max(results['a10g_profile_4']['gpu'][bench]['rps'] / 1.006, results['a10g_profile_8']['gpu'][bench]['rps'] / 1.006)
    best_a10g = max(best_a10g, best_a10g_2)

    try:
        best_intel = max(results['t4_8']['x86'][m[bench]]['rps'] / 0.17, results['t4_8']['x86'][m[bench]]['rps'] / 0.17)
        best_amd = max(results['t4_4']['x86'][m[bench]]['rps'] / 0.154, results['t4_4']['x86'][m[bench]]['rps'] / 0.154)

        print ("Intel: ", bench, max(best_t4 / best_intel, best_a10g / best_intel))
        print ("AMD: ", bench, max(best_t4 / best_amd, best_a10g / best_amd))
    except Exception as e:
        print (e)
        pass

m2 = dict()
for bench, bench2 in zip(results['t4_4']['gpu'].keys(), results['t4_4']['wasm'].keys()):
    m2[bench] = bench2

print ("throughput results")
for bench, res in results['t4_4']['gpu'].items():
    best_t4 = max(results['t4_4']['gpu'][bench]['rps'], results['t4_8']['gpu'][bench]['rps'] )
    best_t4_2 = max(results['t4_profile_4']['gpu'][bench]['rps'], results['t4_profile_8']['gpu'][bench]['rps'] )
    best_t4 = max(best_t4, best_t4_2)

    best_a10g = max(results['a10g_4']['gpu'][bench]['rps'], results['a10g_8']['gpu'][bench]['rps'])
    best_a10g_2 = max(results['a10g_profile_4']['gpu'][bench]['rps'], results['a10g_profile_8']['gpu'][bench]['rps'])
    best_a10g = max(best_a10g, best_a10g_2)

    try:
        best_intel = max(results['t4_8']['x86'][m[bench]]['rps'], results['t4_8']['x86'][m[bench]]['rps'])
        best_amd = max(results['t4_4']['x86'][m[bench]]['rps'], results['t4_4']['x86'][m[bench]]['rps'])
        best_amd_wasm = max(results['t4_4']['wasm'][m2[bench]]['rps'], results['t4_4']['wasm'][m2[bench]]['rps'])
        best_amd_exe = min(results['t4_4']['x86'][m[bench]]['on_dev_exe_time'], results['t4_4']['x86'][m[bench]]['on_dev_exe_time'])
        best_intel_exe = min(results['t4_8']['x86'][m[bench]]['on_dev_exe_time'], results['t4_8']['x86'][m[bench]]['on_dev_exe_time'])
        best_t4_exe = min(results['t4_4']['gpu'][bench]['on_dev_exe_time'], results['t4_8']['gpu'][bench]['on_dev_exe_time'] )
        best_t4_exe_prof = min(results['t4_profile_4']['gpu'][bench]['on_dev_exe_time'], results['t4_profile_8']['gpu'][bench]['on_dev_exe_time'] )
        best_t4_exe = min(best_t4_exe, best_t4_exe_prof)
        print ("EXE ratio T4/AMD: ", bench, best_t4_exe / best_amd_exe)
        #print ("EXE ratio T4/Intel: ", bench, best_t4_exe / best_intel_exe)
        #print ("AMD (x86): ", bench, max(best_t4 / best_amd, best_a10g / best_amd))
        #print ("AMD (WASM): ", bench, max(best_t4 / best_amd_wasm, best_a10g / best_amd_wasm))
    except Exception as e:
        print (e)
        pass

# Strings Go/AssemblyScript
for bench, res in results['t4_4']['gpu'].items():
    best_t4_4 = np.average(results['t4_4']['gpu'][bench]['rps'])
    best_t4_8 = np.average(results['t4_8']['gpu'][bench]['rps'])
    best_t4_4_prof = np.average(results['t4_profile_4']['gpu'][bench]['rps'] )
    best_t4_8_prof = np.average(results['t4_profile_8']['gpu'][bench]['rps'])
    best_amd_wasm = max(results['t4_4']['wasm'][m2[bench]]['rps'], results['t4_8']['wasm'][m2[bench]]['rps'])
    avg = (best_t4_4 + best_t4_8 + best_t4_4_prof + best_t4_8_prof)/4
    print ("t4", bench, avg)

    best_t4_4 = np.average(results['a10g_4']['gpu'][bench]['rps'])
    best_t4_8 = np.average(results['a10g_8']['gpu'][bench]['rps'])
    best_t4_4_prof = np.average(results['a10g_profile_4']['gpu'][bench]['rps'] )
    best_t4_8_prof = np.average(results['a10g_profile_8']['gpu'][bench]['rps'])
    avg = np.average((best_t4_4 + best_t4_8 + best_t4_4_prof + best_t4_8_prof)/4)
    print ("a10g", bench, avg)
