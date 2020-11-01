use std::{cell::Ref, cell::RefMut, collections::BTreeMap, fmt::Debug};

use store::StoreQuery;

use crate::{
    components::{ChildEntitiesData, Size, SoftwareFramebuffer, Window, ZIndex},
    entity_component_system::{ComponentStore, EntityID, SystemError, SystemTrait},
    primitive_types::Vector2I,
};

type ReadWindowEntity<'a> = (EntityID, Ref<'a, Window>, Ref<'a, Size>);
type MaybeReadChildEntities<'a> = (EntityID, Option<Ref<'a, ChildEntitiesData>>);

pub trait Renderer {
    type Data: Clone;

    fn traverse_control_entities(
        db: &ComponentStore,
        entity_id: EntityID,
        entity_z: i64,
    ) -> BTreeMap<EntityID, i64> {
        let mut z_layers: BTreeMap<EntityID, i64> = BTreeMap::new();

        let entity_z = if let (_, Some(z_index)) =
            StoreQuery::<(EntityID, Option<Ref<ZIndex>>)>::get(db.as_ref(), &entity_id)
        {
            **z_index
        } else {
            entity_z
        };

        if Self::entity_predicate(db, entity_id) {
            z_layers.insert(entity_id, entity_z);
        }

        if let (_, Some(child_entities)) =
            StoreQuery::<MaybeReadChildEntities>::get(db.as_ref(), &entity_id)
        {
            for child_id in child_entities.iter() {
                z_layers.append(&mut Self::traverse_control_entities(
                    db, *child_id, entity_z,
                ));
            }
        }

        z_layers
    }

    fn render(
        &self,
        db: &ComponentStore,
        framebuffer: &mut RefMut<SoftwareFramebuffer<Self::Data>>,
        window_size: Vector2I,
        entity_id: EntityID,
        z: i64,
    );

    fn entity_predicate(db: &ComponentStore, entity_id: EntityID) -> bool;
}

impl<T> SystemTrait for T
where
    T: Debug + Renderer + 'static,
{
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        let (window_entity_id, _window, size) = StoreQuery::<ReadWindowEntity>::iter(db.as_ref())
            .next()
            .expect("No window entity");

        let (_, mut framebuffer) =
            StoreQuery::<(EntityID, RefMut<SoftwareFramebuffer<T::Data>>)>::iter(db.as_ref())
                .next()
                .expect("No CPU framebuffer entity");

        // Fetch color buffer entity
        let Vector2I(window_width, window_height) = **size;
        let cell_count = (window_width * window_height) as usize;
        framebuffer.resize(cell_count);

        // Recursively traverse parent-child tree and populate Z-ordered list of controls
        let control_entities: BTreeMap<EntityID, i64> =
            Self::traverse_control_entities(&db, window_entity_id, 0);

        // Render Entities
        framebuffer.clear();

        if window_width == 0 || window_height == 0 {
            return Ok(());
        }

        for (entity_id, z) in control_entities {
            self.render(
                db,
                &mut framebuffer,
                Vector2I(window_width, window_height),
                entity_id,
                z,
            );
        }

        Ok(())
    }
}
