use log::info;

pub fn test() {
    info!("test() from cpu.rs");
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_test() {
        info!("testing:");
        test();
    }
}
