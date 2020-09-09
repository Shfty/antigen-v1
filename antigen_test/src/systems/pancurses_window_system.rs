use crate::{
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::{
        pancurses_color_pair_component::PancursesColorPairComponent,
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::{PancursesColor, PancursesColorPair},
};
use antigen::{
    components::ParentEntityComponent,
    components::{CharComponent, PositionComponent, SizeComponent},
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemEvent, SystemTrait},
    primitive_types::IVector2,
ecs::EntityComponentDatabaseDebug};
use pancurses::ToChtype;
use std::collections::HashMap;

// TODO: Drop subwindow entities when their parent is dropped
//       Can't query the window's internal 'is deleted' field for this
//       Need to have all interested systems process a 'component deleted' event before the ComponentData box is dropped
//       Have systems register a drop callback with the ECS for a given component type

#[derive(Debug)]
pub struct PancursesWindowSystem;

impl PancursesWindowSystem {
    pub fn new() -> Self {
        PancursesWindowSystem
    }

    fn try_create_window(
        &mut self,
        ecs: &mut impl EntityComponentDatabase,
        entity_id: EntityID,
        parent_window_entity_id: Option<EntityID>,
    ) -> Result<(), String> {
        let pancurses_window_component =
            ecs.get_entity_component::<PancursesWindowComponent>(entity_id)?;

        if pancurses_window_component.window.is_some() {
            return Ok(());
        }

        let IVector2(pos_x, pos_y) = match ecs.get_entity_component::<PositionComponent>(entity_id)
        {
            Ok(position_component) => position_component.data,
            Err(_) => IVector2(0, 0),
        };

        let size_component = ecs.get_entity_component::<SizeComponent>(entity_id)?;
        let IVector2(width, height) = size_component.data;

        let background_char = match ecs.get_entity_component::<CharComponent>(entity_id) {
            Ok(char_component) => char_component.data,
            Err(_) => ' ',
        };

        let background_color_pair =
            match ecs.get_entity_component::<PancursesColorPairComponent>(entity_id) {
                Ok(pancurses_color_pair_component) => pancurses_color_pair_component.data,
                Err(_) => PancursesColorPair::default(),
            };

        let window = match parent_window_entity_id {
            Some(parent_window_entity_id) => {
                let parent_window_component =
                    ecs.get_entity_component::<PancursesWindowComponent>(parent_window_entity_id)?;
                if let Some(parent_window) = &parent_window_component.window {
                    parent_window
                        .derwin(height as i32, width as i32, pos_y as i32, pos_x as i32)
                        .unwrap()
                } else {
                    return Err("Invalid parent window".into());
                }
            }
            None => {
                let window = pancurses::initscr();
                pancurses::resize_term(height as i32, width as i32);
                pancurses::curs_set(0);
                pancurses::noecho();
                pancurses::start_color();

                let iter = 0..8;
                let colors: HashMap<PancursesColor, i16> = iter
                    .map(|i| {
                        let (r, g, b) = pancurses::color_content(i);
                        (PancursesColor::new(r, g, b), i)
                    })
                    .collect();

                let color_pairs = vec![(PancursesColorPair::default(), 0)]
                    .into_iter()
                    .collect();

                let color_entity = ecs.create_entity("Pancurses Colors");
                ecs.add_component_to_entity(
                    color_entity,
                    PancursesColorSetComponent::new(colors, color_pairs),
                )?;

                window
            }
        };

        window.keypad(true);
        window.nodelay(true);

        let pancurses_color_set_entities = ecs.get_entities_by_predicate(|entity_id| {
            ecs.entity_has_component::<PancursesColorSetComponent>(entity_id)
        });
        assert!(pancurses_color_set_entities.len() <= 1);
        let background_color_pair = if let Some(entity_id) = pancurses_color_set_entities.get(0) {
            let pancurses_color_set_component =
                ecs.get_entity_component::<PancursesColorSetComponent>(*entity_id)?;
            pancurses_color_set_component.get_color_pair_idx(background_color_pair)
        } else {
            return Err("No pancurses color set entity".into());
        };

        window.bkgdset(
            background_char.to_chtype() | pancurses::COLOR_PAIR(background_color_pair as u64),
        );

        let window_component = ecs.get_entity_component::<PancursesWindowComponent>(entity_id)?;
        window_component.window = Some(window);

        Ok(())
    }

    /*
    fn destroy_window(&mut self, window_id: WindowID) {
        if window_id == 0 {
            pancurses::endwin();
            self.windows.clear();
        } else {
            self.windows.remove(&window_id);
        }
    }
        */
}

impl<T> SystemTrait<T> for PancursesWindowSystem where T: EntityComponentDatabase + EntityComponentDatabaseDebug
{
    fn run(&mut self, db: &mut T) -> Result<SystemEvent, String> {
        // Get window entities, update internal window state
        let mut window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && db.entity_has_component::<SizeComponent>(entity_id)
        });

        window_entities.sort_by(|lhs, rhs| {
            let lhs_window_component = db
                .get_entity_component::<PancursesWindowComponent>(*lhs)
                .unwrap();
            let lhs_window_id = lhs_window_component.window_id;

            let rhs_window_component = db
                .get_entity_component::<PancursesWindowComponent>(*rhs)
                .unwrap();
            let rhs_window_id = rhs_window_component.window_id;

            lhs_window_id.cmp(&rhs_window_id)
        });

        for entity_id in &window_entities {
            let entity_id = *entity_id;

            let parent_entity_id =
                match db.get_entity_component::<ParentEntityComponent>(entity_id) {
                    Ok(parent_entity_component) => Some(parent_entity_component.parent_id),
                    Err(_) => None,
                };

            self.try_create_window(db, entity_id, parent_entity_id)?;
        }

        /*
        let inactive_window_ids: Vec<WindowID> = window_components
            .filter(|(_, window_component)| !active_window_ids.contains(window_component.window_id))
            .copied()
            .collect();

        for window_id in inactive_window_ids {
            self.destroy_window(window_id);
        }
        */

        Ok(SystemEvent::None)
    }
}
