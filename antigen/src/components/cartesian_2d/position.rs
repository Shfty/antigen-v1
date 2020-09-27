use crate::{
    entity_component_system::{ComponentDebugTrait, ComponentTrait},
    primitive_types::Vector2I,
};
use std::fmt::Debug;

#[derive(Debug, Default, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Position(pub Vector2I);

impl From<Vector2I> for Position {
    fn from(vec: Vector2I) -> Self {
        Position(vec)
    }
}

impl Into<Vector2I> for Position {
    fn into(self) -> Vector2I {
        self.0
    }
}

impl ComponentTrait for Position {}

impl ComponentDebugTrait for Position {
    fn get_name() -> String {
        "Position".into()
    }

    fn get_description() -> String {
        "2D Cartesian Position".into()
    }
}
