use crate::gb::GameBoy;

#[derive(Default)]
pub struct MMU {
    mem: Vec<u8>,
}

impl MMU {
    pub fn new() -> Self {
        let mut mmu = MMU::default();
        // TODO: init
        mmu.mem = vec![0xFF; 0xFFFF];
        mmu
    }

    pub fn read(ctx: &mut GameBoy, address: u16) -> u8 {
        ctx.mmu.mem[address as usize]
    }

    pub fn write(ctx: &mut GameBoy, address: u16, byte: u8) {
        ctx.mmu.mem[address as usize] = byte;
    }
}
