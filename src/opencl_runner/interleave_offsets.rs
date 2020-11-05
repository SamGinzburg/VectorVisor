pub struct Interleave {}

impl Interleave {
    pub fn read_u32(offset: u32, num_threads: u32, vm_id: u32) -> usize {
        (offset * num_threads + vm_id) as usize
    }
}