use crate::gb::GameBoy;

#[inline(always)]
pub fn nop(_ctx: &mut GameBoy) {
    // Do nothing
}

#[cfg(test)]
mod tests {
    use crate::step_test;

    use super::*;

    #[test]
    fn nop() {
        let ctx = &mut GameBoy::new();
        step_test! {
            ctx: ctx;
            code: 0x00, length: 1, cycles: 1
            // Nothing else to do!
        }
    }
}
