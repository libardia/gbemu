use crate::mem_region::MemoryRegion;

#[derive(Debug)]
pub struct MappedRegion {
    region: MemoryRegion,
    buffer: Vec<u8>,
}
impl MappedRegion {
    pub fn new(region: MemoryRegion) -> Self {
        Self {
            region,
            buffer: vec![0; region.usize()],
        }
    }

    pub fn contains(&self, address: u16) -> bool {
        self.region.contains(address)
    }

    pub fn get(&self, address: u16) -> u8 {
        self.buffer[self.region.uoffset(address)]
    }

    pub fn set(&mut self, address: u16, value: u8) {
        self.buffer[self.region.uoffset(address)] = value;
    }
}
