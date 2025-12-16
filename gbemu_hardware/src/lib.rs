use crate::{cpu::CPU, mmu::MMU};

#[allow(dead_code)]
mod cpu;
mod mmu;
mod regions;

#[derive(Default, Debug)]
struct GameBoy {
    mmu: MMU,
    cpu: CPU,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
