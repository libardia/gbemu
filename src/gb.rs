use crate::{
    gb::hardware::{
        audio::Audio,
        cartridge::{Cartridge, load_cart},
        graphics::Graphics,
        input::Input,
        memory::Memory,
        processor::Processor,
        serial::Serial,
        timer::Timer,
    },
    number_type,
};

mod hardware;
mod macros;
mod regions;
mod registers;

pub struct GameBoy {
    cart: Box<dyn Cartridge>,
    cpu: Processor,
    mem: Memory,
    gfx: Graphics,
    timer: Timer,
    input: Input,
    aud: Audio,
    serial: Serial,
}

number_type!(MTime: u16);
number_type!(Dot: u16);

impl GameBoy {
    pub fn new(rom_path: &str) -> Self {
        // Make gb
        let mut gb = Self {
            cart: load_cart(rom_path),
            cpu: Processor::default(),
            mem: Memory::default(),
            gfx: Graphics::default(),
            timer: Timer::default(),
            input: Input::default(),
            aud: Audio::default(),
            serial: Serial::default(),
        };

        // Initialize
        gb.reset();

        gb
    }

    pub fn reset(&mut self) {
        // TODO: init
    }

    pub fn run(&mut self) {
        //TODO: run
    }
}
