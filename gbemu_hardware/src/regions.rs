#[derive(Debug, Default, PartialEq, Eq)]
pub struct MemoryRegion {
    pub begin: u16,
    pub end: u16,
    size: usize,
}

impl MemoryRegion {
    pub const fn new(begin: u16, end: u16) -> Self {
        Self {
            begin,
            end,
            size: (end - begin) as usize + 1,
        }
    }

    pub const fn contains(&self, address: u16) -> bool {
        self.begin <= address && self.end >= address
    }

    pub const fn local_address(&self, address: u16) -> u16 {
        address - self.begin
    }

    pub const fn size(&self) -> u16 {
        self.size as u16
    }

    pub const fn usize(&self) -> usize {
        self.size
    }
}

#[derive(Debug)]
pub struct MappedMemoryRegion {
    pub region: MemoryRegion,
    mem: Vec<u8>,
}

impl MappedMemoryRegion {
    pub fn new(region: MemoryRegion) -> MappedMemoryRegion {
        let mem = vec![0xFF; region.size];
        MappedMemoryRegion { region, mem }
    }

    pub fn local_address(&self, address: u16) -> u16 {
        self.region.local_address(address)
    }

    pub fn get(&self, address: u16) -> u8 {
        self.mem[self.local_address(address) as usize]
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let local = self.local_address(address);
        self.mem[local as usize] = value;
    }
}

macro_rules! def_regions {
    ($($name:ident: $begin:expr, $end:expr;)+) => {
        $(pub const $name: MemoryRegion = MemoryRegion::new($begin, $end);)+
    };
}

def_regions! {
    ALL_MEM:        0x0000, 0xFFFF;
      ROM_SPACE:    0x0000, 0x7FFF;
        HEADER:     0x0100, 0x014F;
      VRAM:         0x8000, 0x9FFF;
      CART_RAM:     0xA000, 0xBFFF;
      WORK_RAM:     0xC000, 0xDFFF;
      ECHO_RAM:     0xE000, 0xFDFF;
      OAM:          0xFE00, 0xFE9F;
      UNUSABLE:     0xFEA0, 0xFEFF;
      IO_REGS:      0xFF00, 0xFF7F;
      HIGH_RAM:     0xFF80, 0xFFFE;
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_contains() {
        let reg = MemoryRegion::new(5, 10);
        assert!(!reg.contains(4));
        assert!(reg.contains(5));
        assert!(reg.contains(7));
        assert!(reg.contains(10));
        assert!(!reg.contains(11));
    }

    #[test]
    fn test_local_address() {
        let reg = MappedMemoryRegion::new(MemoryRegion::new(5, 10));
        assert_eq!(reg.local_address(5), 0);
        assert_eq!(reg.local_address(7), 2);
        assert_eq!(reg.local_address(10), 5);
    }

    #[test]
    fn test_getset() {
        let mut reg = MappedMemoryRegion::new(MemoryRegion::new(5, 10));
        reg.set(7, 0xAD);
        assert_eq!(reg.get(6), 0xDE);
        assert_eq!(reg.get(7), 0xAD);
        reg.set(6, 0xBE);
        reg.set(7, 0xEF);
        assert_eq!(reg.get(6), 0xBE);
        assert_eq!(reg.get(7), 0xEF);
    }
}
