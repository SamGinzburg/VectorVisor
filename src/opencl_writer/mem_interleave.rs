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
fn emit_fast_read_u8(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!("fast_read_u8({}, {}, {})", addr, mem_start, warp_id)
}

fn emit_fast_write_u8(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "fast_write_u8({}, {}, {}, {})",
        addr, mem_start, value, warp_id
    )
}

fn emit_write_u16_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            // write the lower byte first
            result += &format!(
                "\t{}\n",
                "write_u8(addr, mem_start, value & 0xFF, warp_id);"
            );
            // now write the upper byte
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+1), mem_start, (value >> 8) & 0xFF, warp_id);"
            );
        }
        1 => {
            // Compute the address first
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );

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
                    result += &format!(
                        "\t{}\n",
                        "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);"
                    );
                    // now write the upper byte
                    result += &format!("\t{}\n",
                                        "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");
                }
            };
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 2)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "*((global ushort*)((global uchar*)write_addr)) = value;"
                );
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u16(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)((global uchar*)write_addr)) = value;"
                );
            } else {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n", "uint temp[2];");
                result += &format!("\t{}\n", "temp[0] = (uint)*((global uint*)write_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (uint)*((global uint*)write_addr+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n", "*((global uint*)write_addr) = temp[0];");
                result += &format!(
                    "\t{}\n",
                    "*((global uint*)write_addr+(NUM_THREADS)) = temp[1];"
                );
            }
        }
        8 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 2)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "*((global ushort*)((global uchar*)write_addr)) = value;"
                );
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u16(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)((global uchar*)write_addr)) = value;"
                );
            } else {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "ulong temp[2];");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)write_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)write_addr+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n", "*((global ulong*)write_addr) = temp[0];");
                result += &format!(
                    "\t{}\n",
                    "*((global ulong*)write_addr+(NUM_THREADS)) = temp[1];"
                );
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_write_u32_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
    emit_volatile: bool,
) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!(
                "\t{}\n",
                "write_u8(addr, mem_start, value & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+1), mem_start, (value >> 8) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 16) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+3), mem_start, (value >> 24) & 0xFF, warp_id);"
            );
        }
        1 => {
            // Compute the address first
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );
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
                    result += &format!(
                        "\t{}\n",
                        "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);"
                    );
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*2), mem_start, (value >> 16) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*3), mem_start, (value >> 24) & 0xFF, warp_id);");
                }
            }
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "*((global uint*)((global uchar*)write_addr)) = value;"
                );
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u32(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "*((global volatile uint*)((global uchar*)write_addr)) = value;"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "*((global uint*)((global uchar*)write_addr)) = value;"
                    );
                }
            } else {
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "case 1: goto offset_1;");
                result += &format!("\t\t{}\n", "case 2: goto offset_2;");
                result += &format!("\t\t{}\n", "case 3: goto offset_3;");
                result += &format!("\t{}\n", "}");
                result += &format!("{}\n", "offset_0:");
                result += &format!("\t{}\n", "*((global uint*)cell1) = value;");
                result += &format!("\t{}\n", "return;");
                result += &format!("{}\n", "offset_1:");
                result += &format!(
                    "\t{}\n",
                    "*((global uchar*)cell1+cell_offset) = value & 0xFF;"
                );
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)(cell1+cell_offset+1)) = (value >> 8) & 0xFFFF;"
                );
                result += &format!("\t{}\n", "cell2 = cell1 + (NUM_THREADS*4);");
                result += &format!("\t{}\n", "*((global uchar*)cell2) = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n", "return;");
                result += &format!("{}\n", "offset_2:");
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)(cell1+cell_offset)) = value & 0xFFFF;"
                );
                result += &format!("\t{}\n", "cell2 = cell1 + (NUM_THREADS*4);");
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)cell2) = (value >> 16) & 0xFFFF;"
                );
                result += &format!("\t{}\n", "return;");
                result += &format!("{}\n", "offset_3:");
                result += &format!(
                    "\t{}\n",
                    "*((global uchar*)(cell1+cell_offset)) = value & 0xFF;"
                );
                result += &format!("\t{}\n", "cell2 = cell1 + (NUM_THREADS*4);");
                result += &format!(
                    "\t{}\n",
                    "*((global ushort*)cell2) = (value >> 8) & 0xFFFF;"
                );
                result += &format!(
                    "\t{}\n",
                    "*((global uchar*)(cell2+2)) = (value >> 24) & 0xFF;"
                );
                result += &format!("\t{}\n", "return;");
            }
        }
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "*((global uint*)((global uchar*)write_addr)) = value;"
                );
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u32(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "*((global volatile uint*)((global uchar*)write_addr)) = value;"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "*((global uint*)((global uchar*)write_addr)) = value;"
                    );
                }
            } else {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "ulong temp[2];");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)write_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)write_addr+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+2] = (value >> 16) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+3] = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n", "*((global ulong*)write_addr) = temp[0];");
                result += &format!(
                    "\t{}\n",
                    "*((global ulong*)write_addr+(NUM_THREADS)) = temp[1];"
                );
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_write_u64_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
    emit_volatile: bool,
) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!(
                "\t{}\n",
                "write_u8(addr, mem_start, value & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}",
                "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 8) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+2), mem_start, (value >> 16) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}",
                "write_u8((ulong)(((char*)addr)+3), mem_start, (value >> 24) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+4), mem_start, (value >> 32) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}",
                "write_u8((ulong)(((char*)addr)+5), mem_start, (value >> 40) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "write_u8((ulong)(((char*)addr)+6), mem_start, (value >> 48) & 0xFF, warp_id);"
            );
            result += &format!(
                "\t{}",
                "write_u8((ulong)(((char*)addr)+7), mem_start, (value >> 56) & 0xFF, warp_id);"
            );
        }
        1 => {
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );

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
                    result += &format!(
                        "\t{}\n",
                        "fast_write_u8(addr, mem_start, value & 0xFF, warp_id);"
                    );
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS), mem_start, (value >> 8) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*2), mem_start, (value >> 16) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*3), mem_start, (value >> 24) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*4), mem_start, (value >> 32) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*5), mem_start, (value >> 40) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*6), mem_start, (value >> 48) & 0xFF, warp_id);");
                    result += &format!("\t{}\n",
                                "fast_write_u8((ulong)(((ulong)addr)+NUM_THREADS*7), mem_start, (value >> 56) & 0xFF, warp_id);");
                }
            }
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "*((global uint*)((global uint*)write_addr)) = value & 0xFFFFFFFF;"
                );
                result += &format!("\t\t{}\n",
                            "*((global uint*)((global uint*)write_addr+NUM_THREADS)) = (value >> 32) & 0xFFFFFFFF;");
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u64(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "*((global volatile uint*)((global volatile uint*)write_addr)) = value & 0xFFFFFFFF;"
                    );
                    result += &format!("\t{}\n",
                                "*((global volatile uint*)((global volatile uint*)write_addr+NUM_THREADS)) = (value >> 32) & 0xFFFFFFFF;");
                } else {
                    result += &format!(
                        "\t{}\n",
                        "*((global uint*)((global uint*)write_addr)) = value & 0xFFFFFFFF;"
                    );
                    result += &format!("\t{}\n",
                                "*((global uint*)((global uint*)write_addr+NUM_THREADS)) = (value >> 32) & 0xFFFFFFFF;");
                }
            } else {
                result += &format!("\t{}\n",
                                "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n", "uint temp[3];");
                result += &format!("\t{}\n", "temp[0] = (uint)*((global uint*)write_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (uint)*((global uint*)write_addr+(NUM_THREADS));"
                );
                result += &format!(
                    "\t{}\n",
                    "temp[2] = (uint)*((global uint*)write_addr+(NUM_THREADS*2));"
                );
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+2] = (value >> 16) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+3] = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+4] = (value >> 32) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+5] = (value >> 40) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+6] = (value >> 48) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+7] = (value >> 56) & 0xFF;");
                result += &format!("\t{}\n", "*((global uint*)write_addr) = temp[0];");
                result += &format!(
                    "\t{}\n",
                    "*((global uint*)write_addr+(NUM_THREADS)) = temp[1];"
                );
                result += &format!(
                    "\t{}\n",
                    "*((global uint*)write_addr+(NUM_THREADS*2)) = temp[2];"
                );
            }
        }
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 8)) {");
                result += &format!("\t\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                /*
                result += &format!(
                            "\t\t{}\n",
                            "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                        );
                */
                result += &format!(
                    "\t\t{}\n",
                    "*((global ulong*)((global uchar*)write_addr)) = value;"
                );
                result += &format!("\t{}\n", "} else {");
                result += &format!("\t\t{}\n",
                            "write_u64(addr, mem_start, value, warp_id, read_idx, thread_idx, scratch_space);");
                result += &format!("\t{}\n", "}");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                /*
                result += &format!(
                            "\t{}\n",
                            "write_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                        );
                */
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "*((global volatile ulong*)((global uchar*)write_addr)) = value;"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "*((global ulong*)((global uchar*)write_addr)) = value;"
                    );
                }
            } else {
                result += &format!("\t{}\n",
                            "global uchar *write_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "ulong temp[2];");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)write_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)write_addr+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "combined[cell_offset] = value & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+1] = (value >> 8) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+2] = (value >> 16) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+3] = (value >> 24) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+4] = (value >> 32) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+5] = (value >> 40) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+6] = (value >> 48) & 0xFF;");
                result += &format!("\t{}\n", "combined[cell_offset+7] = (value >> 56) & 0xFF;");
                result += &format!("\t{}\n", "*((global ulong*)write_addr) = temp[0];");
                result += &format!(
                    "\t{}\n",
                    "*((global ulong*)write_addr+(NUM_THREADS)) = temp[1];"
                );
            }
        }

        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u16_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
) -> String {
    let mut result = String::from("");
    match interleave {
        0 => {
            result += &format!("\t{}\n", "ushort temp = 0;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);"
            );
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!("\t{}\n", "temp += read_u8(addr, mem_start, warp_id);");
            result += &format!("\t{}", "return temp;");
        }
        1 => {
            // use a local variable to store the result as we perform the reads
            // we have to read in the reverse order!!! (high bits then low bits)
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );
            result += &format!("\t{}\n", "ushort temp = 0;");

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
                                        "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n", "temp += fast_read_u8(addr, mem_start, warp_id);");
                }
            }

            result += &format!("\t{}", "return temp;");
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 2)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "return *((global ushort*)((global ushort*)read_addr));"
                );
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u16(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!(
                    "\t{}\n",
                    "return *((global ushort*)((global uchar*)read_addr));"
                );
            } else {
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "uint temp[2] = { 0 };");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "case 1: goto offset_1;");
                result += &format!("\t\t{}\n", "case 2: goto offset_2;");
                result += &format!("\t\t{}\n", "case 3: goto offset_3;");
                result += &format!("\t{}\n", "}");

                result += &format!("{}\n", "offset_0:");
                result += &format!(
                    "\t{}\n",
                    "return *((global ushort*)((global ushort*)cell1));"
                );
                result += &format!("{}\n", "offset_2:");
                result += &format!("\t{}\n",
                                "return *((global ushort*)((global ushort*)((global uchar*)cell1+cell_offset)));");
                result += &format!("{}\n", "offset_1:");
                result += &format!("{}\n", "offset_3:");
                result += &format!("\t{}\n", "temp[0] = (uint)*((global uint*)cell1);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (uint)*((global uint*)cell1+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "temp[1] = (temp[1] << ((4-cell_offset)*8));");
                result += &format!("\t{}\n", "return (ushort)(temp[0] + temp[1]);");
            }
        }
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 2)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "return *((global ushort*)((global uchar*)read_addr));"
                );
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u16(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t{}\n",
                    "return *((global ushort*)((global uchar*)read_addr));"
                );
            } else {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "ulong temp[2];");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)read_addr);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)read_addr+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "temp[1] = (temp[1] << ((8-cell_offset)*8));");
                result += &format!("\t{}\n", "return (ushort)(temp[0] + temp[1]);");
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u32_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
    emit_volatile: bool,
) -> String {
    let mut result = String::from("");
    match interleave {
        0 => {
            result += &format!("\t{}\n", "uint temp = 0;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+3), mem_start, warp_id);"
            );
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+2), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);"
            );
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)), mem_start, warp_id);"
            );
            result += &format!("\t{}", "return temp;");
        }
        1 => {
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );
            result += &format!("\t{}\n", "uchar4 temp = (uchar4)(0);");
            result += &format!("\t{}\n",
                                "temp.s3 = fast_read_u8((ulong)(((ulong)addr)+(NUM_THREADS*3)), mem_start, warp_id);");
            result += &format!("\t{}\n",
                                "temp.s2 = fast_read_u8((ulong)(((ulong)addr)+(NUM_THREADS*2)), mem_start, warp_id);");
            result += &format!(
                "\t{}\n",
                "temp.s1 = fast_read_u8((ulong)(((ulong)addr)+(NUM_THREADS)), mem_start, warp_id);"
            );
            result += &format!(
                "\t{}\n",
                "temp.s0 = fast_read_u8((ulong)(((ulong)addr)), mem_start, warp_id);"
            );
            result += &format!("\t{}", "return as_uint(temp);");
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "return *((global uint*)((global uint*)read_addr));"
                );
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u32(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "return *((global volatile uint*)((global uint*)read_addr));"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "return *((global uint*)((global uint*)read_addr));"
                    );
                }
            } else {
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "uint temp[2] = { 0 };");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "case 1: goto offset_1;");
                result += &format!("\t\t{}\n", "case 2: goto offset_2;");
                result += &format!("\t\t{}\n", "case 3: goto offset_3;");
                result += &format!("\t{}\n", "}");

                result += &format!("{}\n", "offset_0:");
                result += &format!("\t{}\n", "return *((global uint*)((global uint*)cell1));");
                result += &format!("{}\n", "offset_1:");
                result += &format!("{}\n", "offset_2:");
                result += &format!("{}\n", "offset_3:");
                result += &format!("\t{}\n", "temp[0] = (uint)*((global uint*)cell1);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (uint)*((global uint*)cell1+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "temp[1] = (temp[1] << ((4-cell_offset)*8));");
                result += &format!("\t{}\n", "return (uint)(temp[0] + temp[1]);");
            }
        }
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t\t{}\n",
                    "ulong offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!(
                    "\t\t{}\n",
                    "return *((global uint*)((global uchar*)read_addr+offset));"
                );
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u32(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "return *((global volatile uint*)((global uchar*)read_addr));"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "return *((global uint*)((global uchar*)read_addr));"
                    );
                }
            } else {
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "ulong temp[2] = { 0 };");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "case 1:");
                result += &format!("\t\t{}\n", "case 2:");
                result += &format!("\t\t{}\n", "case 3:");
                result += &format!("\t\t{}\n", "case 4: goto offset_1;");
                result += &format!("\t\t{}\n", "default: goto offset_2;");
                result += &format!("\t{}\n", "}");
                result += &format!("{}\n", "offset_0:");
                result += &format!(
                    "\t{}\n",
                    "return *((global uint*)((global uchar*)cell1+cell_offset));"
                );
                result += &format!("{}\n", "offset_1:");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)cell1);");
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "return (uint)(temp[0]);");
                result += &format!("{}\n", "offset_2:");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)cell1);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)cell1+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "temp[1] = (temp[1] << ((8-cell_offset)*8));");
                result += &format!("\t{}\n", "return (uint)(temp[0] + temp[1]);");
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

fn emit_read_u64_body(
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    emit_aligned: bool,
    emit_checked: bool,
    emit_volatile: bool,
) -> String {
    let mut result = String::from("");

    match interleave {
        0 => {
            result += &format!("\t{}\n", "ulong temp = 0;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+7), mem_start, warp_id);"
            );
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+6), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+5), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+4), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+3), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+2), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)+1), mem_start, warp_id);"
            );
            result += &format!("\t{}\n", "temp = temp << 8;");
            result += &format!(
                "\t{}\n",
                "temp += read_u8((ulong)(((char*)addr)), mem_start, warp_id);"
            );
            result += &format!("\t{}", "return temp;");
        }
        1 => {
            result += &format!(
                "\t{}\n",
                "addr = (ulong)((ulong)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start));"
            );

            // use a local variable to store the result as we perform the reads
            result += &format!("\t{}\n", "ulong temp = 0;");

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
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*7), mem_start, warp_id);");
                    // bitshift over to make room for the next byte
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*6), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*5), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*4), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*3), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS*2), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!("\t{}\n",
                                    "temp += fast_read_u8((ulong)(((ulong)addr)+NUM_THREADS), mem_start, warp_id);");
                    result += &format!("\t{}\n", "temp = temp << 8;");
                    result += &format!(
                        "\t{}\n",
                        "temp += fast_read_u8((ulong)(((ulong)addr)), mem_start, warp_id);"
                    );
                }
            }

            result += &format!("\t{}", "return temp;");
        }
        4 => {
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 4)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!("\t\t{}\n", "ulong temp = 0;");
                result += &format!(
                    "\t\t{}\n",
                    "temp += *((global uint*)((global uint*)read_addr+NUM_THREADS));"
                );
                result += &format!("\t\t{}\n", "temp = temp << 32;");
                result += &format!(
                    "\t\t{}\n",
                    "temp += *((global uint*)((global uint*)read_addr));"
                );
                result += &format!("\t\t{}\n", "return temp;");
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u64(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!("\t{}\n", "ulong temp = 0;");
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "temp += *((global volatile uint*)((global uint*)read_addr+NUM_THREADS));"
                    );
                    result += &format!("\t{}\n", "temp  = temp << 32;");
                    result += &format!(
                        "\t{}\n",
                        "temp += *((global volatile uint*)((global uint*)read_addr));"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "temp += *((global uint*)((global uint*)read_addr+NUM_THREADS));"
                    );
                    result += &format!("\t{}\n", "temp  = temp << 32;");
                    result += &format!(
                        "\t{}\n",
                        "temp += *((global uint*)((global uint*)read_addr));"
                    );
                }
                result += &format!("\t{}\n", "return temp;");
            } else {
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/4)*(NUM_THREADS*4) + (warp_id*4) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 4);"
                );
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "uint temp[3] = { 0 };");
                result += &format!("\t{}\n", "uchar result[8];");
                result += &format!("\t{}\n", "uchar *combined = (uchar*)&temp;");
                result += &format!("\t{}\n", "ulong opt_temp = 0;");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "case 1: goto offset_1;");
                result += &format!("\t\t{}\n", "case 2: goto offset_2;");
                result += &format!("\t\t{}\n", "case 3: goto offset_3;");
                result += &format!("\t{}\n", "}");

                result += &format!("{}\n", "offset_0:");
                result += &format!(
                    "\t{}\n",
                    "opt_temp += *((global uint*)((global uint*)cell1+NUM_THREADS));"
                );
                result += &format!("\t{}\n", "opt_temp  = opt_temp << 32;");
                result += &format!(
                    "\t{}\n",
                    "opt_temp += *((global uint*)((global uint*)cell1));"
                );
                result += &format!("\t{}\n", "return opt_temp;");
                result += &format!("{}\n", "offset_1:");
                result += &format!("{}\n", "offset_2:");
                result += &format!("{}\n", "offset_3:");
                result += &format!("\t{}\n", "temp[0] = (uint)*((global uint*)cell1);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (uint)*((global uint*)cell1+(NUM_THREADS));"
                );
                result += &format!(
                    "\t{}\n",
                    "temp[2] = (uint)*((global uint*)cell1+(NUM_THREADS*2));"
                );

                result += &format!("\t{}\n", "result[0] = combined[cell_offset];");
                result += &format!("\t{}\n", "result[1] = combined[cell_offset+1];");
                result += &format!("\t{}\n", "result[2] = combined[cell_offset+2];");
                result += &format!("\t{}\n", "result[3] = combined[cell_offset+3];");
                result += &format!("\t{}\n", "result[4] = combined[cell_offset+4];");
                result += &format!("\t{}\n", "result[5] = combined[cell_offset+5];");
                result += &format!("\t{}\n", "result[6] = combined[cell_offset+6];");
                result += &format!("\t{}\n", "result[7] = combined[cell_offset+7];");

                result += &format!("\tulong final_result = 0;\n");
                result += &format!(
                    "\t___private_memcpy_nonmmu((void*)&final_result, (void*)&result, sizeof(ulong));\n",
                );
                result += &format!("\t{}\n", "return final_result;");
            }
        }
        8 => {
            // determine which cell to read
            if emit_aligned && emit_checked {
                result += &format!("\t{}\n", "if (IS_ALIGNED_POW2((ulong)addr, 8)) {");
                result += &format!("\t\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                /*
                result += &format!(
                            "\t\t{}\n",
                            "read_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                        );
                */
                result += &format!(
                    "\t\t{}\n",
                    "return *((global ulong*)((global uchar*)read_addr));"
                );
                result += &format!("\t{}\n", "}");
                result += &format!("\t{}\n",
                                "return read_u64(addr, mem_start, warp_id, read_idx, thread_idx, scratch_space);");
            } else if emit_aligned {
                result += &format!("\t{}\n",
                                "global uchar *read_addr = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                /*
                result += &format!(
                            "\t{}\n",
                            "read_addr += GET_POW2_OFFSET((addr-mem_start), 8);"
                        );
                */
                if emit_volatile {
                    result += &format!(
                        "\t{}\n",
                        "return *((global volatile ulong*)((global uchar*)read_addr));"
                    );
                } else {
                    result += &format!(
                        "\t{}\n",
                        "return *((global ulong*)((global uchar*)read_addr));"
                    );
                }
            } else {
                result += &format!("\t{}\n",
                                "global uchar *cell1 = ((global uchar*)(((addr-mem_start)/8)*(NUM_THREADS*8) + (warp_id*8) + mem_start));");
                result += &format!(
                    "\t{}\n",
                    "ulong cell_offset = GET_POW2_OFFSET((addr-mem_start), 8);"
                );
                result += &format!("\t{}\n", "global uchar *cell2;");
                result += &format!("\t{}\n", "ulong temp[2] = { 0 };");
                result += &format!("\t{}\n", "switch (cell_offset) {");
                result += &format!("\t\t{}\n", "case 0: goto offset_0;");
                result += &format!("\t\t{}\n", "default: goto offset_1;");
                result += &format!("\t{}\n", "}");
                result += &format!("{}\n", "offset_0:");
                result += &format!("\t{}\n", "return *((global ulong*)((global ulong*)cell1));");
                result += &format!("{}\n", "offset_1:");
                result += &format!("\t{}\n", "temp[0] = (ulong)*((global ulong*)cell1);");
                result += &format!(
                    "\t{}\n",
                    "temp[1] = (ulong)*((global ulong*)cell1+(NUM_THREADS));"
                );
                result += &format!("\t{}\n", "temp[0] = (temp[0] >> (cell_offset*8));");
                result += &format!("\t{}\n", "temp[1] = (temp[1] << ((8-cell_offset)*8));");
                result += &format!("\t{}\n", "return (ulong)(temp[0] + temp[1]);");
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result
}

pub fn generate_bulkmem(fill: bool, interleave: u32) -> String {
    let mut result = String::from("");

    let name = match fill {
        false => {
            result += &format!("\n{}\n",
            "void ___private_bulk_memcpy(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong heap_base, ulong buf_len_bytes, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {",
            );
        }
        true => {
            result += &format!("\n{}\n",
            "void ___private_bulk_memfill(ulong dst, ulong mem_start_dst, uchar value, ulong heap_base, ulong buf_len_bytes, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {",
            );
        }
    };

    // Copy from front-to-back
    if !fill {
        result += &format!("\t{}\n", "if (dst < src) {");
    }

    result += &format!("\t\t{}\n", "uint counter = 0;");
    match (fill, interleave) {
        (true, 4) => {
            result += &format!("\t{}\n", "uint fillval = 0;");
            result += &format!("\t{}\n", "fillval += (ulong)value;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 8;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 16;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 24;");
        }
        (true, _) => {
            result += &format!("\t{}\n", "ulong fillval = 0;");
            result += &format!("\t{}\n", "fillval += (ulong)value;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 8;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 16;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 24;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 32;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 40;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 48;");
            result += &format!("\t{}\n", "fillval += (ulong)value << 56;");
        }
        _ => {}
    };

    // fastpath for u32 ops
    match (fill, interleave) {
        (false, 4) | (false, 1) => {
            result += &format!(
        	"\t\t{}\n",
        	"if (buf_len_bytes >= 128 && IS_ALIGNED_POW2((ulong)src, 4) && IS_ALIGNED_POW2((ulong)dst, 4)) {"
    	    );
        }
        (false, 8) => {
            result += &format!(
        	"\t\t{}\n",
        	"if (buf_len_bytes >= 128 && IS_ALIGNED_POW2((ulong)src, 8) && IS_ALIGNED_POW2((ulong)dst, 8)) {"
    	    );
        }
        (true, 4) | (true, 1) => {
            result += &format!(
                "\t\t{}\n",
                "if (buf_len_bytes >= 128 && IS_ALIGNED_POW2((ulong)dst, 4)) {"
            );
        }
        (true, 8) => {
            result += &format!(
                "\t\t{}\n",
                "if (buf_len_bytes >= 128 && IS_ALIGNED_POW2((ulong)dst, 8)) {"
            );
        }
        _ => panic!("unspecified interleave in bulkmem"),
    };
    result += &format!(
        "\t\t\t{}\n",
        "for (; counter < (buf_len_bytes-GET_POW2_OFFSET(buf_len_bytes, 128)); counter+=128) {"
    );

    match interleave {
        1 | 4 => {
            result += &format!(
                "\t\t\t\t#pragma unroll(8)\n\t\t\t{}\n",
                "for (uint unroll = 0; unroll < 128; unroll+=16) {"
            );
        }
        8 => {
            result += &format!(
                "\t\t\t\t#pragma unroll(4)\n\t\t\t{}\n",
                "for (uint unroll = 0; unroll < 128; unroll+=32) {"
            );
        }
        _ => panic!("unspecified interleave in bulkmem ops"),
    }

    match (fill, interleave) {
        (true, 1) => {
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll)",
                    "(ulong)(mem_start_dst)",
                    &"fillval",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+4)",
                    "(ulong)(mem_start_dst)",
                    &"fillval",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+8)",
                    "(ulong)(mem_start_dst)",
                    &"fillval",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+12)",
                    "(ulong)(mem_start_dst)",
                    &"fillval",
                    "warp_id"
                ),
            );
        }
        (true, 4) => {
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+4)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+8)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+12)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
        }
        (true, _) => {
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+8)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+16)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+24)",
                    "(ulong)(heap_base)",
                    &"fillval",
                ),
            );
        }
        // For interleaves of 1/8 it is better to emit standard ops
        // But only for memcpy, for memfill this is not needed
        (false, 1) => {
            result += &format!(
                "\t\t\t\t\tuint value1 = {};\n",
                &emit_read_u32_aligned(
                    "(ulong)(mem_start_src+src+counter+unroll)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\tuint value2 = {};\n",
                &emit_read_u32_aligned(
                    "(ulong)(mem_start_src+src+counter+unroll+4)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\tuint value3 = {};\n",
                &emit_read_u32_aligned(
                    "(ulong)(mem_start_src+src+counter+unroll+8)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\tuint value4 = {};\n",
                &emit_read_u32_aligned(
                    "(ulong)(mem_start_src+src+counter+unroll+12)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                ),
            );

            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll)",
                    "(ulong)(mem_start_dst)",
                    "value1",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+4)",
                    "(ulong)(mem_start_dst)",
                    "value2",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+8)",
                    "(ulong)(mem_start_dst)",
                    "value3",
                    "warp_id"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_aligned(
                    "(ulong)(mem_start_dst+dst+counter+unroll+12)",
                    "(ulong)(mem_start_dst)",
                    "value4",
                    "warp_id"
                ),
            );
        }
        (false, 4) => {
            result += &format!(
                "\t\t\t\t\tuint value1 = {};\n",
                &emit_read_u32_fast("(ulong)(src+counter+unroll)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tuint value2 = {};\n",
                &emit_read_u32_fast("(ulong)(src+counter+unroll+4)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tuint value3 = {};\n",
                &emit_read_u32_fast("(ulong)(src+counter+unroll+8)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tuint value4 = {};\n",
                &emit_read_u32_fast("(ulong)(src+counter+unroll+12)", "(ulong)(heap_base)",),
            );

            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll)",
                    "(ulong)(heap_base)",
                    "value1"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+4)",
                    "(ulong)(heap_base)",
                    "value2"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+8)",
                    "(ulong)(heap_base)",
                    "value3"
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u32_fast(
                    "(ulong)(dst+counter+unroll+12)",
                    "(ulong)(heap_base)",
                    "value4"
                ),
            );
        }
        (false, _) => {
            result += &format!(
                "\t\t\t\t\tulong value1 = {};\n",
                &emit_read_u64_fast("(ulong)(src+counter+unroll)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tulong value2 = {};\n",
                &emit_read_u64_fast("(ulong)(src+counter+unroll+8)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tulong value3 = {};\n",
                &emit_read_u64_fast("(ulong)(src+counter+unroll+16)", "(ulong)(heap_base)",),
            );
            result += &format!(
                "\t\t\t\t\tulong value4 = {};\n",
                &emit_read_u64_fast("(ulong)(src+counter+unroll+24)", "(ulong)(heap_base)",),
            );

            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll)",
                    "(ulong)(heap_base)",
                    "value1",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+8)",
                    "(ulong)(heap_base)",
                    "value2",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+16)",
                    "(ulong)(heap_base)",
                    "value3",
                ),
            );
            result += &format!(
                "\t\t\t\t\t{};\n",
                &emit_write_u64_fast(
                    "(ulong)(dst+counter+unroll+24)",
                    "(ulong)(heap_base)",
                    "value4",
                ),
            );
        }
    };

    result += &format!("\t\t\t\t{}\n", "}");
    result += &format!("\t\t\t{}\n", "}");
    result += &format!("\t\t{}\n", "}");

    // slow path for remaining ops
    result += &format!("\t\t{}\n", "for (; counter < buf_len_bytes; counter++) {");

    match fill {
        true => {
            result += &format!(
                "\t\t\t{};\n",
                &emit_write_u8(
                    "(ulong)(mem_start_dst+dst+counter)",
                    "(ulong)(mem_start_dst)",
                    &"value",
                    "warp_id"
                )
            );
        }
        false => {
            result += &format!(
                "\t\t\t{};\n",
                &emit_write_u8(
                    "(ulong)(mem_start_dst+dst+counter)",
                    "(ulong)(mem_start_dst)",
                    &emit_read_u8(
                        "(ulong)(mem_start_src+src+counter)",
                        "(ulong)(mem_start_src)",
                        "warp_id"
                    ),
                    "warp_id"
                )
            );
        }
    };

    result += &format!("\t\t{}\n", "}");

    if !fill {
        result += &format!("\t{}\n", "} else if (src < dst) {");
        // Copy back-to-front
        // Try to perform higher-performance copies
        match interleave {
            1 => {
                // alignments don't matter for the 1 byte interleave ("everything" is aligned)
                result += &format!("\t\t{}\n", "if (buf_len_bytes >= 128) {");
                result += &format!(
                    "\t\t\t{}\n",
                    "for (; buf_len_bytes >= 128; buf_len_bytes -= 128) {"
                );
                result += &format!(
                    "\t\t\t\t#pragma unroll(8)\n\t\t\t\t{}\n",
                    "for (uint unroll = 0; unroll < 128; unroll+=16) {"
                );
                result += &format!(
                    "\t\t\t\t\tuint value1 = {};\n",
                    &emit_read_u32_aligned(
                        "(ulong)(mem_start_src+src+buf_len_bytes-unroll-4)",
                        "(ulong)(mem_start_src)",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value2 = {};\n",
                    &emit_read_u32_aligned(
                        "(ulong)(mem_start_src+src+buf_len_bytes-unroll-8)",
                        "(ulong)(mem_start_src)",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value3 = {};\n",
                    &emit_read_u32_aligned(
                        "(ulong)(mem_start_src+src+buf_len_bytes-unroll-12)",
                        "(ulong)(mem_start_src)",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value4 = {};\n",
                    &emit_read_u32_aligned(
                        "(ulong)(mem_start_src+src+buf_len_bytes-unroll-16)",
                        "(ulong)(mem_start_src)",
                        "warp_id"
                    ),
                );

                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_aligned(
                        "(ulong)(mem_start_dst+dst+buf_len_bytes-unroll-4)",
                        "(ulong)(mem_start_dst)",
                        "value1",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_aligned(
                        "(ulong)(mem_start_dst+dst+buf_len_bytes-unroll-8)",
                        "(ulong)(mem_start_dst)",
                        "value2",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_aligned(
                        "(ulong)(mem_start_dst+dst+buf_len_bytes-unroll-12)",
                        "(ulong)(mem_start_dst)",
                        "value3",
                        "warp_id"
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_aligned(
                        "(ulong)(mem_start_dst+dst+buf_len_bytes-unroll-16)",
                        "(ulong)(mem_start_dst)",
                        "value4",
                        "warp_id"
                    ),
                );
                result += &format!("\t\t\t\t}}\n",);
                result += &format!("\t\t\t}}\n",);
            }
            4 => {
                result += &format!(
                "\t\t{}\n",
                "if (buf_len_bytes >= 128 && IS_ALIGNED_POW2(src+buf_len_bytes, 4) && IS_ALIGNED_POW2(dst+buf_len_bytes, 4)) {"
                );
                result += &format!(
                    "\t\t\t{}\n",
                    "for (; buf_len_bytes >= 128; buf_len_bytes -= 128) {"
                );
                result += &format!(
                    "\t\t\t\t#pragma unroll(8)\n\t\t\t\t{}\n",
                    "for (uint unroll = 0; unroll < 128; unroll+=16) {"
                );
                result += &format!(
                    "\t\t\t\t\tuint value1 = {};\n",
                    &emit_read_u32_fast(
                        "(ulong)(src+buf_len_bytes-unroll-4)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value2 = {};\n",
                    &emit_read_u32_fast(
                        "(ulong)(src+buf_len_bytes-unroll-8)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value3 = {};\n",
                    &emit_read_u32_fast(
                        "(ulong)(src+buf_len_bytes-unroll-12)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tuint value4 = {};\n",
                    &emit_read_u32_fast(
                        "(ulong)(src+buf_len_bytes-unroll-16)",
                        "(ulong)(heap_base)",
                    ),
                );

                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-4)",
                        "(ulong)(heap_base)",
                        "value1",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-8)",
                        "(ulong)(heap_base)",
                        "value2",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-12)",
                        "(ulong)(heap_base)",
                        "value3",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u32_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-16)",
                        "(ulong)(heap_base)",
                        "value4",
                    ),
                );

                result += &format!("\t\t\t\t}}\n",);
                result += &format!("\t\t\t}}\n",);
            }
            8 => {
                result += &format!(
                "\t\t{}\n",
                "if (buf_len_bytes >= 128 && IS_ALIGNED_POW2(src+buf_len_bytes, 8) && IS_ALIGNED_POW2(dst+buf_len_bytes, 8)) {"
                );
                result += &format!(
                    "\t\t\t{}\n",
                    "for (; buf_len_bytes >= 128; buf_len_bytes -= 128) {"
                );
                result += &format!(
                    "\t\t\t\t#pragma unroll(4)\n\t\t\t\t{}\n",
                    "for (uint unroll = 0; unroll < 128; unroll+=32) {"
                );
                result += &format!(
                    "\t\t\t\t\tulong value1 = {};\n",
                    &emit_read_u64_fast(
                        "(ulong)(src+buf_len_bytes-unroll-8)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tulong value2 = {};\n",
                    &emit_read_u64_fast(
                        "(ulong)(src+buf_len_bytes-unroll-16)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tulong value3 = {};\n",
                    &emit_read_u64_fast(
                        "(ulong)(src+buf_len_bytes-unroll-24)",
                        "(ulong)(heap_base)",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\tulong value4 = {};\n",
                    &emit_read_u64_fast(
                        "(ulong)(src+buf_len_bytes-unroll-32)",
                        "(ulong)(heap_base)",
                    ),
                );

                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u64_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-8)",
                        "(ulong)(heap_base)",
                        "value1",
                    ),
                );

                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u64_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-16)",
                        "(ulong)(heap_base)",
                        "value2",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u64_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-24)",
                        "(ulong)(heap_base)",
                        "value3",
                    ),
                );
                result += &format!(
                    "\t\t\t\t\t{};\n",
                    &emit_write_u64_fast(
                        "(ulong)(dst+buf_len_bytes-unroll-32)",
                        "(ulong)(heap_base)",
                        "value4",
                    ),
                );

                result += &format!("\t\t\t\t}}\n",);
                result += &format!("\t\t\t}}\n",);
            }
            _ => panic!("unimplemented interleave for back-to-front memory.copy"),
        }
        result += &format!("\t\t}}\n",);

        result += &format!("\t\t{}\n", "while (buf_len_bytes--) {");

        result += &format!(
            "\t\t\t{};\n",
            &emit_write_u8(
                "(ulong)(mem_start_dst+dst+buf_len_bytes)",
                "(ulong)(mem_start_dst)",
                &emit_read_u8(
                    "(ulong)(mem_start_src+src+buf_len_bytes)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                ),
                "warp_id"
            )
        );
        result += &format!("\t\t{}\n", "}");
        result += &format!("\t{}\n", "}");
    }

    result += &format!("\n{}\n", "}");

    result
}

pub fn generate_read_write_calls(
    _writer: &opencl_writer::OpenCLCWriter,
    interleave: u32,
    local_work_group: usize,
    mexec: usize,
    volatile: bool,
    _debug: bool,
) -> String {
    let mut result = String::from("");

    result += &format!(
        "\n{}\n",
        "inline void * ___private_memcpy_nonmmu(void *dest, void *src, size_t len) {"
    );
    result += &format!(
        "\t{}\n\t{}\n\t{}\n\t{}\n\t{}\n",
        "char *d = dest;", "char *s = src;", "while (len--)", "  *d++ = *s++;", "return dest;"
    );
    result += &format!("}}\n");

    // fast_write_u8 is used for writes greater than 1 byte to reduce computation
    result += &format!(
        "\n{}\n",
        "inline void fast_write_u8(ulong addr, ulong mem_start, uchar value, uint warp_id) {"
    );

    match interleave {
        0 | 1 | 4 | 8 => {
            if volatile {
                result += &format!("\t{}", "*((global volatile uchar*)addr) = value;");
            } else {
                result += &format!("\t{}", "*((global uchar*)addr) = value;");
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "inline void write_u8(ulong addr, ulong mem_start, uchar value, uint warp_id, uint read_idx) {");

    match interleave {
        0 => {
            result += &format!("\t{}\n", "*((global uchar*)addr) = value;");
        }
        1 => {
            if volatile {
                result += &format!(
                    "\t{}\n",
                    "*((global volatile uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start)) = value;"
                )
            } else {
                result += &format!(
                    "\t{}\n",
                    "*((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start)) = value;"
                )
            }
        }
        4 | 8 => {
            // determine which cell to read
            let write = format!("global uchar *write_addr = ((global uchar*)(((addr-mem_start)/{})*(NUM_THREADS*{}) + (warp_id*{}) + mem_start));", interleave, interleave, interleave);
            result += &format!("\t{}\n", write);
            result += &format!(
                "\t{}\n",
                format!(
                    "write_addr += GET_POW2_OFFSET((addr-mem_start), {});",
                    interleave
                )
            );
            if volatile {
                result += &format!("\t{}\n", "*(global volatile uchar*)(write_addr) = value;")
            } else {
                result += &format!("\t{}\n", "*(global uchar*)(write_addr) = value;")
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "void write_u16(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u16_body(interleave, local_work_group, mexec, false, false)
    );
    result += &format!("\n{}\n", "}");

    if interleave != 4 {
        result += &format!("\n{}\n",
                                "inline void write_u16_aligned(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
        result += &format!(
            "{}",
            emit_write_u16_body(interleave, local_work_group, mexec, true, false)
        );
        result += &format!("\n{}\n", "}");
    }

    result += &format!("\n{}\n",
                        "inline void write_u16_aligned_checked(ulong addr, ulong mem_start, ushort value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u16_body(interleave, local_work_group, mexec, true, true)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "void write_u32(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u32_body(interleave, local_work_group, mexec, false, false, volatile)
    );
    result += &format!("\n{}\n", "}");

    if interleave != 4 {
        result += &format!("\n{}\n",
                                "inline void write_u32_aligned(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
        result += &format!(
            "{}",
            emit_write_u32_body(interleave, local_work_group, mexec, true, false, volatile)
        );
        result += &format!("\n{}\n", "}");
    }

    result += &format!("\n{}\n",
                        "inline void write_u32_aligned_checked(ulong addr, ulong mem_start, uint value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u32_body(interleave, local_work_group, mexec, true, true, volatile)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "void write_u64(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u64_body(interleave, local_work_group, mexec, false, false, volatile)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                            "inline void write_u64_aligned(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u64_body(interleave, local_work_group, mexec, true, false, volatile)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "inline void write_u64_aligned_checked(ulong addr, ulong mem_start, ulong value, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_write_u64_body(interleave, local_work_group, mexec, true, true, volatile)
    );
    result += &format!("\n{}\n", "}");

    // the read functions

    result += &format!(
        "\n{}\n",
        "inline uchar fast_read_u8(ulong addr, ulong mem_start, uint warp_id) {"
    );
    match interleave {
        0 | 1 | 4 | 8 => {
            if volatile {
                result += &format!("\t{}", "return *((global volatile uchar*)addr);");
            } else {
                result += &format!("\t{}", "return *((global uchar*)addr);");
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n", "}");

    result += &format!(
        "\n{}\n",
        "inline uchar read_u8(ulong addr, ulong mem_start, uint warp_id, uint read_idx) {"
    );
    match interleave {
        0 => {
            result += &format!("\t{}", "return *((global uchar*)addr);");
        }
        1 => {
            if volatile {
                result += &format!(
                    "\t{}",
                    "return *((global volatile uchar*)((addr-mem_start)*NUM_THREADS + warp_id + mem_start));"
                );
            } else {
                result += &format!(
                    "\t{}",
                    "return *((global uchar*)((addr-mem_start)*NUM_THREADS + warp_id + mem_start));"
                );
            }
        }
        4 | 8 => {
            let read = format!("global uchar *read_addr = ((global uchar*)(((addr-mem_start)/{})*(NUM_THREADS*{}) + (warp_id*{}) + mem_start));", interleave, interleave, interleave);
            result += &format!("\t{}\n", read);
            result += &format!(
                "\t{}\n",
                format!(
                    "read_addr += GET_POW2_OFFSET((addr-mem_start), {});",
                    interleave
                )
            );
            if volatile {
                result += &format!("\t{}\n", "return *(global volatile uchar*)(read_addr);")
            } else {
                result += &format!("\t{}\n", "return *(read_addr);")
            }
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "ushort read_u16(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u16_body(interleave, local_work_group, mexec, false, false)
    );
    result += &format!("\n{}", "}");

    if interleave != 4 {
        result += &format!("\n{}\n",
                                "inline ushort read_u16_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
        result += &format!(
            "{}",
            emit_read_u16_body(interleave, local_work_group, mexec, true, false)
        );
        result += &format!("\n{}", "}");
    }

    result += &format!("\n{}\n",
                        "inline ushort read_u16_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u16_body(interleave, local_work_group, mexec, true, true)
    );
    result += &format!("\n{}", "}");

    result += &format!("\n{}\n",
                        "uint read_u32(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u32_body(interleave, local_work_group, mexec, false, false, volatile)
    );
    result += &format!("\n{}", "}");

    if interleave != 4 {
        result += &format!("\n{}\n",
                                "inline uint read_u32_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
        result += &format!(
            "{}",
            emit_read_u32_body(interleave, local_work_group, mexec, true, false, volatile)
        );
        result += &format!("\n{}", "}");
    }

    result += &format!("\n{}\n",
                        "inline uint read_u32_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u32_body(interleave, local_work_group, mexec, true, true, volatile)
    );
    result += &format!("\n{}", "}");

    result += &format!("\n{}\n",
                        "ulong read_u64(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u64_body(interleave, local_work_group, mexec, false, false, volatile)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                            "inline ulong read_u64_aligned(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u64_body(interleave, local_work_group, mexec, true, false, volatile)
    );
    result += &format!("\n{}\n", "}");

    result += &format!("\n{}\n",
                        "inline ulong read_u64_aligned_checked(ulong addr, ulong mem_start, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!(
        "{}",
        emit_read_u64_body(interleave, local_work_group, mexec, true, true, volatile)
    );
    result += &format!("\n{}\n", "}");

    // emit a memcpy function as well, for utility purposes
    result += &format!("\n{}\n",
        "void ___private_memcpy(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx) {");

    result += &format!("\t{}\n", "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

    result += &format!(
        "\t{};\n",
        emit_write_u8(
            "(ulong)(dst+idx)",
            "(ulong)(mem_start_dst)",
            &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id"),
            "warp_id"
        )
    );

    result += &format!("\t{}\n", "}");
    result += &format!("\n{}\n", "}");

    // emit bulk memory operations
    result += &generate_bulkmem(true, interleave);
    result += &generate_bulkmem(false, interleave);

    // write from the GPU (interleaved) back to the CPU (non-interleaved)
    // The destination is always >> 16 byte aligned.
    // The source is a u8 vec so it doesn't have alignment guarantees.
    // We check the alignment to see if we get lucky though
    result += &format!("\n{}\n",
        "void ___private_memcpy_gpu2cpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("\t{}\n", "global char *dst_tmp = (global char*)(dst);");

    match interleave {
        0 => {
            result += &format!("\t{}\n", "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

            result += &format!(
                "\t\t{} = {};\n",
                "*dst_tmp++",
                &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id")
            );

            result += &format!("\t{}\n", "}");
        }
        1 => {
            result += &format!("\t{}\n",
                               "ulong addr = (ulong)((global uchar*)(((src)-(ulong)(mem_start_src))*(NUM_THREADS) + warp_id + (ulong)mem_start_src));");

            result += &format!("\t{}\n", "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

            result += &format!(
                "\t\t{} = {};\n",
                "*dst_tmp++",
                &emit_fast_read_u8(
                    "(ulong)(addr+idx*NUM_THREADS)",
                    "(ulong)(mem_start_src)",
                    "warp_id"
                )
            );

            result += &format!("\t{}\n", "}");
        }
        4 | 8 => {
            result += &format!("\t{}\n", "global uint *dst_tmp_uint = (global uint*)(dst);");
            result += &format!("\t{}\n", "uint counter = 0;");
            result += &format!(
                "\t{}\n",
                "if (buf_len_bytes > 4 && IS_ALIGNED_POW2((ulong)src, 4) && IS_ALIGNED_POW2((ulong)dst, 4)) {"
            );
            result += &format!(
                "\t\t{}\n",
                "for (; counter < (buf_len_bytes-GET_POW2_OFFSET(buf_len_bytes, 4)); counter+=4) {"
            );

            result += &format!(
                "\t\t\t{} = {};\n",
                "*dst_tmp_uint++",
                &emit_read_u32_aligned("(ulong)(src+counter)", "(ulong)(mem_start_src)", "warp_id"),
            );

            result += &format!("\t\t{}\n", "}");
            result += &format!("\t{}\n", "}");
            result += &format!("\t{}\n", "dst_tmp = (global uchar*)(dst_tmp_uint);");
            result += &format!("\t{}\n", "for (; counter < buf_len_bytes; counter++) {");
            result += &format!(
                "\t\t{} = {};\n",
                "*(global uchar*)dst_tmp++",
                &emit_read_u8("(ulong)(src+counter)", "(ulong)(mem_start_src)", "warp_id")
            );
            result += &format!("\t{}\n", "}");
        }
        _ => panic!("Unsupported read/write interleave"),
    }

    result += &format!("\n{}\n", "}");

    // emit another de-interleave memcpy, that reads linear memory and writes to interleaved memory
    // dst is always 8-aligned, so we just have to check buf_len_bytes
    result += &format!("\n{}\n",
        "void ___private_memcpy_cpu2gpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id, uint read_idx, uint thread_idx, local ulong2 *scratch_space) {");
    result += &format!("\t{}\n", "global ulong *src_tmp = (global ulong*)(src);");
    result += &format!("\t{}\n", "ulong counter = 0;");
    result += &format!("\t{}\n", "if (buf_len_bytes > 8 && IS_ALIGNED_POW2((ulong)dst, 8) && IS_ALIGNED_POW2((ulong)src, 8)) {");
    result += &format!(
        "\t\t{}\n",
        "for (; counter < (buf_len_bytes-GET_POW2_OFFSET(buf_len_bytes, 8)); counter+=8) {"
    );

    result += &format!(
        "\t\t{};\n",
        emit_write_u64_aligned(
            "(ulong)(dst+counter)",
            "(ulong)(mem_start_dst)",
            "*src_tmp++",
            "warp_id"
        )
    );

    result += &format!("\t\t{}\n", "}");
    result += &format!("\t{}\n", "}");
    result += &format!(
        "\t{}\n",
        "global uchar *src_tmp_remaining = (global uchar*)(src_tmp);"
    );
    // finish the remaining bytes
    result += &format!("\t{}\n", "for (; counter < buf_len_bytes; counter++) {");

    result += &format!(
        "\t{};\n",
        emit_write_u8(
            "(ulong)(dst+counter)",
            "(ulong)(mem_start_dst)",
            "*src_tmp_remaining++",
            "warp_id"
        )
    );

    result += &format!("\t{}\n", "}");

    result += &format!("\n{}\n", "}");

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
pub fn emit_read_u8(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!("read_u8({}, {}, {}, read_idx)", addr, mem_start, warp_id)
}

pub fn emit_write_u8(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u8({}, {}, {}, {}, read_idx)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u16(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u16({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u16(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u16({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u32(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u32({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u32(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u32({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u64(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u64({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u64(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u64({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u16_aligned(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u16_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u16_aligned(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u16_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u32_aligned(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u32_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_read_u32_fast(offset: &str, base: &str) -> String {
    format!("read_u32_fast({}, {})", offset, base)
}

pub fn emit_write_u32_fast(offset: &str, base: &str, value: &str) -> String {
    format!("write_u32_fast({}, {}, {})", offset, base, value)
}

pub fn emit_read_u64_fast(offset: &str, base: &str) -> String {
    format!("read_u64_fast({}, {})", offset, base)
}

pub fn emit_write_u64_fast(offset: &str, base: &str, value: &str) -> String {
    format!("write_u64_fast({}, {}, {})", offset, base, value)
}

pub fn emit_write_u32_aligned(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u32_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u64_aligned(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u64_aligned({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u64_aligned(addr: &str, mem_start: &str, value: &str, warp_id: &str) -> String {
    format!(
        "write_u64_aligned({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u16_aligned_checked(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u16_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u16_aligned_checked(
    addr: &str,
    mem_start: &str,
    value: &str,
    warp_id: &str,
) -> String {
    format!(
        "write_u16_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u32_aligned_checked(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u32_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u32_aligned_checked(
    addr: &str,
    mem_start: &str,
    value: &str,
    warp_id: &str,
) -> String {
    format!(
        "write_u32_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_read_u64_aligned_checked(addr: &str, mem_start: &str, warp_id: &str) -> String {
    format!(
        "read_u64_aligned_checked({}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, warp_id
    )
}

pub fn emit_write_u64_aligned_checked(
    addr: &str,
    mem_start: &str,
    value: &str,
    warp_id: &str,
) -> String {
    format!(
        "write_u64_aligned_checked({}, {}, {}, {}, read_idx, thread_idx, scratch_space)",
        addr, mem_start, value, warp_id
    )
}

pub fn emit_intra_vm_memcpy(
    src_addr: &str,
    src_mem_start: &str,
    dst_addr: &str,
    dst_mem_start: &str,
    heap_base: &str,
    buf_len_bytes: &str,
    warp_id: &str,
) -> String {
    format!(
        "___private_bulk_memcpy({}, {}, {}, {}, {}, {}, {}, read_idx, thread_idx, scratch_space);",
        src_addr, src_mem_start, dst_addr, dst_mem_start, heap_base, buf_len_bytes, warp_id
    )
}

pub fn emit_intra_vm_memfill(
    dst_addr: &str,
    dst_mem_start: &str,
    value: &str,
    heap_base: &str,
    buf_len_bytes: &str,
    warp_id: &str,
) -> String {
    format!(
        "___private_bulk_memfill({}, {}, {}, {}, {}, {}, read_idx, thread_idx, scratch_space);",
        dst_addr, dst_mem_start, value, heap_base, buf_len_bytes, warp_id
    )
}
