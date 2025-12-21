use crate::cart::Cart;

mod regs;

pub struct MMU {
    pub cart: Box<dyn Cart>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;
}
