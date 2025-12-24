use std::fs;

use log::debug;

use crate::{
    gb::{mbc::MBC, mmu::mbc_rom_only::RomOnlyMBC},
    mem_region::header_data::CART_TYPE,
};

use super::MMU;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum CartType {
    ROM_ONLY,
    MBC1,
    MBC1_RAM,
    MBC1_RAM_BATTERY,
    MBC2,
    MBC2_BATTERY,
    ROM_RAM,
    ROM_RAM_BATTERY,
    MMM01,
    MMM01_RAM,
    MMM01_RAM_BATTERY,
    MBC3_TIMER_BATTERY,
    MBC3_TIMER_RAM_BATTERY,
    MBC3,
    MBC3_RAM,
    MBC3_RAM_BATTERY,
    MBC5,
    MBC5_RAM,
    MBC5_RAM_BATTERY,
    MBC5_RUMBLE,
    MBC5_RUMBLE_RAM,
    MBC5_RUMBLE_RAM_BATTERY,
    MBC6,
    MBC7_SENSOR_RUMBLE_RAM_BATTERY,
    POCKET_CAMERA,
    BANDAI_TAMA5,
    HUC3,
    HUC1_RAM_BATTERY,
}
impl From<u8> for CartType {
    fn from(value: u8) -> Self {
        use CartType::*;
        match value {
            0x00 => ROM_ONLY,
            0x01 => MBC1,
            0x02 => MBC1_RAM,
            0x03 => MBC1_RAM_BATTERY,
            0x05 => MBC2,
            0x06 => MBC2_BATTERY,
            0x08 => ROM_RAM,
            0x09 => ROM_RAM_BATTERY,
            0x0B => MMM01,
            0x0C => MMM01_RAM,
            0x0D => MMM01_RAM_BATTERY,
            0x0F => MBC3_TIMER_BATTERY,
            0x10 => MBC3_TIMER_RAM_BATTERY,
            0x11 => MBC3,
            0x12 => MBC3_RAM,
            0x13 => MBC3_RAM_BATTERY,
            0x19 => MBC5,
            0x1A => MBC5_RAM,
            0x1B => MBC5_RAM_BATTERY,
            0x1C => MBC5_RUMBLE,
            0x1D => MBC5_RUMBLE_RAM,
            0x1E => MBC5_RUMBLE_RAM_BATTERY,
            0x20 => MBC6,
            0x22 => MBC7_SENSOR_RUMBLE_RAM_BATTERY,
            0xFC => POCKET_CAMERA,
            0xFD => BANDAI_TAMA5,
            0xFE => HUC3,
            0xFF => HUC1_RAM_BATTERY,
            _ => panic!("The value 0x{:0>2X} is invalid for cartridge type", value),
        }
    }
}

impl MMU {
    pub fn load_cart_from_file(&mut self, path: &str) {
        let bytes = fs::read(path).expect(format!("Failed to read file {}", path).as_str());
        self.load_cart_from_bytes(&bytes);
    }

    pub fn load_cart_from_bytes(&mut self, bytes: &[u8]) {
        use CartType::*;

        // Get cartridge type byte as enum
        let cart_type: CartType = bytes[CART_TYPE as usize].into();

        debug!("Cart type: {:?}", cart_type);

        // Make the appropriate MBC
        let mbc: Box<dyn MBC> = Box::new(match cart_type {
            ROM_ONLY => RomOnlyMBC::from_arr(bytes, false),
            MBC1 => todo!("Cart type {:?}", cart_type),
            MBC1_RAM => todo!("Cart type {:?}", cart_type),
            MBC1_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC2 => todo!("Cart type {:?}", cart_type),
            MBC2_BATTERY => todo!("Cart type {:?}", cart_type),
            ROM_RAM => RomOnlyMBC::from_arr(bytes, true),
            ROM_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MMM01 => todo!("Cart type {:?}", cart_type),
            MMM01_RAM => todo!("Cart type {:?}", cart_type),
            MMM01_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC3_TIMER_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC3_TIMER_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC3 => todo!("Cart type {:?}", cart_type),
            MBC3_RAM => todo!("Cart type {:?}", cart_type),
            MBC3_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC5 => todo!("Cart type {:?}", cart_type),
            MBC5_RAM => todo!("Cart type {:?}", cart_type),
            MBC5_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC5_RUMBLE => todo!("Cart type {:?}", cart_type),
            MBC5_RUMBLE_RAM => todo!("Cart type {:?}", cart_type),
            MBC5_RUMBLE_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            MBC6 => todo!("Cart type {:?}", cart_type),
            MBC7_SENSOR_RUMBLE_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
            POCKET_CAMERA => todo!("Cart type {:?}", cart_type),
            BANDAI_TAMA5 => todo!("Cart type {:?}", cart_type),
            HUC3 => todo!("Cart type {:?}", cart_type),
            HUC1_RAM_BATTERY => todo!("Cart type {:?}", cart_type),
        });

        // Set MBC to the MMU
        self.mbc = Some(mbc);
    }
}
