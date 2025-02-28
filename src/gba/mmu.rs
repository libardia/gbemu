const MEM_SIZE: usize = 0xFFFF;

#[derive(Debug)]
pub struct MMU {
    pub mem: [u8; MEM_SIZE],
}

impl MMU {
    pub fn new() -> Self {
        MMU { mem: [0; MEM_SIZE] }
    }

    pub fn reset(&mut self) {
        self.mem = [0; MEM_SIZE];
    }

    // 8-bit ======================================================================================

    pub fn read_byte(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    // 16-bit =====================================================================================

    // NOTE: LITTLE-ENDIAN: second byte of value is stored in address, first byte is stored in
    // address + 1. This is very important because virtual 16-bit registers are BIG-ENDIAN.

    pub fn read_word(&self, address: u16) -> u16 {
        let ls = self.read_byte(address);
        let ms = self.read_byte(address + 1);
        ((ms as u16) << 8) + (ls as u16)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        self.write_byte(address, (value & 0xFF) as u8);
        self.write_byte(address, ((value & 0xFF00) >> 8) as u8);
    }
}
