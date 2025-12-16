use std::ops::Add;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MemoryRegion {
    begin: u16,
    end: u16,
}

impl MemoryRegion {
    pub fn begin(&self) -> u16 {
        self.begin
    }

    pub fn end(&self) -> u16 {
        self.begin
    }

    pub fn contains(&self, address: u16) -> bool {
        self.begin <= address && self.end >= address
    }

    pub fn size<T>(&self) -> T
    where
        T: Add<Output = T>,
        u16: Into<T>,
    {
        (self.end - self.begin).into() + 1.into()
    }
}

pub struct MappedMemoryRegion {
    region: MemoryRegion,
    mem: Vec<u8>,
}

impl MappedMemoryRegion {
    pub fn region(&self) -> &MemoryRegion {
        &self.region
    }

    pub fn local_address(&self, address: u16) -> u16 {
        address - self.region.begin
    }

    pub fn get(&self, address: u16) -> u8 {
        self.mem[self.local_address(address) as usize]
    }

    pub fn set(&mut self, address: u16, value: u8) {
        let local = self.local_address(address);
        self.mem[local as usize] = value;
    }
}

pub fn map_region(region: MemoryRegion) -> MappedMemoryRegion {
    let mem = vec![0xFF; region.size()];
    MappedMemoryRegion { region, mem }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_u16_size() {
        let reg = MemoryRegion { begin: 4, end: 9 };
        let res: u16 = reg.size();
        assert_eq!(res, 6u16);
    }

    #[test]
    fn test_usize_size() {
        let reg = MemoryRegion {
            begin: 0x0000,
            end: 0xFFFF,
        };
        let res: usize = reg.size();
        assert_eq!(res, 0x10000usize);
    }

    #[test]
    fn test_contains() {
        let reg = MemoryRegion { begin: 5, end: 10 };
        assert!(!reg.contains(4));
        assert!(reg.contains(5));
        assert!(reg.contains(7));
        assert!(reg.contains(10));
        assert!(!reg.contains(11));
    }

    #[test]
    fn test_local_address() {
        let reg = map_region(MemoryRegion { begin: 5, end: 10 });
        assert_eq!(reg.local_address(5), 0);
        assert_eq!(reg.local_address(7), 2);
        assert_eq!(reg.local_address(10), 5);
    }

    #[test]
    fn test_getset() {
        let mut reg = map_region(MemoryRegion { begin: 5, end: 10 });
        reg.set(6, 0xDE);
        reg.set(7, 0xAD);
        assert_eq!(reg.get(6), 0xDE);
        assert_eq!(reg.get(7), 0xAD);
        reg.set(6, 0xBE);
        reg.set(7, 0xEF);
        assert_eq!(reg.get(6), 0xBE);
        assert_eq!(reg.get(7), 0xEF);
    }
}
