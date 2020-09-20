use antigen::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::IVector2,
};

#[derive(Debug, Copy, Clone)]
pub struct PancursesMouseComponent {
    position: IVector2,
    button_mask_buffer: [i64; 2],
}

impl<'a> PancursesMouseComponent {
    pub fn new() -> Self {
        PancursesMouseComponent {
            position: IVector2::default(),
            button_mask_buffer: [0, 0],
        }
    }

    pub fn get_position(&self) -> IVector2 {
        self.position
    }

    pub fn get_button_mask(&self) -> i64 {
        self.button_mask_buffer[1]
    }

    pub fn set_mouse_x(&mut self, x: i64) -> &mut Self {
        self.position.0 = x;
        self
    }

    pub fn set_mouse_y(&mut self, y: i64) -> &mut Self {
        self.position.1 = y;
        self
    }

    pub fn set_button_mask(&mut self, button_mask: i64) -> &mut Self {
        self.button_mask_buffer[0] = self.button_mask_buffer[1];
        self.button_mask_buffer[1] = button_mask;
        self
    }

    pub fn was_button_just_pressed(&self, button_mask: i64) -> bool {
        self.button_mask_buffer[1] & button_mask > 0
            && self.button_mask_buffer[0] & button_mask == 0
    }
}

impl<'a> Default for PancursesMouseComponent {
    fn default() -> Self {
        PancursesMouseComponent::new()
    }
}

impl ComponentTrait for PancursesMouseComponent {}

impl ComponentDebugTrait for PancursesMouseComponent {
    fn get_name() -> String {
        "Pancurses Mouse".into()
    }

    fn get_description() -> String {
        "Holds mouse position and button state".into()
    }
}
