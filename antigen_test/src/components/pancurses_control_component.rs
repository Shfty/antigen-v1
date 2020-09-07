use super::pancurses_window_component::WindowID;
use antigen::ecs::{ComponentMetadataTrait, ComponentTrait};

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ControlData {
    String,
    Rect { filled: bool },
}

impl Default for ControlData {
    fn default() -> Self {
        ControlData::String
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PancursesControlComponent {
    pub window_id: WindowID,
    pub control_data: ControlData,
}

impl PancursesControlComponent {
    pub fn new(
        window_id: WindowID,
        control_data: ControlData,
    ) -> Self {
        PancursesControlComponent {
            window_id,
            control_data,
        }
    }
}

impl Default for PancursesControlComponent {
    fn default() -> Self {
        PancursesControlComponent::new(0, ControlData::default())
    }
}

impl ComponentTrait for PancursesControlComponent {}

impl ComponentMetadataTrait for PancursesControlComponent {
    fn get_name() -> &'static str {
        "Pancurses Control"
    }

    fn get_description() -> &'static str {
        "UI control for use with Pancurses"
    }
}
