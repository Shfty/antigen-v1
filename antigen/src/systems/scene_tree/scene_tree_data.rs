use crate::{
    components::ChildEntitiesData,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
};
use crate::{components::ParentEntity, entity_component_system::ComponentStore};

use store::{NoField, StoreQuery};

use std::{
    cell::{Ref, RefMut},
    fmt::Debug,
    marker::PhantomData,
    ops::Add,
};

type MaybeReadChildEntities<'a> = (EntityID, Option<Ref<'a, ChildEntitiesData>>);

#[derive(Debug, Default)]
pub struct SceneTreeData<I, O>
where
    I: Copy + Default + Into<O> + Add<I, Output = I> + 'static,
    O: 'static,
{
    _phantom_i: PhantomData<I>,
    _phantom_o: PhantomData<O>,
}

impl<I, O> SceneTreeData<I, O>
where
    I: Debug + Copy + Default + Into<O> + Add<I, Output = I> + 'static,
    O: Debug + 'static,
{
    fn traverse_tree(&self, db: &ComponentStore, entity_id: EntityID, acc: I) {
        let acc = if let (_, Some(input)) =
            StoreQuery::<(EntityID, Option<Ref<I>>)>::get(db.as_ref(), &entity_id)
        {
            acc + (*input)
        } else {
            acc
        };

        if let (_, Some(mut output)) =
            StoreQuery::<(EntityID, Option<RefMut<O>>)>::get(db.as_ref(), &entity_id)
        {
            let acc: O = acc.into();
            *output = acc;
        }

        if let (_, Some(child_entities)) =
            StoreQuery::<MaybeReadChildEntities>::get(db.as_ref(), &entity_id)
        {
            for child_id in child_entities.iter() {
                self.traverse_tree(db, *child_id, acc);
            }
        }
    }
}

impl<I, O> SystemTrait for SceneTreeData<I, O>
where
    I: Debug + Copy + Default + Into<O> + Add<I, Output = I> + 'static,
    O: Debug + 'static,
{
    fn run(&mut self, db: &mut ComponentStore) -> Result<(), SystemError> {
        for (entity_id, _, _) in
            StoreQuery::<(EntityID, NoField<ParentEntity>, Ref<ChildEntitiesData>)>::iter(
                db.as_ref(),
            )
        {
            self.traverse_tree(db, entity_id, I::default());
        }

        Ok(())
    }
}
