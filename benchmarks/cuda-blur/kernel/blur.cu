extern "C" __global__ void blur(const unsigned char *input, unsigned char *output, const unsigned int width, const unsigned int height, const float *kernel, const unsigned int kernelWidth) {

    //const unsigned int col = threadIdx.x + blockIdx.x * blockDim.x;
    //const unsigned int row = threadIdx.y + blockIdx.y * blockDim.y;
    const unsigned int row = blockIdx.x;
    const unsigned int col = threadIdx.x;

    //printf("col: %d\n", col);
    //printf("row: %d\n", row);

    // blur once with the higher sigma kernel
    if (row < height && col < width) {
        const int half = kernelWidth / 2;
        float blur = 0.0;
        for(int i = -half; i <= half; i++) {
            for(int j = -half; j <= half; j++) {

                const unsigned int y = max(0, min(height - 1, row + i));
                const unsigned int x = max(0, min(width - 1, col + j));

                const float w = kernel[(j + half) + (i + half) * kernelWidth];
                blur += w * input[x + y * width];
            }
        }
        output[col + row * width] = static_cast<unsigned char>(blur);
    }
}
