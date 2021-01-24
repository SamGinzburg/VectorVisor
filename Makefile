debian:
		DOCKER_BUILDKIT=1 docker build -f Dockerfile.debian -t wasm-parser -o bin/ .
centos:
		DOCKER_BUILDKIT=1 docker build -f Dockerfile.centos -t wasm-parser -o bin/ .
run:
		docker build -f Dockerfile.run_demo -t wasm-parser -o bin/ .
benchmarks:
		DOCKER_BUILDKIT=1 docker build -f Dockerfile.builddemo -t wasm-parser -o bin/ .

