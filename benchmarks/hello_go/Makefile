all: main.go
	tinygo build -target wasi.json -no-debug -panic=trap
	wasm-opt -O1 -g hello_go.wasm -o hello_go_opt.wasm
