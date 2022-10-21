all: main.go
	tinygo build -target wasi -no-debug -panic=trap
	wasm-opt -O3 hello_go.wasm -o hello_go_opt.wasm
