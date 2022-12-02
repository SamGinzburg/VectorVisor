all: main.go
	tinygo build -target wasi.json -no-debug -panic=trap
	wasm-opt -O3 -g go-strings.wasm -o go-strings-opt.wasm
