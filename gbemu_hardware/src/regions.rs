use std::ops::Add;

pub struct MemRegion {
    pub begin: u16,
    pub end: u16,
}

impl MemRegion {
    fn size<T>(&self) -> T
    where
        T: Add<Output = T>,
        u16: Into<T>,
    {
        (self.end - self.begin).into() + 1.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_u16_size() {
        let reg = MemRegion { begin: 4, end: 9 };
        let res: u16 = reg.size();
        assert_eq!(res, 6u16);
    }

    #[test]
    fn test_usize_size() {
        let reg = MemRegion {
            begin: 0x0000,
            end: 0xFFFF,
        };
        let res: usize = reg.size();
        assert_eq!(res, 0x10000usize);
    }
}
