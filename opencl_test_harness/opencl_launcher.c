#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <OpenCL/opencl.h>
#include <time.h>
#include "../includes/wasm_hypercall.h"

#define DATA_SIZE (1024 * 100)

#define WARP_SIZE 16

#define STACK_SIZE_BYTES 1024 * 1024
#define HEAP_SIZE_BYTES  1024 * 1024

#define uchar unsigned char
#define ulong unsigned long
#define uint unsigned int

/* Create program from a file and compile it */
cl_program build_program(cl_context ctx, cl_device_id dev, const char* filename) {
    cl_program program;
    FILE *program_handle;
    char *program_buffer, *program_log;
    size_t program_size, log_size;
    int err;

    /* Read program file and place content into buffer */
    program_handle = fopen(filename, "r");
    if(program_handle == NULL) {
        perror("Couldn't find the program file");
        exit(1);
    }

    fseek(program_handle, 0, SEEK_END);
    program_size = ftell(program_handle);
    rewind(program_handle);
    program_buffer = (char*)malloc(program_size + 1);
    program_buffer[program_size] = '\0';
    fread(program_buffer, sizeof(char), program_size, program_handle);
    fclose(program_handle);

    /* Create program from file 

    Creates a program from the source code in the add_numbers.cl file. 
    Specifically, the code reads the file's content into a char array 
    called program_buffer, and then calls clCreateProgramWithSource.
    */
    program = clCreateProgramWithSource(ctx, 1, (const char**)&program_buffer,
                                        &program_size, &err);
    if(err < 0) {
        perror("Couldn't create the program");
        exit(1);
    }
    free(program_buffer);
    /* Build program 
    The fourth parameter accepts options that configure the compilation. 
    These are similar to the flags used by gcc. For example, you can 
    define a macro with the option -DMACRO=VALUE and turn off optimization 
    with -cl-opt-disable.
    */
    err = clBuildProgram(program, 0, NULL, "-DNUM_THREADS=16", NULL, NULL);
    if(err < 0) {
        /* Find size of log and print to std output */
        clGetProgramBuildInfo(program, dev, CL_PROGRAM_BUILD_LOG, 
                              0, NULL, &log_size);
        program_log = (char*) malloc(log_size + 1);
        program_log[log_size] = '\0';
        clGetProgramBuildInfo(program, dev, CL_PROGRAM_BUILD_LOG, 
                              log_size + 1, program_log, NULL);
        printf("%s\n", program_log);
        free(program_log);
        exit(1);
    }

    return program;
}

int test_fn() {
    int test = 42;
    return test;
}

int main(int argc, char** argv)
{
    int err;                            // error code returned from api calls
      
    float data[DATA_SIZE];              // original data set given to device
    float results[DATA_SIZE];           // results returned from device
    unsigned int correct;               // number of correct results returned
    uint count;
    size_t global;                      // global domain size for our calculation
    size_t local;                       // local domain size for our calculation

    cl_device_id device_id;             // compute device id 
    cl_context context;                 // compute context
    cl_command_queue commands;          // compute command queue
    cl_program program;                 // compute program
    cl_kernel kernel;                   // compute kernel
    cl_kernel data_kernel;              // data kernel

    cl_mem input;                       // device memory used for the input array
    cl_mem output;                      // device memory used for the output array
    cl_mem test_buffer;                 // device memory used for the output array
    uint temp[102400];

    /*
     * These are the arguments for each WASM function
     * 
     */

	cl_mem stack_u32;
	cl_mem stack_u64;
	cl_mem heap_u32;
	cl_mem heap_u64;
	cl_mem stack_frames;
    cl_mem sp;
	cl_mem sfp;
    cl_mem call_stack;
    cl_mem branch_value_stack_state;
    cl_mem loop_value_stack_state;
    cl_mem hypercall_num;
    cl_mem hypercall_continuation;
    cl_mem thread_ids;
    cl_mem entry;

    // the setup data
	uint *stack_frames_setup = calloc(STACK_SIZE_BYTES, sizeof(uint));
	ulong sp_setup = 0;
	ulong sfp_setup = 1;
    ulong entry_setup = 0;
    long hypercall_num_setup = -2;

    // The WASI sandbox, 1 instance per thread
	uvwasi_t uvwasi[WARP_SIZE];
	uvwasi_options_t init_options[WARP_SIZE];

    for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
        uvwasi_options_init(&init_options[warp_idx]);
        uvwasi_init(&uvwasi[warp_idx], &init_options[warp_idx]);
    }

	stack_frames_setup[sfp_setup - 1] = sp_setup;

    // Connect to a compute device
    //
    int gpu = 1;
    err = clGetDeviceIDs(NULL, gpu ? CL_DEVICE_TYPE_GPU : CL_DEVICE_TYPE_CPU, 1, &device_id, NULL);
    if (err != CL_SUCCESS)
    {
        printf("Error: Failed to create a device group!\n");
        return EXIT_FAILURE;
    }
    
    // Create a compute context 
    //
    context = clCreateContext(0, 1, &device_id, NULL, NULL, &err);
    if (!context)
    {
        printf("Error: Failed to create a compute context!\n");
        return EXIT_FAILURE;
    }

    // Create a command commands
    //
    commands = clCreateCommandQueue(context, device_id, 0, &err);
    if (!commands)
    {
        printf("Error: Failed to create a command commands!\n");
        return EXIT_FAILURE;
    }

    // Create the compute program from the source buffer
    //

    program = build_program(context, device_id, "test.cl");
    if (!program)
    {
        printf("Error: Failed to create compute program!\n");
        return EXIT_FAILURE;
    }

    /*
    size_t size;
    int status = clGetProgramInfo(program, CL_PROGRAM_BINARY_SIZES, sizeof(size_t), &size, NULL);
    unsigned char * binary = malloc(size);
    status = clGetProgramInfo(program, CL_PROGRAM_BINARIES, size, &binary, NULL);
    printf("status: %d\n", status);
    FILE * fpbin = fopen("/Users/sam/research/opencl-test/OpenCL-examples/Hello_World/test.txt", "wb" );
    if( fpbin == NULL )
    {
        fprintf( stdout, "Cannot create '%s'\n", "/Users/sam/research/opencl-test/OpenCL-examples/Hello_World/test.txt" );
    }
    else
    {
        fwrite(binary, 1, size, fpbin);
        fclose(fpbin );
    }
    */
    
    // Create the compute kernel in the program we wish to run
    //
    kernel = clCreateKernel(program, "wasm_entry", &err);
    if (!kernel || err != CL_SUCCESS)
    {
        printf("Error: Failed to create compute kernel!\n");
        exit(1);
    }

    data_kernel = clCreateKernel(program, "data_init", &err);
    if (!data_kernel || err != CL_SUCCESS)
    {
        printf("Error: Failed to create data kernel!\n");
        exit(1);
    }

    // Alloc buffers for all 16 programs
    stack_u32 = clCreateBuffer(context,  CL_MEM_READ_WRITE,  STACK_SIZE_BYTES * WARP_SIZE, NULL, NULL);
    stack_u64 = stack_u32;
    heap_u32 = clCreateBuffer(context,  CL_MEM_READ_WRITE, HEAP_SIZE_BYTES * WARP_SIZE, NULL, NULL);
    heap_u64 = heap_u32;
    stack_frames = clCreateBuffer(context,  CL_MEM_READ_WRITE,  STACK_SIZE_BYTES * WARP_SIZE, NULL, NULL);
    sp = clCreateBuffer(context,  CL_MEM_READ_WRITE, sizeof(unsigned long) * WARP_SIZE, NULL, NULL);
    sfp = clCreateBuffer(context,  CL_MEM_READ_WRITE,  STACK_SIZE_BYTES * WARP_SIZE, NULL, NULL);
    call_stack = clCreateBuffer(context,  CL_MEM_READ_WRITE,  1024 * WARP_SIZE, NULL, NULL);
    branch_value_stack_state = clCreateBuffer(context,  CL_MEM_READ_WRITE,  4096 * WARP_SIZE, NULL, NULL);
    loop_value_stack_state = clCreateBuffer(context,  CL_MEM_READ_WRITE,  4096 * WARP_SIZE, NULL, NULL);

    hypercall_num = clCreateBuffer(context,  CL_MEM_READ_WRITE,  sizeof(ulong) * WARP_SIZE, NULL, NULL);
    hypercall_continuation = clCreateBuffer(context,  CL_MEM_READ_WRITE,  sizeof(ulong) * WARP_SIZE, NULL, NULL);

    entry = clCreateBuffer(context,  CL_MEM_READ_WRITE,  sizeof(ulong) * WARP_SIZE, NULL, NULL);
    
    thread_ids = clCreateBuffer(context,  CL_MEM_READ_WRITE,  sizeof(ushort) * WARP_SIZE, NULL, NULL);

    if (!stack_u32 || !heap_u32 || !stack_frames || 
        !sp || !sfp || !call_stack || !loop_value_stack_state 
        || !branch_value_stack_state || !entry || !hypercall_num || !hypercall_continuation || !thread_ids) {
        printf("Error: Failed to allocate device memory!\n");
        exit(1);
    }


    // Write our data set into the input array in device memory 
    for (uint count = 0; count < WARP_SIZE; count++) {
        printf("test\n");

        // for each VM we have to prepare it for launch by setting up the stack frame
        // In the future: if we want to pass parameters it has to be done on the stack

        // set the stack pointer: sp = 0
        err = clEnqueueWriteBuffer(commands, sp, CL_TRUE, count * sizeof(ulong), sizeof(ulong), &sp_setup, 0, NULL, NULL);
        printf("err:%d\n", err);
        // set the stack frame pointer: sfp = 1
        err |= clEnqueueWriteBuffer(commands, sfp, CL_TRUE, count * STACK_SIZE_BYTES, sizeof(ulong), &sfp_setup, 0, NULL, NULL);
        printf("err:%d\n", err);
        // set the stack frame: stack_frames[sfp - 1] = sp;
        err |= clEnqueueWriteBuffer(commands, stack_frames, CL_TRUE, count * STACK_SIZE_BYTES, STACK_SIZE_BYTES, stack_frames_setup, 0, NULL, NULL);
        printf("err:%d\n", err);
        // set the wasm function entry point
        err |= clEnqueueWriteBuffer(commands, entry, CL_TRUE, count * sizeof(uint), sizeof(uint), &entry_setup, 0, NULL, NULL);
        printf("err:%d\n", err);
        // set the default hypercall_number to -2
        err |= clEnqueueWriteBuffer(commands, hypercall_num, CL_TRUE, count * sizeof(uint), sizeof(uint), &hypercall_num_setup, 0, NULL, NULL);
        printf("err:%d\n", err);

        if (err != CL_SUCCESS)
        {
            printf("Error: Failed to write buffers during setup: %d\n", err);
            exit(1);
        }
    }

    // Set the arguments to our compute kernel
    //
    err = 0;
    err  = clSetKernelArg(kernel, 0, sizeof(cl_mem), &stack_u32);
    err |= clSetKernelArg(kernel, 1, sizeof(cl_mem), &stack_u64);
    err |= clSetKernelArg(kernel, 2, sizeof(cl_mem), &heap_u32);
    err |= clSetKernelArg(kernel, 3, sizeof(cl_mem), &heap_u64);
    err  = clSetKernelArg(kernel, 4, sizeof(cl_mem), &stack_frames);
    err |= clSetKernelArg(kernel, 5, sizeof(cl_mem), &sp);
    err |= clSetKernelArg(kernel, 6, sizeof(cl_mem), &sfp);
    err |= clSetKernelArg(kernel, 7, sizeof(cl_mem), &call_stack);
    err |= clSetKernelArg(kernel, 8, sizeof(cl_mem), &branch_value_stack_state);
    err |= clSetKernelArg(kernel, 9, sizeof(cl_mem), &loop_value_stack_state);
    err |= clSetKernelArg(kernel, 10, sizeof(cl_mem), &hypercall_num);
    err |= clSetKernelArg(kernel, 11, sizeof(cl_mem), &hypercall_continuation);
    err |= clSetKernelArg(kernel, 12, sizeof(cl_mem), &entry);

    // set up the arg for our data kernel
    
    err |= clSetKernelArg(data_kernel, 0, sizeof(cl_mem), &heap_u32);


    if (err != CL_SUCCESS)
    {
        printf("Error: Failed to set kernel arguments! %d\n", err);
        exit(1);
    }

    // Get the maximum work group size for executing the kernel on the device
    //
    err = clGetKernelWorkGroupInfo(kernel, device_id, CL_KERNEL_WORK_GROUP_SIZE, sizeof(local), &local, NULL);
    if (err != CL_SUCCESS)
    {
        printf("Error: Failed to retrieve kernel work group info! %d\n", err);
        exit(1);
    }

    global = WARP_SIZE;
    local = 16;

    ulong temp_sp[WARP_SIZE];
    ulong temp_hypercall_num[WARP_SIZE];
    ulong temp_entry_point[WARP_SIZE];
    ulong entry_point_exited = -1;

    cl_event event_timer;
    ulong starttime;
    ulong endtime;
    clFinish(commands);

    // TODO: for each kernel, launch a secondary setup kernel that initializes all 
    // of the (data ...) sections (much better to do this on the GPU than on the CPU)
    err = clEnqueueNDRangeKernel(commands, data_kernel, 1, NULL, &global, &local, 0, NULL, NULL);
    if (err) {
        printf("Error: Failed to execute kernel!: %d\n", err);
        return EXIT_FAILURE;
    }
    clFinish(commands);
    printf("data init complete\n");

    for (int test = 0; test < 1; test++) {
        clock_t begin = clock();
        while (1) {
            printf("launching kernel\n");
            clGetEventProfilingInfo(event_timer, CL_PROFILING_COMMAND_START, sizeof(ulong), &starttime, NULL);
            clGetEventProfilingInfo(event_timer, CL_PROFILING_COMMAND_END, sizeof(ulong), &endtime, NULL);

            err = clEnqueueNDRangeKernel(commands, kernel, 1, NULL, &global, &local, 0, NULL, &event_timer);
            if (err) {
                printf("Error: Failed to execute kernel!: %d\n", err);
                return EXIT_FAILURE;
            }
            // Wait for the command commands to get serviced before reading back results
            clFinish(commands);
            printf("kernel finished\n");
            printf("elapsed time: %f\n", (float)(endtime - starttime));

            // read SP to see if kernel is done
            err = clEnqueueReadBuffer(commands, sp, CL_TRUE, 0, sizeof(ulong) * WARP_SIZE, &temp_sp, 0, NULL, NULL);  
            if (err != CL_SUCCESS) {
                printf("Unable to read stack pointer from GPU kernels...\n");
                exit(-1);
            }

            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                //printf("warp_idx: %d\tsp: %d\n", warp_idx, temp_sp[warp_idx]);
            }

            // if all (sp) == 0, exit
            bool exit = true;
            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                exit = ((temp_sp[warp_idx] == 0) & exit);
            }

            // if all entry_point == -1, also exit

            printf("exit: %d\n", exit);
            if (exit) {
                printf("all procs exited\n");
                break;
            }

            printf("we have procs left to run...\n");
            // else, block off the threads that are done (sp == 0)
            ulong entry_point_blocked_off = -1;
            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                if (temp_sp[warp_idx] == 0) {
                    // block off the thread by setting the entry_point to -1
                    err = clEnqueueWriteBuffer(commands, entry, CL_TRUE, warp_idx * sizeof(ulong), sizeof(ulong), &entry_point_blocked_off, 0, NULL, NULL);
                }
            }

            // now for the remaining threads, dispatch appropriate hypercalls
            // first read back all the hypercall nums
            err = clEnqueueReadBuffer(commands, hypercall_num, CL_TRUE, 0, sizeof(ulong) * WARP_SIZE, &temp_hypercall_num, 0, NULL, NULL);  
            if (err != CL_SUCCESS) {
                printf("Unable to read stack pointer from GPU kernels...\n");
                return -1;
            }

            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                switch (temp_hypercall_num[warp_idx])
                {
                    case 0:
                        printf("fd_write\n");
                        vmm_fd_write(&uvwasi[warp_idx], stack_u32, temp_sp[warp_idx], heap_u32,
                                     warp_idx, commands, STACK_SIZE_BYTES, HEAP_SIZE_BYTES);
                        break;
                    case 1:
                        // TODO, set entry_point to -1
                        printf("proc_exit\n");
                        err = clEnqueueWriteBuffer(commands, entry, CL_TRUE,
                                                   warp_idx * sizeof(ulong), sizeof(ulong),
                                                   &entry_point_blocked_off, 0, NULL, NULL);
                        break;
                }
            }

            // okay, now do a second check to see if all the procs have exited,
            // this is to handle the proc_exit hypercall
            err = clEnqueueReadBuffer(commands, entry, CL_TRUE, 0, sizeof(ulong) * WARP_SIZE, &temp_entry_point, 0, NULL, NULL);  
            if (err != CL_SUCCESS) {
                printf("Unable to read entry point from GPU kernels...\n");
                return -1;
            }

            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                //printf("warp_idx: %d\tentry_point: %d\n", warp_idx, temp_entry_point[warp_idx]);
            }

            // if all (entry_point) == -1, exit
            bool entry_point_exit = true;
            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                entry_point_exit = ((temp_entry_point[warp_idx] == -1) & entry_point_exit);
            }

            printf("entry pt exit: %d\n", entry_point_exit);
            if (entry_point_exit) {
                break;
            }

            // after handling the hypercalls, reset hypercall_number = -1;
            // we can just blindly write to all threads, since they would all be done at this point
            long hypercall_num_reentry = - 1;
            for (uint warp_idx = 0; warp_idx < WARP_SIZE; warp_idx++) {
                if (temp_sp[warp_idx] != 0) {
                    // block off the thread by setting the hypercall_num to -1
                    err = clEnqueueWriteBuffer(commands, hypercall_num, CL_TRUE, warp_idx * sizeof(ulong), sizeof(ulong), &hypercall_num_reentry, 0, NULL, NULL);
                }
            }

            // now continue and resume the continuation...
        }
        clock_t end = clock();
        double time_spent_gpu = (double)(end - begin) / CLOCKS_PER_SEC;
        printf("GPU: %f\n", time_spent_gpu);
    }

    // debug, read the function return values onto the stack



    uchar *stack_debug = malloc(STACK_SIZE_BYTES * WARP_SIZE);
    clEnqueueReadBuffer(commands, stack_u32, CL_TRUE, 0, STACK_SIZE_BYTES * WARP_SIZE, stack_debug, 0, NULL, NULL);  
    
    // no interleave
    /*
    for (uint idx = 0; idx < WARP_SIZE; idx++) {
        printf("%x, ", stack_debug[(1024 * 16 * idx) * 4]);
    }
    printf("\n");
    */

    // for printing w/interleave
    for (uint idx = 0; idx < WARP_SIZE; idx++) {
        printf("%x, ", stack_debug[idx * WARP_SIZE]);
    }
    printf("\n");

    // Shutdown and cleanup
    //
    //clReleaseMemObject(input);
    //clReleaseMemObject(output);
    clReleaseProgram(program);
    clReleaseKernel(kernel);
    clReleaseCommandQueue(commands);
    clReleaseContext(context);

    return 0;
}

