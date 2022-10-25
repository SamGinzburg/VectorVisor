SRCS = $(wildcard *.cu)
PROGS = $(patsubst %.cu,%,$(SRCS))

all: $(PROGS)

%: %.cu
	/usr/local/cuda/bin/nvcc -O3 -o $@.ptx -c $< --ptx
