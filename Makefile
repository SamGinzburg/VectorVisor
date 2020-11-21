debian:
		DOCKER_BUILDKIT=1 docker build -f Dockerfile.debian -t wasm-parser -o bin/ .
centos:
		DOCKER_BUILDKIT=1 docker build -f Dockerfile.centos -t wasm-parser -o bin/ .

