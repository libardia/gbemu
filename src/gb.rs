use std::{cell::RefCell, fs, rc::Rc};

use log::info;

use crate::{cpu::CPU, hex::HexU16, mem_region::io_regs, mmu::MMU, ppu::PPU};

pub const DEFAULT_FPS: f32 = 59.737156;

pub struct GB<C, P, M: MMU>
where
    C: CPU<M>,
    P: PPU<M>,
{
    cpu: C,
    ppu: P,
    mmu: Rc<RefCell<M>>,
}

impl<C, P, M: MMU> GB<C, P, M>
where
    C: CPU<M>,
    P: PPU<M>,
{
    /// Create a new GameBoy with the default frame rate.
    pub fn new(scale: usize) -> Self {
        Self::new_with_fps(scale, DEFAULT_FPS)
    }

    /// Create a new GameBoy with the given frame rate.
    pub fn new_with_fps(scale: usize, frame_rate: f32) -> Self {
        let mmu = Rc::new(RefCell::new(M::new()));
        let cpu = C::new(mmu.clone());
        let gpu = P::new(mmu.clone(), scale, frame_rate);
        Self { cpu, ppu: gpu, mmu }
    }

    /// Set debug mode. When `true`, breakpoints and debug printing is enabled.
    pub fn set_debug_mode(&mut self, mode: bool) {
        self.cpu.set_debug_mode(mode);
    }

    /// Set breakpoints. Breakpoints only have an effect in debug mode.
    pub fn set_breakpoints(&mut self, breakpoints: &[u16]) {
        self.cpu.set_breakpoints(breakpoints);
    }

    /// Boot as normal, including the boot ROM.
    pub fn boot(&mut self) {
        info!("Performing normal boot (boot ROM is run).");
        self.execute();
    }

    /// Skip the boot ROM: Immediately unmap the boot ROM and begin execution at 0x100.
    pub fn quick_boot(&mut self) {
        info!("Performing quick boot (boot ROM is skipped, and execution begins at 0x100).");
        self.mmu.borrow_mut().set(io_regs::BANK, 1);
        self.execute_at(0x100);
    }

    /// Load a program into ROM.
    pub fn load_rom(&self, load_at: u16, rom: &[u8]) {
        info!(
            "Loading ROM of size {} to address 0x{:0>4X}",
            rom.len(),
            load_at
        );
        self.mmu.borrow_mut().load_rom(load_at, rom);
    }

    /// Load a program into ROM from file.
    pub fn load_rom_file(&self, file_path: &str) {
        info!("Loading ROM from file: {}", file_path);
        let bytes =
            fs::read(file_path).expect(format!("Failed to read file {}", file_path).as_str());
        self.load_rom(0, &bytes);
    }

    /// Begin emulation, starting execution at 0.
    pub fn execute(&mut self) {
        self.execute_at(0);
    }

    /// Begin emulation, starting execution at the given address.
    pub fn execute_at(&mut self, address: u16) {
        info!("Beginning execution at address 0x{address:0>4X}.");
        if self.cpu.get_debug_mode() {
            info!(
                "Debug mode is on, and breakpoints are: {:?}",
                self.cpu
                    .get_breakpoints()
                    .iter()
                    .map(|&x| HexU16(x))
                    .collect::<Vec<_>>()
            );
        } else {
            info!("Debug mode is off.");
        }
        self.cpu.set_pc(address);
        while !self.cpu.should_terminate() && !self.ppu.should_terminate() {
            let dm = self.cpu.step();
            self.ppu.step_dots(dm);
        }
    }
}
