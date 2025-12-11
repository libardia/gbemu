use crate::{cpu::CPU, mmu::MMU};

#[allow(dead_code)]
mod cpu;
mod mmu;

struct GameBoy {
    mmu: MMU,
    cpu: CPU,
}

/* #region Tests */
#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
/* #endregion */
