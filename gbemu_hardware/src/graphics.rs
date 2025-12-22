use crate::HardwareInterface;

#[derive(Debug, Default)]
pub struct Graphics {
    // TODO: Graphics
}

impl HardwareInterface for Graphics {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        todo!()
    }
}

impl Graphics {}
