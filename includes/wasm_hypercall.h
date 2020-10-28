#ifndef WASM_HYPERCALL
#define WASM_HYPERCALL

#include "../uvwasi/include/wasi_types.h"
#include "../uvwasi/include/uvwasi.h"

#ifdef DEBUG
#else
#include <OpenCL/opencl.h>
#endif

#define uchar unsigned char
#define ulong unsigned long
#define uint unsigned int

/*
 * hypercall 0, fd_write
 * 
 * Take in the user stack & heap, we will obtain the parameters
 * 
 */
#ifdef DEBUG
uint vmm_fd_write(uvwasi_t *wasi_instance, uint *stack_u32, ulong *sp, uint *heap_u32) {
    uvwasi_size_t size;
    uvwasi_size_t bytes_written;
    uvwasi_ciovec_t *ciovec;
    uvwasi_size_t iovs_len;
    uvwasi_fd_t fd;

    // ciovec contains the buffer and length of the buffer
    /*
     * (i32.const 1)   ;; fd 1 (stdout)
     * (i32.const 8)   ;; (iovec*)8
     * (i32.const 1)   ;; 1 iovec
     * (i32.const 12)) ;; write the number of written bytes back to iovec.buf_len
     * 
     * Get the data from the stack, use it to read data from the heap
     * 
     */

    
    // set the fd
    fd = (uvwasi_fd_t)stack_u32[*sp-4];

    // the iovec pointer
    //stack_u32[*sp-3];

    // the number of iovecs
    iovs_len = (uvwasi_size_t)stack_u32[*sp-2];

    // allocate the ciovec, which is an array of (*buf, buf_len) structs
    ciovec = calloc(iovs_len, sizeof(*ciovec));
    for (uint idx = 0; idx < iovs_len; idx++) {
        // in debug mode we don't have to copy anything
        // on the GPU we have to copy from the GPU buffer to the CPU
        uint *heap_addr = heap_u32 + (stack_u32[*sp-3]/4 + (sizeof(*ciovec) * idx));
        uint buf_len   = heap_u32[stack_u32[*sp-3]/4 + 1 + (sizeof(*ciovec) * idx)];
        ciovec[idx].buf = malloc(buf_len);
        ciovec[idx].buf_len = buf_len;
        memcpy(ciovec[0].buf, heap_u32+(*heap_addr/4), buf_len);
    }


    uvwasi_fd_write(wasi_instance, fd, ciovec, iovs_len, &bytes_written);

    // before returning, we must write the result of the call back to the stack
    stack_u32[*sp] = (uint)bytes_written;
    *sp += 1;

    // free the allocated temp buffers
    for (uint idx = 0; idx < iovs_len; idx++) {
        free(ciovec[0].buf);
    }
    free(ciovec);

    return (uint) bytes_written;
}
#else
uint vmm_fd_write(uvwasi_t *wasi_instance, cl_mem stack,
                  ulong stack_ptr, cl_mem heap, uint warp_id,
                  cl_command_queue commands, ulong stack_size, ulong heap_size) {
    uvwasi_size_t size;
    uvwasi_size_t bytes_written;
    uvwasi_ciovec_t *ciovec;
    uvwasi_size_t iovs_len;
    uvwasi_fd_t fd;
    uvwasi_errno_t err;

    // wtf why does this have to be 5????
    uint temp_stack_vals[5];
    uint ciovec_temp[2];
    ulong heap_offset;

    printf ("offset = %p\n", (stack_size * warp_id * sizeof(uint)) + stack_ptr - 4);
    printf ("stack ptr: %d\n", stack_ptr);
    // read the last 4 32 bit values off of the stack
    clEnqueueReadBuffer(commands, stack, CL_TRUE, (stack_size * warp_id * sizeof(uint)) + stack_ptr - 4,
                        sizeof(uint) * 4, &temp_stack_vals, 0, NULL, NULL);  

    /*
    printf("temp_stack_vals[0] = %d\n", temp_stack_vals[0]);
    printf("temp_stack_vals[1] = %d\n", temp_stack_vals[1]);
    printf("temp_stack_vals[2] = %d\n", temp_stack_vals[2]);
    printf("temp_stack_vals[3] = %d\n", temp_stack_vals[3]);
    */
    /*
     * (i32.const 1)   ;; fd 1 (stdout)
     * (i32.const 8)   ;; (iovec*)8
     * (i32.const 1)   ;; 1 iovec
     * (i32.const 12)) ;; write the number of written bytes back to iovec.buf_len
     * 
     * Get the data from the stack, use it to read data from the heap
     * 
     */

    
    // set the fd
    fd = (uvwasi_fd_t)temp_stack_vals[0];

    // the number of iovecs
    iovs_len = (uvwasi_size_t)temp_stack_vals[2];
    
    // allocate the ciovec, which is an array of (*buf, buf_len) structs
    ciovec = calloc(iovs_len, sizeof(*ciovec));
    for (uint idx = 0; idx < iovs_len; idx++) {
        // for each ciovec, we need to copy the structure from the heap
        heap_offset = (heap_size * warp_id * 4) + (temp_stack_vals[1] + (sizeof(*ciovec) * idx));
        //printf("heap offset: %p\n", heap_offset);
        clEnqueueReadBuffer(commands, heap, CL_TRUE, heap_offset,
                            sizeof(uint) * 2, &ciovec_temp, 0, NULL, NULL);  

        //printf("ciovec_temp[0] = %p\n", ciovec_temp[0]);
        //printf("ciovec_temp[1] = %p\n", ciovec_temp[1]);
        // ciovec_temp[0] = ptr to buffer
        // ciovec_temp[1] = buf_len
        // now that we have the ciovec, we can read 
        ciovec[idx].buf = calloc(ciovec_temp[1], sizeof(char));
        ciovec[idx].buf_len = ciovec_temp[1];
        // copy from the heap
        heap_offset = (heap_size * warp_id * 4) + ciovec_temp[0];

        //printf("heap offset: %p\n", heap_offset);
        clEnqueueReadBuffer(commands, heap, CL_TRUE, heap_offset,
                            ciovec[idx].buf_len, ciovec[idx].buf, 0, NULL, NULL);  
        //printf("read complete: %p\n", ciovec[idx].buf);
        /*
        memcpy(ciovec[0].buf, heap_u32+(*heap_addr/4), buf_len);
        clEnqueueReadBuffer(commands, heap, CL_TRUE, heap_offset,
                            sizeof(uint) * 2, &ciovec_temp, 0, NULL, NULL);  
        */
    }

    err = uvwasi_fd_write(wasi_instance, fd, ciovec, iovs_len, &bytes_written);

    // before returning, we must write the result of the call back to the stack
    //stack_u32[*sp] = (uint)bytes_written;
    //*sp += 1;

    // free the allocated temp buffers
    for (uint idx = 0; idx < iovs_len; idx++) {
        free(ciovec[idx].buf);
    }
    free(ciovec);
    return (uint) bytes_written;
}
#endif

#endif
