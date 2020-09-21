use antigen::{entity_component_system::ComponentDebugTrait, entity_component_system::ComponentTrait, primitive_types::Vector2I};

#[derive(Debug, Copy, Clone)]
pub struct LocalMousePositionComponent {
    data: Vector2I,
}

impl LocalMousePositionComponent {
    pub fn new() -> Self {
        LocalMousePositionComponent {
            data: Vector2I::default(),
        }
    }

    pub fn get_local_mouse_position(&self) -> Vector2I {
        self.data
    }

    pub fn set_local_mouse_position(&mut self, local_mouse_position: Vector2I) -> &mut Self {
        self.data = local_mouse_position;
        self
    }
}

impl Default for LocalMousePositionComponent {
    fn default() -> Self {
        LocalMousePositionComponent::new()
    }
}

impl ComponentTrait for LocalMousePositionComponent {}

impl ComponentDebugTrait for LocalMousePositionComponent {
    fn get_name() -> String {
        "Local Mouse Position".into()
    }

    fn get_description() -> String {
        "Local-space mouse position".into()
    }
}
