#include <stdint.h>

extern "C" __global__ void blockhash(unsigned char *input_img, unsigned int *blocks, const unsigned int block_width, const unsigned int block_height, const unsigned int bits) {

    // x, y pixel values
    const unsigned int x = blockIdx.x;
    const unsigned int y = threadIdx.y;
    // bits == the number of blocks to divide by horizontally/vertically
    // If img dims are 256x256 with a block size of 8, then bits==32

    // fast blockhash alg, assume width % 4 == 0 and height % 4 == 0

    // Blocks by default are 8 pixels x 8 pixels in size for our benchmark

    // For each pixel
    // 1) Compute the block value
    // 1.1) Check alpha channel for pixel, if alpha==0 then value = 765
    // 1.2) Else, value = sum(px) (RGB channels) for each value in a block
    // 1.3) Assign block value

    int64_t value = 0;
    for (uint64_t blocky = 0; blocky < block_height; blocky++) {
        for (uint64_t blockx = 0; blockx < block_width; blockx++) {
            uint64_t block_idx = ((y * block_height + blocky) * block_width + (x * block_width + blockx)) * 4;
            uint32_t alpha = input_img[block_idx+3];
            if (alpha == 0) {
                value += 765;
            } else {
                value += input_img[block_idx] + input_img[block_idx+1] + input_img[block_idx+2];
            }
        }
    }

    blocks[y * bits + x] = value;
}
