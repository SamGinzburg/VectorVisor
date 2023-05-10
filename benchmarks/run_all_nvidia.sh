# param 1 --> AMD AMI
# param 2 --> T4 AMI
# param 3 --> A10G AMI
# param 4 --> awsarn
# e.g., ./run_all.sh AMD T4 A10G

dir_name=$(printf '%(%Y-%m-%d-%H:%M:%S)T\n' -1)
amd_membench="$dir_name/amd_membench/"
t4_membench="$dir_name/t4_membench/"
a10g_membench="$dir_name/a10g_membench/"
t4_cuda="$dir_name/t4_cuda/"
t4_cuda_2x="$dir_name/t4_cuda_2x/"
t4_dir_4="$dir_name/t4_amd_4/"
t4_dir_8="$dir_name/t4_amd_8/"
t4_dir_4_breakdown="$dir_name/t4_amd_4_breakdown/"
t4_dir_8_breakdown="$dir_name/t4_amd_8_breakdown/"
t4_dir_4_breakdown_profile="$dir_name/t4_amd_4_breakdown_profile/"
t4_dir_4_profile="$dir_name/t4_amd_4_profile/"
t4_dir_8_profile="$dir_name/t4_amd_8_profile/"
a10g_cuda="$dir_name/a10g_cuda/"
a10g_cuda_2x="$dir_name/a10g_cuda_2x/"
a10g_dir_4="$dir_name/a10g_intel_4/"
a10g_dir_8="$dir_name/a10g_intel_8/"
a10g_dir_4_breakdown="$dir_name/a10g_intel_4_breakdown/"
a10g_dir_8_breakdown="$dir_name/a10g_intel_8_breakdown/"
a10g_dir_4_profile="$dir_name/a10g_intel_4_profile/"
a10g_dir_8_profile="$dir_name/a10g_intel_8_profile/"
v520_dir_4_profile="$dir_name/v520_4_profile/"
v520_dir_8_profile="$dir_name/v520_8_profile/"
v520_dir_4_breakdown="$dir_name/v520_4_breakdown/"
v520_dir_8_breakdown="$dir_name/v520_8_breakdown/"
mkdir -p $dir_name

echo "Starting A10G benchmarks..."
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=amd --interleave=4 --dir=$a10g_dir_4 --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=intel --interleave=8 --dir=$a10g_dir_8 --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running a10g benchmarks"
sleep 60

echo "Starting A10G profile benchmarks..."
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=amd --interleave=4 --dir=$a10g_dir_4_profile --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True --run-profile True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=intel --interleave=8 --dir=$a10g_dir_8_profile --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True --run-profile True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running a10g profile benchmarks"
sleep 60

echo "Starting A10G breakdown benchmarks..."
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=amd --interleave=4 --dir=$a10g_dir_4_breakdown --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True --breakdown=True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=intel --interleave=8 --dir=$a10g_dir_8_breakdown --ami=$3 --cpuami=$3 --skip-cpu True --skip-membench True --breakdown=True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running a10g breakdown benchmarks"
sleep 60

# Run T4 benchmarks...
# Run both w/CPU benchmarks, 4,8 interleaves
echo "Starting T4 benchmarks w/CPU..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=amd --interleave=4 --dir=$t4_dir_4 --ami=$2 --cpuami=$2 --skip-membench=True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=intel --interleave=8 --dir=$t4_dir_8 --ami=$2 --cpuami=$2 --skip-membench=True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running t4+CPU benchmarks"
sleep 60

# Run T4 profiled benchmarks...
echo "Starting T4 profile benchmarks..."
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=amd --interleave=4 --dir=$t4_dir_4_profile --ami=$2 --cpuami=$2 --skip-cpu True --skip-membench True --run-profile True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=intel --interleave=8 --dir=$t4_dir_8_profile --ami=$2 --cpuami=$2 --skip-cpu True --skip-membench True --run-profile True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running t4 profile benchmarks"
sleep 60

echo "Starting T4 benchmarks breakdown w/CPU..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=amd --interleave=4 --dir=$t4_dir_4_breakdown --ami=$2 --cpuami=$2 --skip-cpu True --skip-membench True --breakdown=True & 
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=intel --interleave=8 --dir=$t4_dir_8_breakdown --ami=$2 --cpuami=$2 --skip-cpu True --skip-membench True --breakdown=True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running T4 breakdown benchmarks"
sleep 60


echo "Starting T4 profile benchmarks breakdown w/CPU..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=amd --interleave=4 --dir=$t4_dir_4_breakdown_profile --ami=$2 --cpuami=$2 --skip-cpu True --skip-membench True --breakdown=True --run-profile True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running T4 breakdown benchmarks"
sleep 60

# Now finish things off with the membench runs...

# T4 membench
echo "Starting T4 membench..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=t4 --cpu=amd --interleave=4 --dir=$t4_membench --ami=$2 --cpuami=$2 --skip-cpu True --membench=True
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running T4 membench"
sleep 60

#A10G membench
echo "Starting A10G membench..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --awsarn=$4 --gpu=a10g --cpu=amd --interleave=4 --dir=$a10g_membench --ami=$3 --cpuami=$3 --skip-cpu True --membench=True
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running A10G membench"
sleep 60

echo "Starting T4 CUDA benchmarks..."
STARTTIME=$(date +%s)
python3 run_cuda.py --awsarn=$4 --gpu=t4 --cpu=amd --double=True --interleave=8 --dir=$t4_cuda_2x --ami=$2 --cpuami=$2 --skip-membench True --skip-cpu True --run-profile True & 
python3 run_cuda.py --awsarn=$4 --gpu=t4 --cpu=amd --double=False --interleave=8 --dir=$t4_cuda --ami=$2 --cpuami=$2 --skip-membench True --skip-cpu True --run-profile True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running t4 profile benchmarks"
sleep 60

echo "Starting A10G CUDA benchmarks..."
STARTTIME=$(date +%s)
python3 run_cuda.py --awsarn=$4 --gpu=a10g --cpu=amd --double=True --interleave=8 --dir=$a10g_cuda_2x --ami=$3 --cpuami=$3 --skip-membench True --skip-cpu True --run-profile True & 
python3 run_cuda.py --awsarn=$4 --gpu=a10g --cpu=amd --double=False --interleave=8 --dir=$a10g_cuda --ami=$3 --cpuami=$3 --skip-membench True --skip-cpu True --run-profile True & 
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running t4 profile benchmarks"
sleep 60

echo "Finished all benchmarks!"
echo "$(($ENDTIME - $TOTAL_START_TIME)) seconds ellapsed"
