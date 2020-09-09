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
    pub control_data: ControlData,
}

impl PancursesControlComponent {
    pub fn new(
        control_data: ControlData,
    ) -> Self {
        PancursesControlComponent {
            control_data,
        }
    }
}

impl Default for PancursesControlComponent {
    fn default() -> Self {
        PancursesControlComponent::new(ControlData::default())
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
