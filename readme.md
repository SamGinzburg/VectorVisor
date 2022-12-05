# VectorVisor

GPU programming models offer a level of abstraction that is either too low-level (e.g., OpenCL, CUDA) or too high-level (e.g., TensorFlow, Halide), depending on the language. Not all appli- cations fit into either category, resulting in lost opportunities for GPU acceleration.

VectorVisor is a vectorized binary translator which remedies this issue by taking in existing single-threaded WebAssembly programs and running many copies of them using a GPU. Unlike OpenCL or CUDA programs, we provide support for system calls and a CPU-like flat memory model. While less efficient than manual translation, this approach substantially reduces the barrier to accelerating throughput-oriented workloads using GPUs, ultimately improving the throughput of applications that would otherwise run on CPUs.

## Installation, Setup, and Hardware Compatibility

### Installation & Setup

VectorVisor requires OpenCL 1.2+ to be installed, in addition to having the proper GPU drivers installed and OpenCL development header files. VectorVisor is built using Rust, and requires a recent version of stable rust to compile as well.

OpenCL and GPU driver setups can be verified by running:
```
clinfo
```

VectorVisor can be built using cargo:

```
cargo build --release
```

### Hardware Compatibility

VectorVisor was built with compatibility in mind, and should *theoretically* run on any GPU supporting OpenCL 1.2. In practice, VectorVisor has been mostly evaluated using NVIDIA GPUs on Linux. No Windows based setups have been evaluated, but before attempting this, ensure that TDR is either disabled or set to a larger timeout value.

Devices with full functionality should be able to run any of our benchmarks or examples. Partial functionality varies by device, but these devices should be able to run short examples at the minimum.

Vendor | Evaluated OS | GPU | Level of Support
------------- | ------------- | ------------- | -------------
NVIDIA | Linux | GTX 1080 Ti | ✅
NVIDIA | Linux | RTX 2080 Ti | ✅
NVIDIA | Linux | RTX 3080 Ti | ✅
NVIDIA | Linux | T4 | ✅
NVIDIA | Linux | A10G | ✅
NVIDIA | Linux | V100 | ✅
AMD (ROCm/HSA OpenCL) | Linux | AMD Radeon Pro V520 | ⚠️
Intel  | macOS | Iris Pro | ⚠️

Intel devices feature limited support, but fail for programs more complex than our smoke tests (compilation failures, possibly due to compiler bugs in the Intel OpenCL C compiler). AMD devices (ROCm/HSA OpenCL) run (but sometimes crash). Generally, NVIDIA GPUs obtain the best performance, although newer Intel/AMD dedicated GPUs have not been tested. 

All non-nvidia targets should be run with the following flags:
```
--nvidia=false
```

AMD targets need to be run with:
```
--patch=true
```

## Configuring VectorVisor

VectorVisor has many different configuration options which can be accessed with "--help"
```
cargo run --release -- --help
```

We include a benchmark suite (benchmarks/run_benchmarks_aws.py), which provides examples of how to run our sample benchmarks using different configurations. Generally, real applications require a heap size of 3--4 MiB, a stack size of 128 KiB, a hypercall buffer of 128-512 KiB, along with several other flags regarding application partitioning and "pretty" inputs. Different GPU configurations support varying amounts of concurrent VMs, based on the available GPU memory (e.g., 11 GiB, 16 GiB, 24 GiB) and application requirements.

## Example Usage

We include both complete end-to-end benchmarks as well as a series of simpe smoke tests to confirm that VectorVisor is working properly. 

### Simple Examples

We include a series of simple examples in the examples/ directory. The "printreturn" flag is useful for debugging simple programs that return a value.

```
cd examples/
cargo run --release -- -i arithmetic/factorial.wat --printreturn=true
```

### Running Full Applications

Our end-to-end benchmarks are built with the "wasm-serverless-invoke" library (wasm-serverless-invoke), which provides an interface for VectorVisor to transfer inputs to and from running VMs on the GPU. Examples of programs using this library can be found in the benchmarks/ directory (e.g., benchmarks/scrypt/, benchmarks/average, benchmarks/imageblur-bmp/, ...).
