# VectorVisor Evaluation

This subdirectory contains the evaluation materials for the USENIX ATC 2023 paper "VectorVisor: A Binary Translation Scheme for Throughput-Oriented GPU Acceleration".

There are two primary components to our evaluation:
- VectorVisor, the vectorizing binary translator for GPUs (https://github.com/SamGinzburg/VectorVisor)
- Our PGO (profile-guided optimization) instrumentation tool (https://github.com/SamGinzburg/vv-pgo-instrument)

VectorVisor can be built directly from source, although we also offer prepackaged Amazon AWS AMIs for cloud evaluation.

## Building from source (local testing)

### Prerequisites:
1. Ubuntu 18.04 LTS
2. CUDA 12 (NVIDIA driver version 525)
3. OpenCL C development headers & libraries
4. Stable Rust 1.6+

To confirm that the GPU driver and OpenCL setup is complete, run "clinfo" and/or "nvidia-smi", and ensure that "cargo" is in your $PATH.

Examples of how we build VectorVisor from source can be seen in the "make_image.py" script (used to generate AWS AMI images).

## Cloud Evaluation

TODO
