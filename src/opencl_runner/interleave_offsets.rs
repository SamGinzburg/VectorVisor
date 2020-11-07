pub struct Interleave {}

impl Interleave {
    pub fn read_u8(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u8 {
        buffer[(offset * num_threads + vm_id) as usize]
    }

    pub fn read_16(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u16 {
        let mut temp: u16 = 0;
        temp += Interleave::read_u8(buffer, offset+1, num_threads, vm_id) as u16;
        temp = temp << 8;
        temp += Interleave::read_u8(buffer, offset, num_threads, vm_id) as u16;
        temp
    }

    pub fn read_u32(buffer: &mut [u8], offset: u32, num_threads: u32, vm_id: u32) -> u32 {
        let mut temp: u32 = 0;
        temp += Interleave::read_16(buffer, offset+2, num_threads, vm_id) as u32;
        temp = temp << 16;
        temp += Interleave::read_16(buffer, offset, num_threads, vm_id) as u32;
        temp
    }
}