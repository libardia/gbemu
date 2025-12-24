use std::{collections::HashMap, vec};

use crate::util::Hex16;

use super::{
    cpu::{
        instructions::{
            ArgR16MEM, ArgR8,
            Instruction::{self, *},
        },
        optables::{OP_TABLE, PREFIX_TABLE},
    },
    GB,
};

impl GB {
    pub(crate) fn compile(&mut self, prog: Vec<Instruction>) -> Box<Vec<u8>> {
        self.init_encode_table();

        let mut compiled = Box::new(Vec::new());
        for inst in prog {
            compiled.append(&mut self.encode(inst));
        }
        compiled
    }

    fn encode(&mut self, inst: Instruction) -> Vec<u8> {
        // We'll build this up to hold the final bytes of the instruction
        let mut final_bytes = Vec::new();
        // Get a reference to the encoding table (for convenience)
        let table = self.encode_table.as_ref().expect(
            "Tried to encode an instruction, but the encode table was not initialized yet.",
        );
        // The key always contains 0 as constants, so we'll need to cut those out and add them to
        // the code's bytes
        let (key, extra) = match inst {
            // 0x
            LD_r16_n16(x, v) => (LD_r16_n16(x, 0.into()), self.hex16_to_bytes(v)),
            LD_r8_r8(x, ArgR8::CONST(v)) => (LD_r8_r8(x, ArgR8::CONST(0.into())), vec![v.to()]),
            LD_mn16_sp(v) => (LD_mn16_sp(0.into()), self.hex16_to_bytes(v)),

            // 1x
            STOP(v) => (STOP(0.into()), vec![v.to()]),
            JR_e8(v) => (JR_e8(0), vec![v as u8]),

            // 2x
            JR_cc_e8(x, v) => (JR_cc_e8(x, 0), vec![v as u8]),

            // 3x
            LD_sp_n16(v) => (LD_sp_n16(0.into()), self.hex16_to_bytes(v)),

            // Cx
            JP_cc_n16(x, v) => (JP_cc_n16(x, 0.into()), self.hex16_to_bytes(v)),
            JP_n16(v) => (JP_n16(0.into()), self.hex16_to_bytes(v)),
            CALL_cc_n16(x, v) => (CALL_cc_n16(x, 0.into()), self.hex16_to_bytes(v)),
            ADD_a_r8(ArgR8::CONST(v)) => (ADD_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),
            CALL_n16(v) => (CALL_n16(0.into()), self.hex16_to_bytes(v)),
            ADC_a_r8(ArgR8::CONST(v)) => (ADC_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),

            // Dx
            SUB_a_r8(ArgR8::CONST(v)) => (SUB_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),
            SBC_a_r8(ArgR8::CONST(v)) => (SBC_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),

            // Ex
            LDH_mn16_a(v) => (LDH_mn16_a(0.into()), vec![v.to()]),
            AND_a_r8(ArgR8::CONST(v)) => (AND_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),
            ADD_sp_e8(v) => (ADD_sp_e8(0), vec![v as u8]),
            LD_mr16_a(ArgR16MEM::CONST(v)) => (
                LD_mr16_a(ArgR16MEM::CONST(0.into())),
                self.hex16_to_bytes(v),
            ),
            XOR_a_r8(ArgR8::CONST(v)) => (XOR_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),

            // Fx
            LDH_a_mn16(v) => (LDH_a_mn16(0.into()), vec![v.to()]),
            OR_a_r8(ArgR8::CONST(v)) => (OR_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),
            LD_hl_sp_plus_e8(v) => (LD_hl_sp_plus_e8(0), vec![v as u8]),
            LD_a_mr16(ArgR16MEM::CONST(v)) => (
                LD_a_mr16(ArgR16MEM::CONST(0.into())),
                self.hex16_to_bytes(v),
            ),
            CP_a_r8(ArgR8::CONST(v)) => (CP_a_r8(ArgR8::CONST(0.into())), vec![v.to()]),

            // Everything else
            _ => (inst, vec![]),
        };
        // Get the base bytes
        let base = &table[&key];
        // Add the base bytes to the final bytes
        final_bytes.extend(base);
        // Add the extra bytes to the final bytes
        final_bytes.extend(extra);
        // Return the final bytes
        final_bytes
    }

    fn hex16_to_bytes(&self, v: Hex16) -> Vec<u8> {
        let msb = ((v.to() & 0xFF00) >> 8) as u8;
        let lsb = (v.to() & 0xFF) as u8;
        vec![lsb, msb]
    }

    fn init_encode_table(&mut self) {
        if self.encode_table.is_some() {
            // Encode table was already built
            return;
        }

        let mut encode_table: Box<HashMap<Instruction, Vec<u8>>> = Box::new(HashMap::new());

        // Build encode table
        for (high, outer) in OP_TABLE.iter().enumerate() {
            for (low, inst) in outer.iter().enumerate() {
                // This might be the instruction we use as the key, but if it's PREFIX we might
                // change it.
                let mut final_inst = *inst;
                // Working space to build the instruction code
                let mut code = Vec::new();
                // Build the code byte for this instruction
                let code_byte = ((high << 4) | low) as u8;
                // Push that byte into the working vector
                code.push(code_byte);
                // If the instruction right now is PREFIX, we need to extend the code vector
                if *inst == PREFIX {
                    for (pref_high, pref_outer) in PREFIX_TABLE.iter().enumerate() {
                        for (pref_low, pref_inst) in pref_outer.iter().enumerate() {
                            // Change the instruction we'll use as the key
                            final_inst = *pref_inst;
                            // Build the code byte for the suffix instruction
                            let code_byte = ((pref_high << 4) | pref_low) as u8;
                            // Push that byte into the working vector
                            code.push(code_byte);
                        }
                    }
                }
                // Finally insert into the encoding table
                encode_table.insert(final_inst, code);
            }
        }

        // Save out the encode table
        self.encode_table = Some(encode_table);
    }
}
