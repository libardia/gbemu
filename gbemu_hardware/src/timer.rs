use crate::HardwareInterface;

#[derive(Debug, Default)]
pub struct Timer {
    // TODO: Timer
    system_timer: u16,
}

impl HardwareInterface for Timer {
    fn read(&self, address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, address: u16, value: u8) {
        todo!()
    }
}

impl Timer {}
