use std::convert::TryInto;

pub struct Interleave {}

impl Interleave {
    pub fn read_u8(
        buffer: &[u8],
        offset: u32,
        num_threads: u32,
        vm_id: u32,
        interleave: u32,
    ) -> u8 {
        let cell = offset / interleave;
        let cell_offset = offset % interleave;
        buffer[((cell * num_threads * interleave) + (vm_id * interleave) + cell_offset) as usize]
    }

    pub fn write_u8(
        buffer: &mut [u8],
        offset: u32,
        num_threads: u32,
        value: u8,
        vm_id: u32,
        interleave: u32,
    ) {
        let cell = offset / interleave;
        let cell_offset = offset % interleave;
        buffer[((cell * num_threads * interleave) + (vm_id * interleave) + cell_offset) as usize] =
            value;
    }

    pub fn read_u16(
        buffer: &[u8],
        offset: u32,
        num_threads: u32,
        vm_id: u32,
        interleave: u32,
    ) -> u16 {
        let mut temp: u16 = 0;
        temp += Interleave::read_u8(buffer, offset + 1, num_threads, vm_id, interleave) as u16;
        temp = temp << 8;
        temp += Interleave::read_u8(buffer, offset, num_threads, vm_id, interleave) as u16;
        temp
    }

    pub fn write_u16(
        buffer: &mut [u8],
        offset: u32,
        num_threads: u32,
        value: u16,
        vm_id: u32,
        interleave: u32,
    ) {
        Interleave::write_u8(
            buffer,
            offset,
            num_threads,
            (value & 0xFF).try_into().unwrap(),
            vm_id,
            interleave,
        );
        Interleave::write_u8(
            buffer,
            offset + 1,
            num_threads,
            ((value >> 8) & 0xFF).try_into().unwrap(),
            vm_id,
            interleave,
        );
    }

    pub fn read_u32(
        buffer: &[u8],
        offset: u32,
        num_threads: u32,
        vm_id: u32,
        interleave: u32,
    ) -> u32 {
        let mut temp: u32 = 0;
        temp += Interleave::read_u16(buffer, offset + 2, num_threads, vm_id, interleave) as u32;
        temp = temp << 16;
        temp += Interleave::read_u16(buffer, offset, num_threads, vm_id, interleave) as u32;
        temp
    }

    pub fn read_u64(
        buffer: &[u8],
        offset: u32,
        num_threads: u32,
        vm_id: u32,
        interleave: u32,
    ) -> u64 {
        let mut temp: u64 = 0;
        temp += Interleave::read_u32(buffer, offset + 4, num_threads, vm_id, interleave) as u64;
        temp = temp << 32;
        temp += Interleave::read_u32(buffer, offset, num_threads, vm_id, interleave) as u64;
        temp
    }

    pub fn read_u128(
        buffer: &[u8],
        offset: u32,
        num_threads: u32,
        vm_id: u32,
        interleave: u32,
    ) -> u128 {
        let mut temp: u128 = 0;
        temp += Interleave::read_u64(buffer, offset + 8, num_threads, vm_id, interleave) as u128;
        temp = temp << 64;
        temp += Interleave::read_u64(buffer, offset, num_threads, vm_id, interleave) as u128;
        temp
    }

    pub fn write_u32(
        buffer: &mut [u8],
        offset: u32,
        num_threads: u32,
        value: u32,
        vm_id: u32,
        interleave: u32,
    ) {
        Interleave::write_u16(
            buffer,
            offset,
            num_threads,
            (value & 0xFFFF).try_into().unwrap(),
            vm_id,
            interleave,
        );
        Interleave::write_u16(
            buffer,
            offset + 2,
            num_threads,
            ((value >> 16) & 0xFFFF).try_into().unwrap(),
            vm_id,
            interleave,
        );
    }
}
