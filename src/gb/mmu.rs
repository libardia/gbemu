use crate::gb::GameBoy;

#[derive(Default)]
pub struct MMU {
    mem: Vec<u8>,
}

impl MMU {
    pub fn new() -> Self {
        let mut mmu = MMU::default();
        // TODO: init
        mmu.mem = vec![0xFF; 0x10000];
        mmu
    }

    pub fn read(ctx: &mut GameBoy, address: u16) -> u8 {
        ctx.mmu.mem[address as usize]
    }

    pub fn write(ctx: &mut GameBoy, address: u16, byte: u8) {
        ctx.mmu.mem[address as usize] = byte;
    }

    /* #region For test purposes */
    #[cfg(test)]
    pub fn force_read(ctx: &mut GameBoy, address: u16) -> u8 {
        ctx.mmu.mem[address as usize]
    }

    #[cfg(test)]
    pub fn force_write(ctx: &mut GameBoy, address: u16, byte: u8) {
        ctx.mmu.mem[address as usize] = byte;
    }
    /* #endregion */
}
