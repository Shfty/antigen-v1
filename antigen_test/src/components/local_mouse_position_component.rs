use antigen::{primitive_types::IVector2, ecs::ComponentTrait, ecs::ComponentMetadataTrait};

#[derive(Debug, Copy, Clone)]
pub struct LocalMousePositionComponent {
    pub data: IVector2,
}

impl LocalMousePositionComponent {
    pub fn new(data: IVector2) -> Self {
        LocalMousePositionComponent { data }
    }
}

impl Default for LocalMousePositionComponent {
    fn default() -> Self {
        LocalMousePositionComponent::new(IVector2::default())
    }
}

impl ComponentTrait for LocalMousePositionComponent {}

impl ComponentMetadataTrait for LocalMousePositionComponent {
    fn get_name() -> &'static str {
        "Local Mouse Position"
    }

    fn get_description() -> &'static str {
        "Local-space mouse position"
    }
}
