use crate::{gb::time_types::TTime, mem_region::regions::OAM};

use super::{
    object::{Object, OBJECT_BYTE_SIZE},
    RenderMode,
};

const SELECTED_OBJS_RESERVED: usize = 10 + 5;
const FIFO_RESERVED: usize = 16 + 8;
pub const NUM_OBJECTS: u16 = OAM.size() / OBJECT_BYTE_SIZE;

#[derive(Debug)]
pub struct DrawState {
    pub mode: RenderMode,
    pub last_executed_mode: RenderMode,
    pub current_line: u8,
    pub time_this_line: TTime,
    pub end_mode_time: TTime,
    pub selected_objects: Vec<Object>,
}
impl DrawState {
    pub fn new() -> Self {
        let mut selected_objects = Vec::new();
        selected_objects.reserve(SELECTED_OBJS_RESERVED);

        Self {
            mode: RenderMode::OamScan,
            last_executed_mode: RenderMode::OamScan,
            current_line: 0,
            time_this_line: 0.into(),
            end_mode_time: 0.into(),
            selected_objects,
        }
    }
}
