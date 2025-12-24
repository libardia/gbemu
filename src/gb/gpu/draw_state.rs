use crate::gb::time_types::TTime;

use super::{object::Object, RenderMode, OBJECTS_PER_LINE};

#[derive(Debug)]
pub struct DrawState {
    pub mode: RenderMode,
    pub last_executed_mode: RenderMode,
    pub current_line: u8,
    pub win_y_counter: usize,
    pub time_this_line: TTime,
    pub end_mode_time: TTime,
    pub selected_objects: Vec<Object>,
}
impl DrawState {
    pub fn new() -> Self {
        Self {
            mode: RenderMode::OamScan,
            last_executed_mode: RenderMode::OamScan,
            current_line: 0,
            win_y_counter: 0,
            time_this_line: 0.into(),
            end_mode_time: 0.into(),
            selected_objects: Vec::with_capacity(OBJECTS_PER_LINE),
        }
    }
}
