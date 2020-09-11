use antigen::{
    ecs::{ComponentMetadataTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct PancursesMouseComponent {
    pub position: IVector2,
    pub button_mask: i64,
}

impl<'a> PancursesMouseComponent {
    pub fn new() -> Self {
        PancursesMouseComponent {
            position: IVector2::default(),
            button_mask: 0,
        }
    }
}

impl<'a> Default for PancursesMouseComponent {
    fn default() -> Self {
        PancursesMouseComponent::new()
    }
}

impl ComponentTrait for PancursesMouseComponent {}

impl ComponentMetadataTrait for PancursesMouseComponent {
    fn get_name() -> &'static str {
        "Pancurses Mouse"
    }

    fn get_description() -> &'static str {
        "Holds mouse position and button state"
    }
}
