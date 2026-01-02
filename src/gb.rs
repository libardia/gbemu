use derive_new::new;
use immut::Immutable;

#[derive(new)]
pub struct GameBoy {
    #[new(into)]
    options: Immutable<u8>, // TODO: placeholder
}

impl GameBoy {
    pub fn run(&mut self) {}
}
