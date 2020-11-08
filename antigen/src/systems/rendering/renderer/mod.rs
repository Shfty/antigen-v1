mod software_raster_renderer;
pub use software_raster_renderer::*;

use crate::entity_component_system::{ComponentStore, EntityID};

/// Gathers data from the ECS and renders it to a framebuffer
pub trait Renderer {
    fn render(&self, db: &ComponentStore, framebuffer_id: EntityID, entities: &[EntityID]);
}
