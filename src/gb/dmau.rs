use std::{cell::RefCell, rc::Rc};

use log::debug;

use crate::{
    mem_region::{
        io_regs::REG_DMA,
        regions::{HIGH_RAM, OAM},
        MemoryRegion,
    },
    util::new,
};

use super::{mmu::MMU, time_types::MTime};

#[derive(Debug, Default)]
pub struct DMAU {
    // Reference to MMU
    mmu: Rc<RefCell<MMU>>,
    // State
    source: u16,
    dest: u16,
    dma_in_progress: bool,
}

impl DMAU {
    new!(mmu: Rc<RefCell<MMU>>);

    pub fn step(&mut self, dt: MTime) {
        const LOWER_MEMORY: MemoryRegion = MemoryRegion::new(0, HIGH_RAM.begin() - 1);

        // Convenience
        let mut b_mmu = self.mmu.borrow_mut();

        // Start DMA if requested
        if b_mmu.should_start_dma() {
            debug!("Beginning DMA transfer");
            self.dma_in_progress = true;
            // Set initial source and dest
            self.source = b_mmu.get(REG_DMA) as u16 * 0x100;
            self.dest = OAM.begin();
            // Block EVERYTHING except HRAM
            b_mmu.block_region(LOWER_MEMORY);
        }

        // If DMA is in progress...
        if self.dma_in_progress {
            for _ in 0..dt.to() {
                // Copy value
                let value = b_mmu.get(self.source);
                b_mmu.set(self.dest, value);

                // Increment addresses
                self.source += 1;
                self.dest += 1;

                // DMA is done when the next dest would be out of OAM
                if self.dest > OAM.end() {
                    debug!("DMA transfer complete");
                    self.dma_in_progress = false;
                    b_mmu.unblock_region(LOWER_MEMORY);
                    break;
                }
            }
        }
    }
}
