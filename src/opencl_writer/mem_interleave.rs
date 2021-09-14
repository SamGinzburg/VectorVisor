use crate::opencl_writer;

/*
 * All reads and writes are abstracted through these calls
 * We want to support no interleave, as well as 1 byte, 4 byte, and 8 byte interleaves
 * 
 * Addressing model:
 *  For no interleave, the linear memory is divided into N regions, where N=NUM_THREADS
 * 
 *  Virtual Address = address, this calc is easy because we simply set the heap/stack pointers
 *  at the start of the kernel call, so no pointer math has to be done!
 * 
 *  For a 1 byte interleave, the linear memory is interleaved, with corresponding offsets
 *  mapped to adjacent bytes. For example: if you had 4 threads, that each write 0x1 to
 *  to an address of 0, the corresponding memory would look like:
 *
 *  [0x1, 0x1, 0x1, 0x1], with each of the writes sharing an offset of 0
 *  [T0 (byte 0), T1 (byte 0), T2 (byte 0), T3 (byte 0), T0 (byte 1), ...]
 * 
 * For a X byte interleave, there are X adjacent bytes per 'cell' of memory. For example, 
 * with 4 threads with an interleave of 8 that you write 0x1 to an address of 0, the corresponding memory would look like:
 * 
 *  [0x1, 0x1, 0x1, 0x1], with each of the writes sharing an offset of 0
 *  [T0 (byte 0), T0 (byte 1), T0 (byte 2), T0 (byte 3), T0 (byte 4), T0 (byte 5), T0 (byte 6), T0 (byte 7),
 *   ...
 *   T3 (byte 0), T3 (byte 1), T3 (byte 2), T3 (byte 3), T3 (byte 4), T3 (byte 5), T3 (byte 6), T3 (byte 7)]
 * 
 *  The offset calc is:
 *  offset = address - mem_start_addr
 *  Virtual Address = ((offset/interleave) * (NUM_THREADS/interleave)) + (warp_idx*interleave) + mem_start_addr
 *  
 *  ex: if you are in thread 0, and you write to 0, and then 1
 *  the physical addresses are first 0, and then address+NUM_THREADS, with each
 *  subsequent byte being 1 stride of NUM_THREADS away
 * 
 *  We expect NUM_THREADS to be defined at compile time with the macro NUM_THREADS
 * 
 *  We also have to split multi-cell reads into multiple calls, in little-endian format
 * 
 */

// fast_read/write functions - are for private use only
fn emit_fast_read_u8(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("fast_read_u8({}, {}, {})", addr, mem_start, warp_id)
}

fn emit_write_u16_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            // write the lower byte first
            result += &format!("\t{}\n",
                                "write_u8(addr, mem_start, value & 0xFF, warp_id);");
            // now write the upper byte
            result += &format!("\t{}\n",
                                "write_u8((ulong)(((char*)addr)+1), mem_start, (value >> 8) & 0xFF, warp_id);");
        },
        1 => {
            // Compute the address first
            result += &format!("\t{}\n",
                            "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");

            match mexec {
                /*
                2 => {
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*read_idx)), mem_start, (value >> (8*read_idx)) & 0xFF, warp_id);");
                },
                4 => {
                    result += &format!("\n\tread_idx = read_idx / 2;\n"); // we have to duplicate work
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*read_idx)), mem_start, (value >> (8*read_idx)) & 0xFF, warp_id);");
                },
                */
                _ => {
                    // write the lower byte first
                    result += &format!("\t{}\n",
                                        "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);");
                    // now write the upper byte
                    result += &format!("\t{}\n",
                                        "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");

                }
            };
        },
        8 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "write_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "if (IS_ALIGNED_POW2((ulong)write_addr, 2)) {");
                result += &format!("\t\t{}\n",
                            "*((global ushort*)((global uchar*)write_addr)) = value;");
                result += &format!("\t{}\n",
                            "} else {");
                result += &format!("\t\t{}\n",
                            "write_u16(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n",
                            "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "*((global ushort*)((global uchar*)write_addr+cell_offset)) = value;");
            } else {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "scratch_space[thread_idx].lo = (ulong)*((global ulong*)write_addr);");
                result += &format!("\t{}\n",
                            "scratch_space[thread_idx].hi = (ulong)*((global ulong*)write_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                            "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                            "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n",
                            "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n",
                            "*((global ulong*)write_addr) = scratch_space[thread_idx].lo;");
                result += &format!("\t{}\n",
                            "*((global ulong*)write_addr+(NUM_THREADS)) = scratch_space[thread_idx].hi;");
            }
        },
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_write_u32_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!("\t{}\n",
                        "write_u8(addr, mem_start, value & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+1), mem_start, (value >> 8) & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 16) & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+3), mem_start, (value >> 24) & 0xFF, warp_id);");
        },
        1 => {
            // Compute the address first
            result += &format!("\t{}\n",
                            "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");
            match mexec {
                /*
                2 => {
                    result += &format!("\n\tread_idx = read_idx * 2;\n");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*read_idx)), mem_start, (value >> 8*read_idx) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx+1))), mem_start, (value >> 8*(read_idx+1)) & 0xFF, warp_id);");
                },
                4 => {
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*read_idx)), mem_start, (value >> (8*read_idx)) & 0xFF, warp_id);");
                },
                */
                _ => {                    
                    // write the bytes lowest to highest
                    result += &format!("\t{}\n",
                                "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*2), mem_start, (value >> 16) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*3), mem_start, (value >> 24) & 0xFF, warp_id);");
                }
            }
        },
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "write_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "if (IS_ALIGNED_POW2((ulong)write_addr, 4)) {");
                result += &format!("\t\t{}\n",
                            "*((global uint*)((global uchar*)write_addr)) = value;");
                result += &format!("\t{}\n",
                            "} else {");
                result += &format!("\t\t{}\n",
                            "write_u32(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n",
                            "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "*((global uint*)((global uchar*)write_addr+cell_offset)) = value;");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].lo = (ulong)*((global ulong*)write_addr);");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].hi = (ulong)*((global ulong*)write_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                                "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                                "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+2] = (value >> 16) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+3] = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n",
                                "*((global ulong*)write_addr) = scratch_space[thread_idx].lo;");
                result += &format!("\t{}\n",
                                "*((global ulong*)write_addr+(NUM_THREADS)) = scratch_space[thread_idx].hi;");
            }
        },
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_write_u64_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!("\t{}\n",
                        "write_u8(addr, mem_start, value & 0xFF, warp_id);");
            result += &format!("\t{}",
                        "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 8) & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 16) & 0xFF, warp_id);");
            result += &format!("\t{}",
                        "write_u8((ulong)(((char*)addr)+3), mem_start, (value >> 24) & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+4), mem_start, (value >> 32) & 0xFF, warp_id);");
            result += &format!("\t{}",
                        "write_u8((ulong)(((char*)addr)+5), mem_start, (value >> 40) & 0xFF, warp_id);");
            result += &format!("\t{}\n",
                        "write_u8((ulong)(((char*)addr)+6), mem_start, (value >> 48) & 0xFF, warp_id);");
            result += &format!("\t{}",
                        "write_u8((ulong)(((char*)addr)+7), mem_start, (value >> 56) & 0xFF, warp_id);");
        },
        1 => {
            result += &format!("\t{}\n",
                            "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");

            match mexec {
                /*
                2 => {
                    result += &format!("\n\tread_idx = read_idx * 4;\n");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*read_idx)), mem_start, (value >> 8*(read_idx)) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx+1))), mem_start, (value >> 8*(read_idx+1)) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx+2))), mem_start, (value >> 8*(read_idx+2)) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx+3))), mem_start, (value >> 8*(read_idx+3)) & 0xFF, warp_id);");
                },
                4 => {
                    result += &format!("\n\tread_idx = read_idx * 2;\n");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx))), mem_start, (value >> 8*(read_idx)) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                    "fast_write_u8((ulong)(((char*)addr)+(NUM_THREADS*(read_idx+1))), mem_start, (value >> 8*(read_idx+1)) & 0xFF, warp_id);");
                },
                */
                _ => {
                    // write the bytes lowest to highest
                    result += &format!("\t{}\n",
                                "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);");
                    result += &format!("\t{}",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*2), mem_start, (value >> 16) & 0xFF, warp_id);");
                    result += &format!("\t{}",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*3), mem_start, (value >> 24) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*4), mem_start, (value >> 32) & 0xFF, warp_id);");
                    result += &format!("\t{}",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*5), mem_start, (value >> 40) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*6), mem_start, (value >> 48) & 0xFF, warp_id);");
                    result += &format!("\t{}",
                                "fast_write_u8((ulong)(((char*)addr)+NUM_THREADS*7), mem_start, (value >> 56) & 0xFF, warp_id);");
                }
            }
        },
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "write_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "if (IS_ALIGNED_POW2((ulong)write_addr, 8)) {");
                result += &format!("\t\t{}\n",
                            "*((global ulong*)((global uchar*)write_addr)) = value;");
                result += &format!("\t{}\n",
                            "} else {");
                result += &format!("\t\t{}\n",
                            "write_u64(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n",
                            "}");

            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                            "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                            "*((global ulong*)((global uchar*)write_addr+cell_offset)) = value;");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].lo = (ulong)*((global ulong*)write_addr);");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].hi = (ulong)*((global ulong*)write_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                                "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                                "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+2] = (value >> 16) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+3] = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+4] = (value >> 32) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+5] = (value >> 40) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+6] = (value >> 48) & 0xFF;");
                result += &format!("\t{}\n",
                                "combined[cell_offset+7] = (value >> 56) & 0xFF;");
                result += &format!("\t{}\n",
                                "*((global ulong*)write_addr) = scratch_space[thread_idx].lo;");
                result += &format!("\t{}\n",
                                "*((global ulong*)write_addr+(NUM_THREADS)) = scratch_space[thread_idx].hi;");
            }
        },

        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u16_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");
    match interleave {
        0 => {
            result += &format!("\t{}\n",
                                "ushort temp = 0;");
            result += &format!("\t{}\n",
                                "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                                "temp = temp << 8;");
            result += &format!("\t{}\n",
                                "temp += read_u8(addr, mem_start, warp_id);");
            result += &format!("\t{}",
                                "return temp;");
        },
        1 => {
            // use a local variable to store the result as we perform the reads
            // we have to read in the reverse order!!! (high bits then low bits)
            result += &format!("\t{}\n",
                               "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");
            result += &format!("\t{}\n",
                                "ushort temp = 0;");

            match mexec {
                /*
                2 => {
                    result += &format!("\t{}\n",
                                       "local ushort *read_temp = (local ushort*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\tread_temp_uchar[(thread_idx * 2) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*{})), mem_start, warp_id);\n",
                                       "read_idx", "read_idx");

                    result += &format!("\t{}\n",
                                       "temp = (ushort)read_temp[thread_idx];");
                },
                4 => {
                    result += &format!("\t{}\n",
                                    "local ushort *read_temp = (local ushort*)scratch_space;");
                    result += &format!("\t{}\n",
                                    "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\t{}\n",
                                    "if (read_idx < 2) {{");
                    result += &format!("\t\tread_temp_uchar[(thread_idx * 2) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*{})), mem_start, warp_id);\n",
                                    "read_idx", "read_idx");
                    result += &format!("\t{}\n",
                                    "}}");
                    result += &format!("\t{}\n",
                                    "temp = (ushort)read_temp[thread_idx];");
                },
                */
                _ => {
                    result += &format!("\t{}\n",
                                        "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n",
                                        "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                        "temp += fast_read_u8(addr, mem_start, warp_id);");
                },
            }

            result += &format!("\t{}",
                                "return temp;");
        },
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "read_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "if (IS_ALIGNED_POW2((ulong)read_addr, 2)) {");
                result += &format!("\t\t{}\n",
                                "return *((global ushort*)((global uchar*)read_addr));");
                result += &format!("\t{}\n",
                                "}");
                result += &format!("\t\t{}\n",
                                "return read_u16(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "return *((global ushort*)((global uchar*)read_addr+cell_offset));");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "ushort tmp_vec = 0;");
                result += &format!("\t{}\n",
                                "uchar *tmp = &tmp_vec;");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].lo = (ulong)*((global ulong*)read_addr);");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].hi = (ulong)*((global ulong*)read_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                                "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                                "tmp[0] = combined[cell_offset];");
                result += &format!("\t{}\n",
                                "tmp[1] = combined[cell_offset+1];");
                result += &format!("\t{}\n",
                                "return tmp_vec;");
            }
        },
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u32_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");
    match interleave {
        0 => {
            result += &format!("\t{}\n",
                                "uint temp = 0;");
            result += &format!("\t{}\n",
                                "temp += read_u8((ulong)(((char*)addr)+3), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                                "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+2), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                                "temp = temp << 8;");
            result += &format!("\t{}\n",
                                "temp += read_u8((ulong)(((char*)addr)), mem_start, warp_id);");
            result += &format!("\t{}",
                                "return temp;");
        },
        1 => {
            result += &format!("\t{}\n",
                               "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");

            // use a local variable to store the result as we perform the reads
            result += &format!("\t{}\n",
                                "uint temp = 0;");
            match mexec {
                /*
                2 => {
                    result += &format!("\t{}\n",
                                       "local uint *read_temp = (local uint*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "read_idx = read_idx * 2;");
                    result += &format!("\tread_temp_uchar[(thread_idx * 4) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*{})), mem_start, warp_id);\n",
                                       "read_idx", "read_idx");
                    result += &format!("\tread_temp_uchar[(thread_idx * 4) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx+1", "read_idx+1");
                    result += &format!("\t{}\n",
                                       "temp = (uint)read_temp[thread_idx];");
                },
                4 => {
                    result += &format!("\t{}\n",
                                       "local uint *read_temp = (local uint*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\tread_temp_uchar[(thread_idx * 4) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*{})), mem_start, warp_id);\n",
                                       "read_idx", "read_idx");
                    result += &format!("\t{}\n",
                                       "temp = (uint)read_temp[thread_idx];");
                },
                */
                _ => {
                    result += &format!("\t{}\n",
                                "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*3), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n",
                                "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*2), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n",
                                "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                "temp += fast_read_u8((ulong)(((char*)addr)), mem_start, warp_id);");
                }
            }
            
            result += &format!("\t{}",
                                "return temp;");
        },
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "read_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "if (IS_ALIGNED_POW2((ulong)read_addr, 4)) {");
                result += &format!("\t\t{}\n",
                                "return *((global uint*)((global uchar*)read_addr));");
                result += &format!("\t{}\n",
                                "}");
                result += &format!("\t\t{}\n",
                                "return read_u32(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "return *((global uint*)((global uchar*)read_addr+cell_offset));");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "uint tmp_vec = 0;");
                result += &format!("\t{}\n",
                                "uchar *tmp = &tmp_vec;");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].lo = (ulong)*((global ulong*)read_addr);");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].hi = (ulong)*((global ulong*)read_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                                "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                                "tmp[0] = combined[cell_offset];");
                result += &format!("\t{}\n",
                                "tmp[1] = combined[cell_offset+1];");
                result += &format!("\t{}\n",
                                "tmp[2] = combined[cell_offset+2];");
                result += &format!("\t{}\n",
                                "tmp[3] = combined[cell_offset+3];");
                result += &format!("\t{}\n",
                                "return tmp_vec;");
            }
        },
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u64_body(interleave: u32, local_work_group: usize, mexec: usize, emit_aligned: bool, emit_checked: bool) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!("\t{}\n",
                                "ulong temp = 0;");
            result += &format!("\t{}\n",
                                "temp += read_u8((ulong)(((char*)addr)+7), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+6), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+5), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+4), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+3), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+2), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");
            result += &format!("\t{}\n",
                               "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);");
            result += &format!("\t{}\n",
                               "temp = temp << 8;");   
            result += &format!("\t{}\n",
                                "temp += read_u8((ulong)(((char*)addr)), mem_start, warp_id);");
            result += &format!("\t{}",
                                "return temp;");
        },
        1 => {
            result += &format!("\t{}\n",
                               "addr = (ulong)((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));");

            // use a local variable to store the result as we perform the reads
            result += &format!("\t{}\n",
                                "ulong temp = 0;");

            match mexec {
                /*
                2 => {
                    result += &format!("\t{}\n",
                                       "local ulong *read_temp = (local ulong*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "read_idx = read_idx * 4;");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx", "read_idx");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx+1", "read_idx+1");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx+2", "read_idx+2");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx+3", "read_idx+3");
                    result += &format!("\t{}\n",
                                       "temp = (ulong)read_temp[thread_idx];");
                },
                4 => {
                    result += &format!("\t{}\n",
                                       "local ulong *read_temp = (local ulong*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "local uchar *read_temp_uchar = (local uchar*)scratch_space;");
                    result += &format!("\t{}\n",
                                       "read_idx = read_idx * 2;");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx", "read_idx");
                    result += &format!("\tread_temp_uchar[(thread_idx * 8) + {}] = fast_read_u8((ulong)(((char*)addr)+(NUM_THREADS*({}))), mem_start, warp_id);\n",
                                       "read_idx+1", "read_idx+1");
                    result += &format!("\t{}\n",
                                       "temp = (ulong)read_temp[thread_idx];");
                },
                */
                _ => {
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*7), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*6), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*5), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*4), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*3), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS*2), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)+NUM_THREADS), mem_start, warp_id);");
                    result += &format!("\t{}\n",
                                    "temp = temp << 8;");   
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((char*)addr)), mem_start, warp_id);");
                }
            }

            result += &format!("\t{}",
                                "return temp;");
        },
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "read_addr += (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "if (IS_ALIGNED_POW2((ulong)read_addr, 8)) {");
                result += &format!("\t\t{}\n",
                                "return *((global ulong*)((global uchar*)read_addr));");
                result += &format!("\t{}\n",
                                "}");
                result += &format!("\t\t{}\n",
                                "return read_u64(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "return *((global ulong*)((global uchar*)read_addr+cell_offset));");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!("\t{}\n",
                                "ulong cell_offset = (addr-mem_start) % 8;");
                result += &format!("\t{}\n",
                                "ulong tmp_vec = 0;");
                result += &format!("\t{}\n",
                                "uchar *tmp = &tmp_vec;");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].lo = (ulong)*((global ulong*)read_addr);");
                result += &format!("\t{}\n",
                                "scratch_space[thread_idx].hi = (ulong)*((global ulong*)read_addr+(NUM_THREADS));");
                result += &format!("\t{}\n",
                                "local uchar *combined = &scratch_space[thread_idx];");
                result += &format!("\t{}\n",
                                "tmp[0] = combined[cell_offset];");
                result += &format!("\t{}\n",
                                "tmp[1] = combined[cell_offset+1];");
                result += &format!("\t{}\n",
                                "tmp[2] = combined[cell_offset+2];");
                result += &format!("\t{}\n",
                                "tmp[3] = combined[cell_offset+3];");
                result += &format!("\t{}\n",
                                "tmp[4] = combined[cell_offset+4];");
                result += &format!("\t{}\n",
                                "tmp[5] = combined[cell_offset+5];");
                result += &format!("\t{}\n",
                                "tmp[6] = combined[cell_offset+6];");
                result += &format!("\t{}\n",
                                "tmp[7] = combined[cell_offset+7];");
                result += &format!("\t{}\n",
                                "return tmp_vec;");
            }
        },
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}


pub fn generate_read_write_calls(_writer: &opencl_writer::OpenCLCWriter, interleave: u32, local_work_group: usize, mexec: usize, _debug: bool) -> String {
    let mut result = String::from("");

    // fast_write_u8 is used for writes greater than 1 byte to reduce computation
    result += &format!("\n{}\n",
                        "inline void fast_write_u8(ulong addr, ulong mem_start, uchar value, uint warp_id) {");

    match interleave {
        0 | 1 | 8 => {
            result += &format!("\t{}",
                                "*((global uchar*)addr) = value;");
        },
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");


    result += &format!("\n{}\n",
                        "inline void write_u8(ulong addr, ulong mem_start, uchar value, uint warp_id, uint read_idx) {");

    match interleave {
        0 => {
            result += &format!("\t{}\n",
                                "*((global uchar*)addr) = value;");
        },
        1 => {
            result += &format!("\t{}\n",
                                "*((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start)) = value;")
        },
        8 => {
            // determine which cell to read
            result += &format!("\t{}\n",
                                "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
            result += &format!("\t{}\n",
                               "ulong cell_offset = (addr-mem_start) % 8;");
            result += &format!("\t{}\n",
                               "*(global uchar*)(write_addr + cell_offset) = value;")
        },
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) void write_u16(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u16_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u16_aligned(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u16_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u16_aligned_checked(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u16_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) void write_u32(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u32_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u32_aligned(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u32_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u32_aligned_checked(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u32_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) void write_u64(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u64_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u64_aligned(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u64_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u64_aligned_checked(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_write_u64_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}\n",
                        "}");

    // the read functions
    
    result += &format!("\n{}\n",
                        "inline uchar fast_read_u8(ulong addr, ulong mem_start, uint warp_id) {");
    match interleave {
        0 | 1 | 8 => {
            result += &format!("\t{}",
                                "return *((global uchar*)addr);");
        },
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline uchar read_u8(ulong addr, ulong mem_start, uint warp_id, uint read_idx) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "return *((global uchar*)addr);");
        },
        1 => {
            result += &format!("\t{}",
                                "return *((global uchar*)((addr-mem_start)*NUM_THREADS + warp_id + mem_start));");
        },
        8 => {
            result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
            result += &format!("\t{}\n",
                               "ulong cell_offset = (addr-mem_start) % 8;");
            result += &format!("\t{}\n",
                               "return *(read_addr + cell_offset);")
        },
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) ushort read_u16(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u16_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline ushort read_u16_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u16_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline ushort read_u16_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u16_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) uint read_u32(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u32_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline uint read_u32_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u32_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline uint read_u32_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u32_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "__attribute__((noinline)) ulong read_u64(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u64_body(interleave, local_work_group, mexec, false, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline ulong read_u64_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u64_body(interleave, local_work_group, mexec, true, false));
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline ulong read_u64_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("{}", emit_read_u64_body(interleave, local_work_group, mexec, true, true));
    result += &format!("\n{}\n",
                        "}");

    // emit a memcpy function as well, for utility purposes
    result += &format!("\n{}\n",
        "void ___private_memcpy(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx) {");

    result += &format!("\t{}\n",
                       "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

    result += &format!("\t{};\n",
                       emit_write_u8("(ulong)(dst+idx)", "(ulong)(mem_start_dst)",
                       &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id"), "warp_id"));

    result += &format!("\t{}\n",
                       "}");
    
    result += &format!("\n{}\n",
                       "}");


    // emit a special de-interleave memcpy, that reads interleaved memory, and writes to linear
    // memory

    result += &format!("\n{}\n",
        "void ___private_memcpy_gpu2cpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("\t{}\n",
                       "char *dst_tmp = (char*)(dst);");

    match interleave {
        0 => {
            result += &format!("\t{}\n",
                               "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

            result += &format!("\t\t{} = {};\n",
                               "*dst_tmp++",
                               &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id"));

            result += &format!("\t{}\n",
                               "}");
        },
        1 => {
            result += &format!("\t{}\n",
                               "ulong addr = (ulong)((global uchar*)(((src)-(ulong)(mem_start_src))*(NUM_THREADS) + warp_id + (ulong)mem_start_src));");

            result += &format!("\t{}\n",
                               "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

            result += &format!("\t\t{} = {};\n",
                               "*dst_tmp++",
                               &emit_fast_read_u8("(ulong)(addr+idx*NUM_THREADS)", "(ulong)(mem_start_src)", "warp_id"));

            result += &format!("\t{}\n",
                               "}");
        },
        8 => {
            result += &format!("\t{}\n",
                               "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

            result += &format!("\t\t{} = {};\n",
                               "*dst_tmp++",
                               &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id"));
            result += &format!("\t{}\n",
                               "}");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    
    result += &format!("\n{}\n",
                       "}");   

    // emit another de-interleave memcpy, that reads linear memory and writes to interleaved
    // memory
    result += &format!("\n{}\n",
        "void ___private_memcpy_cpu2gpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx) {");
    result += &format!("\t{}\n",
                       "char *src_tmp = (char*)(src);");
 
    result += &format!("\t{}\n",
                       "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

    result += &format!("\t{};\n",
                       emit_write_u8("(ulong)(dst+idx)", "(ulong)(mem_start_dst)",
                       "*src_tmp++", "warp_id"));

    result += &format!("\t{}\n",
                       "}");
    
    result += &format!("\n{}\n",
                       "}");   

    result += &format!("\n{}\n",
        "inline void * ___private_memcpy_nonmmu(void *dest, void *src, size_t len) {");
    result += &format!("\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
                        "char *d = dest;",
                        "char *s = src;",
                        "while (len--)",
                        "  *d++ = *s++;",
                        "return dest;");
    result += &format!("}}\n");


    // Emit helper functions for saving/restoring local_cache
    /*
    result += &format!("\n{}\n",
        "inline void * save_local_cache(uchar *local_cache, size_t len, ulong addr, ulong mem_start, uint warp_id) {");
    result += &format!("\t{}\n",
                        "for (uint idx = 0; idx < len; idx++) {");
                         result += &format!("\t\t{};\n",
                                            emit_write_u8("addr+idx", "mem_start", "local_cache[idx]", "warp_id"));

    result += &format!("\t}}\n");
    result += &format!("}}\n");

    result += &format!("\n{}\n",
        "inline void * restore_local_cache(uchar *local_cache, size_t len, ulong addr, ulong mem_start, uint warp_id) {");
    result += &format!("\t{}\n",
                    "for (uint idx = 0; idx < len; idx++) {");
                    result += &format!("\t\t*local_cache = {};\n",
                                        emit_read_u8("addr+idx", "mem_start", "warp_id"));
                    result += &format!("\t\t{}\n",
                                       "local_cache++;");

    result += &format!("\t}}\n");
    result += &format!("}}\n");

    result += &format!("\n{}\n",
        "inline void set_bit(uchar *local_cache, uint cache_idx) {");
    //result += &format!("\tlocal_cache[cache_idx / 8] |= (0x1 << (cache_idx % 8));\n");
    result += &format!("\tlocal_cache[cache_idx] = 1;\n");
    result += &format!("}}\n");

    result += &format!("\n{}\n",
        "inline void clear_bit(uchar *local_cache, uint cache_idx) {");
    //result += &format!("\tlocal_cache[cache_idx / 8] &= ~(0x1 << (cache_idx % 8));\n");
    result += &format!("\tlocal_cache[cache_idx] = 0;\n");
    result += &format!("}}\n");

    result += &format!("\n{}\n",
        "inline char get_bit(uchar *local_cache, uint cache_idx) {");
    //result += &format!("\treturn (local_cache[cache_idx / 8] >> (cache_idx % 8)) & 0x1;\n");
    result += &format!("\treturn local_cache[cache_idx];\n");
    result += &format!("}}\n");
    */

    result
}

/*
 * These are compiler-internal utility functions for emitting code
 */
pub fn emit_read_u8(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u8({}, {}, {}, read_idx)", addr, mem_start, warp_id)
}

pub fn emit_write_u8(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u8({}, {}, {}, {}, read_idx)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u16(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u16({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u16(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u16({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u32(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u32({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u32(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u32({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u64(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u64({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u64(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u64({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u16_aligned(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u16_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u16_aligned(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u16_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u32_aligned(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u32_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u32_aligned(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u32_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u64_aligned(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u64_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u64_aligned(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u64_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u16_aligned_checked(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u16_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u16_aligned_checked(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u16_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u32_aligned_checked(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u32_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u32_aligned_checked(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u32_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}

pub fn emit_read_u64_aligned_checked(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u64_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, warp_id)
}

pub fn emit_write_u64_aligned_checked(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u64_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)", addr, mem_start, value, warp_id)
}