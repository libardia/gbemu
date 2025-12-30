use crate::{
    error_panic,
    gb::hardware::{
        HardwareInit,
        audio::Audio,
        cartridge::{Cartridge, load_cart},
        graphics::Graphics,
        input::Input,
        memory::Memory,
        processor::Processor,
        serial::Serial,
        timer::Timer,
    },
    has_opt, number_type,
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
            skip_boot: !has_opt!(opts, DO_BOOT),
            meta_inst: has_opt!(opts, META_INST),
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
        gb.cart.init();
        Processor::init(&mut gb);
        Memory::init(&mut gb);
        Graphics::init(&mut gb);
        Timer::init(&mut gb);
        Input::init(&mut gb);
        Audio::init(&mut gb);
        Serial::init(&mut gb);

        gb
    }

    pub fn run(&mut self) {
        while !self.exit {
            let time = Processor::step(self);
            //TODO: run: update everything else
        }

        info!("Main loop ended. Shutting down.");
    }

    pub fn stop(&mut self) {
        self.exit = true;
    }
}
