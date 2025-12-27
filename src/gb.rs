use crate::{
    error_panic,
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
use getopts::Matches;

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
    opts: Matches,
}

number_type!(MTime: u16);
number_type!(Dot: u16);

impl GameBoy {
    pub fn new(opts: Matches) -> Self {
        // Make sure a ROM file is provided
        if opts.free.len() < 1 {
            error_panic!("No ROM file provided.");
        }

        // Make gb
        let mut gb = Self {
            cart: load_cart(&opts.free[0]),
            cpu: Processor::default(),
            mem: Memory::default(),
            gfx: Graphics::default(),
            timer: Timer::default(),
            input: Input::default(),
            aud: Audio::default(),
            serial: Serial::default(),
            opts,
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
        loop {
            Processor::step(self);
        }
    }
}
