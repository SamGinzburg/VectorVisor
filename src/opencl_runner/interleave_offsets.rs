use std::convert::TryInto;

pub struct Interleave {}

impl Interleave {
    pub fn read_u8(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u8 {
        buffer[(offset * num_threads + vm_id) as usize]
    }

    pub fn write_u8(buffer: &mut [u8], offset: u32, num_threads: u32, value: u8, vm_id: u32) {
        buffer[(offset * num_threads + vm_id) as usize] = value;
    }

    pub fn read_16(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u16 {
        let mut temp: u16 = 0;
        temp += Interleave::read_u8(buffer, offset+1, num_threads, vm_id) as u16;
        temp = temp << 8;
        temp += Interleave::read_u8(buffer, offset, num_threads, vm_id) as u16;
        temp
    }

    pub fn write_u16(buffer: &mut [u8], offset: u32, num_threads: u32, value: u16, vm_id: u32) {
        Interleave::write_u8(buffer, offset, num_threads, (value & 0xFF).try_into().unwrap(), vm_id);
        Interleave::write_u8(buffer, offset+1, num_threads, ((value >> 8) & 0xFF).try_into().unwrap(), vm_id);
    }

    pub fn read_u32(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u32 {
        let mut temp: u32 = 0;
        temp += Interleave::read_16(buffer, offset+2, num_threads, vm_id) as u32;
        temp = temp << 16;
        temp += Interleave::read_16(buffer, offset, num_threads, vm_id) as u32;
        temp
    }

    pub fn read_u64(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u64 {
        let mut temp: u64 = 0;
        temp += Interleave::read_u32(buffer, offset+4, num_threads, vm_id) as u64;
        temp = temp << 32;
        temp += Interleave::read_u32(buffer, offset, num_threads, vm_id) as u64;
        temp
    }

    pub fn write_u32(buffer: &mut [u8], offset: u32, num_threads: u32, value: u32, vm_id: u32) {
        Interleave::write_u16(buffer, offset, num_threads, (value & 0xFFFF).try_into().unwrap(), vm_id);
        Interleave::write_u16(buffer, offset+2, num_threads, ((value >> 16) & 0xFFFF).try_into().unwrap(), vm_id);
    }
}