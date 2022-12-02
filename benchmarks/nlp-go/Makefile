all: main.go
	tinygo build -target wasi.json -no-debug -panic=trap
	wasm-opt -O3 -g nlp-go.wasm -o release-opt.wasm
