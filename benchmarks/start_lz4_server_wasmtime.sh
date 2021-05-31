for i in {0..15}
do
	cargo run --release -- -i json-compression.wasm -h 3145728 --partition=true --vmcount 64 --serverless=true --port=$((8000 + $i)) --ip 0.0.0.0 --vmgroups=1 --wasmtime=true --hcallsize=1048576 &>/dev/null &
done
