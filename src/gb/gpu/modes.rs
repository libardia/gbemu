use crate::{
    mem_region::regions::{OAM, VRAM},
    util::either,
};

use super::{
    color_id::ColorID::{self, *},
    object::{Object, OBJECT_BYTE_SIZE},
    RenderMode::*,
    BASE_SCREEN_WIDTH, GPU, LINES_PER_DRAW, LINES_PER_FRAME, OAM_TIME, OBJECTS_PER_LINE,
    SCANLINE_TIME,
};

pub const NUM_OBJECTS: u16 = OAM.size() / OBJECT_BYTE_SIZE;

impl GPU {
    /* #region Mode execute methods ============================================================= */

    pub(super) fn oam_scan(&mut self) {
        // OAM Scan always ends at the same time
        self.ds.end_mode_time = OAM_TIME;

        // Block OAM memory
        self.mmu.borrow_mut().block_region(OAM);

        // Request the Mode 2 STAT interrupt if we've been asked to
        if self.get_mode_2_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Scan for objects that intersect this line
        for i in 0..NUM_OBJECTS {
            let a = OAM.begin() + i * OBJECT_BYTE_SIZE;
            let obj_y = (self.mmu_get(a) as i16) - 16;
            let obj_height = obj_y + either!(self.get_obj_size() => 16; 8);
            let line = self.ds.current_line as i16;
            if obj_y <= line && line < (obj_y + obj_height) {
                self.ds.selected_objects.push(Object {
                    address: a,
                    y: obj_y,
                    x: self.mmu_get(a + 1) as i16 - 8,
                    tile_index: self.mmu_get(a + 2),
                    flags: self.mmu_get(a + 3),
                });
                if self.ds.selected_objects.len() >= OBJECTS_PER_LINE {
                    // Stop scanning for more objects; only 10 per line
                    break;
                }
            }
        }

        // Sort by priority
        self.ds.selected_objects.sort_by(|a, b| {
            if a.x != b.x {
                // Sort first by x value
                a.x.cmp(&b.x)
            } else {
                // Then by memory address
                a.address.cmp(&b.address)
            }
        });
    }

    pub(super) fn draw(&mut self) {
        const TILE_SIZE: usize = 8;
        const MAP_PX_SIZE: usize = 256;
        const MAP_SIZE: usize = MAP_PX_SIZE / TILE_SIZE;

        // Block VRAM
        self.mmu.borrow_mut().block_region(VRAM);

        let y = self.ds.current_line as usize;

        // Calculating y positions early; some of these may not be used
        // The pixel y position in the 256x256 background
        let bg_map_y = (self.scroll_y as usize + y) % MAP_PX_SIZE;
        // The tile y position in the 32x32 background tile map
        let bg_tile_y = bg_map_y / TILE_SIZE;
        // The pixel y position within the tile
        let bg_intile_y = bg_map_y % TILE_SIZE;

        // The pixel y position in the window's area
        let win_map_y = self.ds.win_y_counter;
        // The tile y position in the tile map
        let win_tile_y = win_map_y / TILE_SIZE;
        // The pixel y position within the tile
        let win_intile_y = win_tile_y % TILE_SIZE;

        for x in 0..BASE_SCREEN_WIDTH {
            // The final color is always 0 if nothing else changes it
            let mut final_color = Color0;

            if self.get_bg_window_enabled() {
                // The pixel x position in the 256x256 background
                let bg_map_x = (self.scroll_x as usize + x) % MAP_PX_SIZE;
                // The tile x position in the 32x32 background tile map
                let bg_tile_x = bg_map_x / TILE_SIZE;
                // The pixel x position within the tile
                let bg_intile_x = bg_map_x % TILE_SIZE;
                // The tile's index in tile data
                let bg_tile_index = self.mmu.borrow().get_tile_index_at(
                    self.get_bg_tile_map(),
                    bg_tile_x,
                    bg_tile_y,
                );
                // The color id at that point in the tile
                let color_id = self
                    .mmu
                    .borrow()
                    .get_tile(self.get_bg_win_tile_data(), bg_tile_index)
                    .pix_at(bg_intile_x, bg_intile_y);
                // The final color is mapped from the background palette
                final_color = self.bg_palette[color_id];

                if self.get_win_enabled() {
                    // If this line overlaps the window's area
                    if y >= self.win_y as usize {
                        // The pixel x position in the window's area
                        let win_map_x = x as isize - (self.win_xp7 as isize - 7);
                        // If the x value is not negative
                        if win_map_x >= 0 {
                            // The tile x position in the 32x32 tile map
                            let win_tile_x = win_map_x as usize / TILE_SIZE;
                            // The pixel position within the tile
                            let win_intile_x = win_map_x as usize % TILE_SIZE;
                            // The tile's index
                            let win_tile_index = self.mmu.borrow().get_tile_index_at(
                                self.get_win_tile_map(),
                                win_tile_x,
                                win_tile_y,
                            );
                            // The color id at that position in the tile
                            let color_id = self
                                .mmu
                                .borrow()
                                .get_tile(self.get_bg_win_tile_data(), win_tile_index)
                                .pix_at(win_intile_x, win_intile_y);
                            // The final color is mapped from the background palette
                            final_color = self.bg_palette[color_id];

                            // Increment the window's y position; note that this is not just the
                            // current y position
                            self.ds.win_y_counter += 1;
                        }
                    }
                }
            }

            if self.get_obj_enabled() {
                for obj in &self.ds.selected_objects {
                    // x and y as signed integers
                    let xi16 = x as i16;
                    let yi16 = y as i16;
                    // If the object overlaps this x position
                    if obj.x <= xi16 && xi16 < obj.x + TILE_SIZE as i16 {
                        // The pixel y position within the object (0-15)
                        let obj_inobj_y = yi16 - obj.y;
                        // The object's tile index; this is just the index in the object's data in
                        // 8x8 mode, but it could be the second one in 8x16 mode
                        let obj_tile_index = if self.get_obj_size() {
                            // 8x16
                            let obj_tiles = obj.get_8x16_tile_indexes();
                            either!(obj_inobj_y < TILE_SIZE as i16 => obj_tiles.0; obj_tiles.1)
                        } else {
                            // 8x8
                            obj.tile_index
                        };
                        // The pixel x position within the tile
                        let obj_intile_x = x - obj.x as usize;
                        // The pixel y position within the tile
                        let obj_intile_y = obj_inobj_y as usize % TILE_SIZE;
                        // The color id at that position in the tile
                        let color_id = self
                            .mmu
                            .borrow()
                            .get_tile(true, obj_tile_index)
                            .pix_at(obj_intile_x, obj_intile_y);
                        // The palette to use; controlled by the object's flags
                        let palette =
                            either!(obj.get_palette() => &self.obj_palette_1; &self.obj_palette_0);
                        // The mapped color at that position in the tile (id 0 is always transparent)
                        let actual_color = either!(color_id == Color0 => Color0; palette[color_id]);

                        // If the sprite is over the background, OR the color so far is 0
                        let should_draw = !obj.get_under_bg() || final_color == Color0;
                        if should_draw && actual_color != Color0 {
                            // If we should draw the pixel, AND the pixel is not 0
                            final_color = actual_color;

                            // Stop looping through the objects
                            break;
                        }
                    }
                }
            }

            self.draw_scaled_pixel(x, final_color);
        }

        // TODO: Calculate actual draw mode length
        self.ds.end_mode_time += 200.into();
    }

    fn draw_scaled_pixel(&mut self, x: usize, color_id: ColorID) {
        let x1 = x * self.scale;
        let x2 = x1 + self.scale;

        let y1 = self.ds.current_line as usize * self.scale;
        let y2 = y1 + self.scale;

        let c = self.meta_palette[color_id];

        for iy in y1..y2 {
            for ix in x1..x2 {
                self.frame_buffer[iy * self.scr_width + ix] = c;
            }
        }
    }

    pub(super) fn hblank(&mut self) {
        // HBlank lasts until the end of the current scanline
        self.ds.end_mode_time = SCANLINE_TIME;

        // Unblock memory
        self.mmu.borrow_mut().unblock_region(OAM);
        self.mmu.borrow_mut().unblock_region(VRAM);

        // Request the Mode 0 STAT interrupt if we've been asked to
        if self.get_mode_0_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Clear the selected objects for the line
        self.ds.selected_objects.clear();
    }

    pub(super) fn vblank(&mut self) {
        // VBlank doesn't necessarily end here, but we do need to check if it ends
        self.ds.end_mode_time = SCANLINE_TIME;

        // Request the VBlank interrupt
        self.set_vblank_interrupt(true);

        // Request the Mode 1 STAT interrupt if we've been asked to
        if self.get_mode_1_interrupt() {
            self.set_stat_interrupt(true);
        }

        // Reset window y counter
        self.ds.win_y_counter = 0;

        // Present the frame and wait if necessary
        self.frame(true);
    }

    /* #endregion */

    /* #region Mode switch methods ============================================================== */

    pub(super) fn oam_scan_next(&mut self) {
        // Mode on to Draw
        self.ds.mode = Draw;
    }

    pub(super) fn draw_next(&mut self) {
        // Move on to horizontal blank
        self.ds.mode = HBlank;
    }

    pub(super) fn hblank_next(&mut self) {
        if self.ds.current_line >= LINES_PER_DRAW {
            // After all lines are drawn, move on to vertical blank
            self.ds.mode = VBlank;
        } else {
            // If there are lines left to draw, go back to OAM scan
            self.ds.mode = OamScan;
        }
    }

    pub(super) fn vblank_next(&mut self) {
        if self.ds.current_line >= LINES_PER_FRAME {
            // All the lines of vblank are done, go back to OAM scan
            self.ds.current_line = 0;
            self.ds.mode = OamScan;
        }
        // Otherwise, just stay in vblank
    }

    /* #endregion */
}
