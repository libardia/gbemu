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
    options::{DO_BOOT, META_INST},
};
use getopts::Matches;
use log::info;

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

    exit: bool,
    meta_inst: bool,
    skip_boot: bool,
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
            skip_boot: !opts.opt_defined(DO_BOOT.long_name),
            meta_inst: opts.opt_defined(META_INST.long_name),
            exit: false,

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
        // TODO: init cart
        Processor::init(&mut gb);
        Memory::init(&mut gb);
        // TODO: init graphics
        // TODO: init timer
        // TODO: init input
        // TODO: init audio
        // TODO: init serial

        gb
    }

    pub fn run(&mut self) {
        //TODO: run
        while !self.exit {
            let time = Processor::step(self);
        }

        info!("Main loop ended. Shutting down.");
    }

    pub fn stop(&mut self) {
        self.exit = true;
    }
}
