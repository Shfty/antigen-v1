use antigen::{ecs::ComponentDebugTrait, ecs::ComponentTrait, primitive_types::IVector2};

#[derive(Debug, Copy, Clone)]
pub struct LocalMousePositionComponent {
    data: IVector2,
}

impl LocalMousePositionComponent {
    pub fn new() -> Self {
        LocalMousePositionComponent {
            data: IVector2::default(),
        }
    }

    pub fn get_local_mouse_position(&self) -> IVector2 {
        self.data
    }

    pub fn set_local_mouse_position(&mut self, local_mouse_position: IVector2) -> &mut Self {
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
