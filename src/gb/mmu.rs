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
}
