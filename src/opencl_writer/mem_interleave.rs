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
 * 
 *  The offset calc is:
 *  (offset = address - mem_start_addr)
 *  Virtual Address = (offset * NUM_THREADS) + warp_idx + mem_start_addr
 *  
 *  ex: if you are in thread 0, and you write to 0, and then 1
 *  the physical addresses are first 0, and then address+NUM_THREADS, with each
 *  subsequent byte being 1 stride of NUM_THREADS away
 * 
 *  We expect NUM_THREADS to be defined at compile time with the macro NUM_THREADS
 * 
 *  We also have to split multi-byte reads into multiple calls, in little-endian format
 * 
 */

pub fn generate_read_write_calls(writer: &opencl_writer::OpenCLCWriter, interleave: u32, debug: bool) -> String {
    let mut result = String::from("");
    // we need the warp id to generate the interleave
    // the write functions

    // TODO: switch between inlined read funcs
    result += &format!("\n{}\n",
                        "inline void write_u8(ulong addr, ulong mem_start, uchar value, uint warp_id) {");

    match interleave {
        0 => {
            result += &format!("\t{}",
                                "*((global uchar*)addr) = value;");
        },
        1 => {
            result += &format!("\t{}",
                                "*((global uchar*)((addr-mem_start)*(NUM_THREADS) + warp_id + mem_start)) = value;")
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u16(ulong addr, ulong mem_start, ushort value, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "*((ushort*)addr) = value;");
        },
        1 => {
            // write the lower byte first
            result += &format!("\t{}\n",
                                "write_u8(addr, mem_start, value & 0xFF, warp_id);");
            // now write the upper byte
            result += &format!("\t{}",
                                "write_u8((ulong)(((char*)addr)+1), mem_start, (value >> 8) & 0xFF, warp_id);");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u32(ulong addr, ulong mem_start, uint value, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "*((uint*)addr) = value;");
        },
        1 => {
            // write the lower byte first
            result += &format!("\t{}\n",
                                "write_u16(addr, mem_start, value & 0xFFFF, warp_id);");
            // now write the upper byte
            result += &format!("\t{}",
                                "write_u16((ulong)(((char*)addr)+2), mem_start, (value >> 16) & 0xFFFF, warp_id);");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    result += &format!("\n{}\n",
                        "inline void write_u64(ulong addr, ulong mem_start, ulong value, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "*((ulong*)addr) = value;");
        },
        1 => {
            // write the lower byte first
            result += &format!("\t{}\n",
                                "write_u32(addr, mem_start, value & 0xFFFFFFFF, warp_id);");
            // now write the upper byte
            result += &format!("\t{}",
                                "write_u32((ulong)(((char*)addr)+4), mem_start, (value >> 32) & 0xFFFFFFFF, warp_id);");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    // the read functions
    
    result += &format!("\n{}\n",
                        "inline uchar read_u8(ulong addr, ulong mem_start, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "return *((global uchar*)addr);");
        },
        1 => {
            result += &format!("\t{}",
                                "return *((global uchar*)((addr-mem_start)*NUM_THREADS + warp_id + mem_start));");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");



    result += &format!("\n{}\n",
                        "inline ushort read_u16(ulong addr, ulong mem_start, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "return *((ushort*)addr);");
        },
        1 => {
            // use a local variable to store the result as we perform the reads
            // we have to read in the reverse order!!! (high bits then low bits)
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
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline uint read_u32(ulong addr, ulong mem_start, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "return *((uint*)addr);");
        },
        1 => {
            // use a local variable to store the result as we perform the reads
            result += &format!("\t{}\n",
                                "uint temp = 0;");
            result += &format!("\t{}\n",
                                "temp += read_u16((ulong)(((char*)addr)+2), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                                "temp = temp << 16;");
            result += &format!("\t{}\n",
                                "temp += read_u16(addr, mem_start, warp_id);");
            result += &format!("\t{}",
                                "return temp;");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}",
                        "}");

    result += &format!("\n{}\n",
                        "inline ulong read_u64(ulong addr, ulong mem_start, uint warp_id) {");
    match interleave {
        0 => {
            result += &format!("\t{}",
                                "return *((ulong*)addr);");
        },
        1 => {
            // use a local variable to store the result as we perform the reads
            result += &format!("\t{}\n",
                                "ulong temp = 0;");
            result += &format!("\t{}\n",
                                "temp += read_u32((ulong)(((char*)addr)+4), mem_start, warp_id);");
            // bitshift over to make room for the next byte
            result += &format!("\t{}\n",
                                "temp = temp << 32;");
            result += &format!("\t{}\n",
                                "temp += read_u32(addr, mem_start, warp_id);");
            result += &format!("\t{}",
                                "return temp;");
        }
        _ => panic!("Unsupported read/write interleave"),
    }
    result += &format!("\n{}\n",
                        "}");

    // emit a memcpy function as well, for utility purposes
    result += &format!("\n{}\n",
        "void ___private_memcpy(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id) {");

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
        "void ___private_memcpy_gpu2cpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id) {");
    result += &format!("\t{}\n",
                       "char *dst_tmp = (char*)(mem_start_dst);");
    result += &format!("\t{}\n",
                       "for (uint idx = 0; idx < buf_len_bytes; idx++) {");

    result += &format!("\t{} = {};\n",
                       "*dst_tmp++",
                       &emit_read_u8("(ulong)(src+idx)", "(ulong)(mem_start_src)", "warp_id"));

    result += &format!("\t{}\n",
                       "}");
    
    result += &format!("\n{}\n",
                       "}");   

    // emit another de-interleave memcpy, that reads linear memory and writes to interleaved
    // memory
    result += &format!("\n{}\n",
        "void ___private_memcpy_cpu2gpu(ulong src, ulong mem_start_src, ulong dst, ulong mem_start_dst, ulong buf_len_bytes, uint warp_id) {");
    result += &format!("\t{}\n",
                       "char *src_tmp = (char*)(mem_start_src);");
 
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

    result
}

/*
 * These are compiler-internal utility functions for emitting code
 */
pub fn emit_read_u8(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u8({}, {}, {})", addr, mem_start, warp_id)
}

pub fn emit_read_u16(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u16({}, {}, {})", addr, mem_start, warp_id)
}

pub fn emit_write_u8(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u8({}, {}, {}, {})", addr, mem_start, value, warp_id)
}

pub fn emit_write_u16(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u16({}, {}, {}, {})", addr, mem_start, value, warp_id)
}

pub fn emit_read_u32(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u32({}, {}, {})", addr, mem_start, warp_id)
}

pub fn emit_write_u32(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u32({}, {}, {}, {})", addr, mem_start, value, warp_id)
}

pub fn emit_read_u64(addr: &str , mem_start: &str, warp_id: &str) -> String {
    format!("read_u64({}, {}, {})", addr, mem_start, warp_id)
}

pub fn emit_write_u64(addr: &str , mem_start: &str, value: &str, warp_id: &str) -> String {
    format!("write_u64({}, {}, {}, {})", addr, mem_start, value, warp_id)
}
