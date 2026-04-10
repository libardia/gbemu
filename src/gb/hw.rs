pub trait HardwareInterface {
    fn init(&mut self);
    fn read(&mut self, address: u16) -> u8;
    fn write(&mut self, address: u16, byte: u8);
}
