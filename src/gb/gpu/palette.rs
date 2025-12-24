use std::ops::Index;

use super::color_id::ColorID;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Palette([ColorID; 4]);
impl From<u8> for Palette {
    fn from(value: u8) -> Self {
        let mut colors = [ColorID::Color0; 4];

        let mut m1 = 0b10;
        let mut m2 = 0b_1;
        for i in 0..colors.len() {
            // Set the color from the bits of the mask
            colors[i] = match (value & m1 != 0, value & m2 != 0) {
                (false, false) => ColorID::Color0,
                (false, true) => ColorID::Color1,
                (true, false) => ColorID::Color2,
                (true, true) => ColorID::Color3,
            };

            // Shift the masks for the next color
            m1 <<= 2;
            m2 <<= 2;
        }

        Self(colors)
    }
}
impl Index<ColorID> for Palette {
    type Output = ColorID;

    fn index(&self, id: ColorID) -> &Self::Output {
        &self.0[id as usize]
    }
}
