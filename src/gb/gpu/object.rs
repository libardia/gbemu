use crate::util::bit_flag;

pub const OBJECT_BYTE_SIZE: u16 = 4;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Object {
    pub address: u16,
    pub y: i16,
    pub x: i16,
    pub tile_index: u8,
    pub flags: u8,
}
impl Object {
    pub fn get_8x16_tile_indexes(&self) -> (u8, u8) {
        let tile1 = self.tile_index & 0xFE;
        let tile2 = self.tile_index | 0x01;
        (tile1, tile2)
    }

    bit_flag!(get => get_under_bg, flags, 7);
    bit_flag!(get => get_y_flip, flags, 6);
    bit_flag!(get => get_x_flip, flags, 5);
    bit_flag!(get => get_palette, flags, 4);
}
