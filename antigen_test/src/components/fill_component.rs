use antigen::ecs::{ComponentTrait, ComponentMetadataTrait};

// TODO: Refactor into PancursesFillColorComponent

#[derive(Debug, Copy, Clone)]
pub struct FillComponent;

impl ComponentTrait for FillComponent {}

impl ComponentMetadataTrait for FillComponent {
    fn get_name() -> &'static str {
        "Fill"
    }

    fn get_description() -> &'static str {
        "Fill flag for primitive shape rendering"
    }
}