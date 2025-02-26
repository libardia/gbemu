#[derive(Debug)]
pub struct MMU {
    pub mem: [u8; 0xFFFF],
}

impl MMU {
    pub fn new() -> Self {
        MMU { mem: [0; 0xFFFF] }
    }
}