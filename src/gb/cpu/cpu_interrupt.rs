use log::debug;

use super::CPU;

const INT_NAMES: [&str; 5] = ["VBlank", "STAT", "Timer", "Serial", "Joypad"];

const INT_MASKS: [u8; 5] = [
    1 << 0, // VBlank
    1 << 1, // STAT
    1 << 2, // Timer
    1 << 3, // Serial
    1 << 4, // Joypad
];

const INT_ADDRESSES: [u16; 5] = [
    0x40, // VBlank
    0x48, // STAT
    0x50, // Timer
    0x58, // Serial
    0x60, // Joypad
];

impl CPU {
    fn should_interrupt(&self, mask: u8) -> bool {
        ((self.int_enabled & mask) & (self.int_flags & mask)) != 0
    }

    pub(super) fn maybe_interrupt(&mut self) -> bool {
        // These are just for readability
        let m_iter = INT_MASKS.iter();
        let a_iter = INT_ADDRESSES.iter();

        for (i, (mask, address)) in m_iter.zip(a_iter).enumerate() {
            if self.should_interrupt(*mask) {
                debug!(
                    "[PC {:?}] Firing interrupt handler: {}",
                    self.hpc(),
                    INT_NAMES[i]
                );

                self.fire_interrupt(*mask, *address);

                // Return early; an interrupt fired
                return true;
            }
        }

        // No interrupt fired
        false
    }

    fn fire_interrupt(&mut self, mask: u8, address: u16) {
        // Reset corresponding interrupt flag
        self.int_flags &= !mask;

        // Disable interrupts
        self.ime = false;

        // Push current PC on the stack
        self.push_word(self.pc);

        // Jump to the handler's address
        self.pc = address;
    }
}
