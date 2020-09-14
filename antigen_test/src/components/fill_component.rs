use antigen::ecs::{ComponentTrait, ComponentDebugTrait};

// TODO: Refactor into PancursesFillColorComponent

#[derive(Debug, Default, Copy, Clone)]
pub struct FillComponent;

impl ComponentTrait for FillComponent {}

impl ComponentDebugTrait for FillComponent {
    fn get_name() -> String {
        "Fill".into()
    }

    fn get_description() -> String {
        "Fill flag for primitive shape rendering".into()
    }
}