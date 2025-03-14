use crate::{
    mem_region::io_regs::*,
    util::{bit_flag, either},
};

use super::GPU;

impl GPU {
    // LCD control flags
    bit_flag!(get => get_enabled, io_lcdc, 7);
    bit_flag!(get => get_window_tile_map, io_lcdc, 6);
    bit_flag!(get => get_window_enabled, io_lcdc, 5);
    bit_flag!(get => get_bg_window_tiles, io_lcdc, 4);
    bit_flag!(get => get_enget_bg_tile_mapabled, io_lcdc, 3);
    bit_flag!(get => get_obj_size, io_lcdc, 2);
    bit_flag!(get => get_obj_enabled, io_lcdc, 1);
    bit_flag!(get => get_bg_window_enabled, io_lcdc, 0);

    // STAT flags
    bit_flag!(get => get_lyc_interrupt, io_stat, 6);
    bit_flag!(get => get_mode_2_interrupt, io_stat, 5);
    bit_flag!(get => get_mode_1_interrupt, io_stat, 4);
    bit_flag!(get => get_mode_0_interrupt, io_stat, 3);
    bit_flag!(set => set_lyc_eq_ly, io_stat, 2);

    // Interrupts
    bit_flag!(set => set_stat_interrupt, interrupt_requests, 1);
    bit_flag!(set => set_vblank_interrupt, interrupt_requests, 0);

    pub(super) fn load_regs(&mut self) {
        // Load IO registers
        self.io_lcdc = self.mmu_read_byte(REG_LCDC);
        self.io_stat = self.mmu_read_byte(REG_STAT);
        self.compare_line = self.mmu_read_byte(REG_LYC);
        self.bg_palette = self.mmu_read_byte(REG_BGP).into();
        self.obj_palette_0 = self.mmu_read_byte(REG_OBP0).into();
        self.obj_palette_1 = self.mmu_read_byte(REG_OBP1).into();
        self.viewport_y = self.mmu_read_byte(REG_SCY);
        self.viewport_x = self.mmu_read_byte(REG_SCX);
        self.window_y = self.mmu_read_byte(REG_WY);
        self.window_xp7 = self.mmu_read_byte(REG_WX);
        self.interrupt_requests = self.mmu_read_byte(REG_IF);
    }

    pub(super) fn set_regs(&mut self) {
        // Set LYC == LY bit in IO_STAT
        let lyc_eq_ly = self.ds.current_line == self.compare_line;
        self.set_lyc_eq_ly(lyc_eq_ly);

        // Request LYC == LY STAT interrupt if we've been asked to
        if self.get_lyc_interrupt() && lyc_eq_ly {
            self.set_stat_interrupt(true);
        }

        // Set mode in IO_STAT (always report 0 when disabled)
        let v = either!(self.get_enabled() => self.ds.mode as u8; 0);
        self.io_stat = (self.io_stat & 0b1111_1100) | v;

        // Set registers
        self.mmu_write_byte(REG_STAT, self.io_stat);
        self.mmu_write_byte(REG_LY, self.ds.current_line);
        self.mmu_write_byte(REG_IF, self.interrupt_requests);
    }
}
