dir_name=$(printf '%(%Y-%m-%d-%H:%M:%S)T\n' -1)
t4_dir_4="$dir_name/t4_amd_4/"
a10g_dir_4="$dir_name/a10g_intel_4/"
t4_dir_8="$dir_name/t4_amd_8/"
a10g_dir_8="$dir_name/a10g_intel_8/"
mkdir -p $dir_name
echo "Starting interleave=4 benchmarks..."
TOTAL_START_TIME=$(date +%s)
STARTTIME=$(date +%s)
python3 run_benchmarks_aws.py --gpu=t4 --cpu=amd --interleave=8 --dir=$t4_dir_4 --skip-membench True --ami=$1 --run-profile True & 
#python3 run_benchmarks_aws.py --gpu=a10g --cpu=intel --interleave=4 --dir=$a10g_dir_4 --skip-membench True --ami=$1 &
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running interleave=4 benchmarks"
sleep 60
echo "Starting interleave=8 benchmarks..."
STARTTIME=$(date +%s)
#python3 run_benchmarks_aws.py --gpu=t4 --cpu=amd --interleave=8 --dir=$t4_dir_8 --skip-membench True & 
#python3 run_benchmarks_aws.py --gpu=a10g --cpu=intel --interleave=8 --dir=$a10g_dir_8 --skip-membench True &
for job in `jobs -p`; do wait ${job}; done
ENDTIME=$(date +%s)
echo "$(($ENDTIME - $STARTTIME)) seconds ellapsed while running interleave=8 benchmarks"
echo "Finished all benchmarks!"
echo "$(($ENDTIME - $TOTAL_START_TIME)) seconds ellapsed"
