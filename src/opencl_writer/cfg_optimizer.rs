use crate::opencl_writer;

/*
 * When compiling partitioned applications, if we do not perform any optimizations
 * each GPU kernel contains a single function, and function-level divergence is handled by the VMM.
 *
 * This is done to avoid exceedingly large compile-times and memory usage by the GPU JIT compiler.
 * 
 * However, partitioning to N=1 functions per kernel results in high device queueing times & VMM overhead.
 * It is usually better to let the GPU handle divergence (if possible).
 *
 * In order to compromise these two tradeoffs (longer compiles vs more efficient execution),
 * we employ a more complex partitioning proceedure that groups functions that call 
 * each other into the same OpenCL kernel.
 */

pub fn form_partitions(num_partitions: u32) -> () {
    // First, if N=1, return the standard partitioning with no groupings

    /*
     * For N>=2:
     * 1) Create a set of all functions in the program (global BTreeSet G)
     * 2) Pop a function (F) out of the global set of functions
     * 3) Run analysis pass on the function
     *    Keep 2 queues (one for high priority calls (calls inside loops), the rest in the other)
     * 4) Using the two queues, prioritize merging from the priority queue first, then merge the rest
     * 5) Once a grouping is formed, remove the selected functions from G
     * 6) Go to 2 and repeat until the global set G is empty
     */
}