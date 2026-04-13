use std::{
    fs::File,
    io::{Read, Result},
};

use log::info;

use crate::gb::{
    apu::APU,
    cart::{Cart, load_cart},
    cpu::CPU,
    inu::INU,
    mmu::MMU,
    ppu::PPU,
    sdu::SDU,
    tmu::TMU,
};

pub mod apu;
pub mod cart;
pub mod cpu;
pub mod hardware_interface;
pub mod inu;
pub mod mmu;
pub mod ppu;
pub mod sdu;
pub mod tmu;

pub struct GameBoy {
    pub mmu: MMU,
    pub cart: Box<dyn Cart>,

    pub cpu: CPU,
    pub ppu: PPU,
    pub tmu: TMU,
    pub inu: INU,
    pub apu: APU,
    pub sdu: SDU,

    pub debug_timer: u64,

    pub debug_isntructions: bool,
    pub skip_boot: bool,
    pub exit: bool,
}

impl GameBoy {
    pub fn new(cart_file: &str) -> Self {
        Self {
            mmu: MMU::new(),
            cart: load_cart(cart_file),

            cpu: CPU::new(),
            ppu: PPU::new(),
            tmu: TMU::new(),
            inu: INU::new(),
            apu: APU::new(),
            sdu: SDU::new(),

            debug_timer: Default::default(),

            debug_isntructions: false,
            skip_boot: false,
            exit: false,
        }
    }

    pub fn load_rom(&mut self, file_path: &str) {
        fn do_load(ctx: &mut GameBoy, file_path: &str) -> Result<()> {
            let mut f = File::open(file_path)?;
            f.read_exact(ctx.mmu.rom.as_mut_slice())
        }
        match do_load(self, file_path) {
            Ok(_) => info!("successfully loaded ROM '{file_path}'"),
            Err(e) => panic!("error opening ROM file: {e}"),
        }
    }

    pub fn run(&mut self) {
        while !self.exit {
            // CPU step will handle ticking all other hardware,
            // because instructions take variable amounts of time
            CPU::step(self);
        }

        info!("Terminating.")
    }

    fn m_tick(&mut self) {
        for _ in 0..4 {
            self.t_tick();
        }
    }

    fn t_tick(&mut self) {
        self.debug_timer += 1;
    }
}

#[cfg(test)]
mod tests {
    use test_log::test;

    use crate::{gb::GameBoy, testutil::dummy_ctx};

    #[test]
    fn test_m_tick() {
        let ctx = &mut dummy_ctx();
        assert_eq!(ctx.debug_timer, 0);
        ctx.m_tick();
        assert_eq!(ctx.debug_timer, 4);
        ctx.m_tick();
        assert_eq!(ctx.debug_timer, 8);
    }

    #[test]
    fn test_t_tick() {
        let ctx = &mut dummy_ctx();
        assert_eq!(ctx.debug_timer, 0);
        ctx.t_tick();
        assert_eq!(ctx.debug_timer, 1);
        ctx.t_tick();
        assert_eq!(ctx.debug_timer, 2);
    }

    #[test]
    fn test_load_rom() {
        let ctx = &mut dummy_ctx();
        ctx.load_rom("res/rom_ascending.bin");
        for i in 0..0x100 {
            assert_eq!(i as u8, ctx.mmu.rom.as_slice()[i]);
        }
        ctx.load_rom("res/rom_dummy.bin");
        for i in 0..0x100 {
            assert_eq!(0, ctx.mmu.rom.as_slice()[i]);
        }
    }

    #[test]
    #[should_panic]
    fn test_small_rom() {
        dummy_ctx().load_rom("res/rom_too_small.bin");
    }

    #[test]
    fn run_test() {
        let ctx = &mut dummy_ctx();
        ctx.debug_isntructions = true;
        ctx.load_rom("res/rom_run_test.bin");
        ctx.run();
    }
}
