use derive_new::new;

use crate::{immut::Immutable, options::Options};

#[derive(new)]
pub struct GameBoy {
    #[new(into)]
    options: Immutable<Options>,
}

impl GameBoy {
    pub fn run(&mut self) {}
}
