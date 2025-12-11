use log::info;

pub fn test() {
    info!("test() from mmu.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_test() {
        test();
    }
}
