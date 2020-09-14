use crate::{
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::pancurses_mouse_component::PancursesMouseComponent,
    components::{
        pancurses_color_pair_component::PancursesColorPairComponent,
        pancurses_window_component::PancursesWindowComponent,
    },
    pancurses_color::{PancursesColor, PancursesColorPair},
};
use antigen::{
    components::StringComponent,
    components::WindowComponent,
    components::{CharComponent, SizeComponent},
    ecs::ComponentID,
    ecs::ComponentStorage,
    ecs::ComponentTrait,
    ecs::EntityID,
    ecs::{EntityComponentDatabase, SystemError, SystemTrait},
    primitive_types::IVector2,
};
use pancurses::ToChtype;
use std::collections::HashMap;

// TODO: Properly delete windows when their component is removed

#[derive(Debug)]
pub struct PancursesWindowSystem;

impl PancursesWindowSystem {
    pub fn new(storage: &mut impl ComponentStorage) -> Self {
        fn drop_callback(_: &mut dyn ComponentTrait) {
            pancurses::endwin();
        }
        storage.register_component_drop_callback(
            ComponentID::get::<PancursesWindowComponent>(),
            drop_callback,
        );
        PancursesWindowSystem
    }

    fn try_create_window(
        &mut self,
        db: &mut impl EntityComponentDatabase,
        entity_id: EntityID,
    ) -> Result<(), String> {
        let pancurses_window_component =
            db.get_entity_component::<PancursesWindowComponent>(entity_id)?;

        if pancurses_window_component.get_window().is_some() {
            return Ok(());
        }

        let IVector2(width, height) = db
            .get_entity_component::<SizeComponent>(entity_id)?
            .get_size();

        let background_char = match db.get_entity_component::<CharComponent>(entity_id) {
            Ok(char_component) => *char_component.get_data(),
            Err(_) => ' ',
        };

        let background_color_pair =
            match db.get_entity_component::<PancursesColorPairComponent>(entity_id) {
                Ok(pancurses_color_pair_component) => *pancurses_color_pair_component.get_data(),
                Err(_) => PancursesColorPair::default(),
            };

        let title = match db.get_entity_component::<StringComponent>(entity_id) {
            Ok(string_component) => string_component.get_data(),
            Err(_) => "Antigen",
        };

        let window = pancurses::initscr();
        window.keypad(true);
        window.nodelay(true);
        window.timeout(0);

        pancurses::mousemask(
            pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
            std::ptr::null_mut(),
        );
        pancurses::mouseinterval(0);
        pancurses::resize_term(height as i32, width as i32);
        pancurses::set_title(&title);
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

        let color_entity = db.create_entity(Some("Pancurses Colors"))?;
        db.add_component_to_entity(
            color_entity,
            PancursesColorSetComponent::new(colors, color_pairs),
        )?;

        let mouse_entity = db.create_entity(Some("Pancurses Mouse"))?;
        db.add_component_to_entity(mouse_entity, PancursesMouseComponent::new())?;

        let pancurses_color_set_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesColorSetComponent>(entity_id)
        });
        assert!(pancurses_color_set_entities.len() <= 1);
        let background_color_pair = if let Some(entity_id) = pancurses_color_set_entities.get(0) {
            db.get_entity_component_mut::<PancursesColorSetComponent>(*entity_id)?
                .get_color_pair_idx(&background_color_pair)
        } else {
            return Err("No pancurses color set entity".into());
        };

        window.bkgdset(
            background_char.to_chtype() | pancurses::COLOR_PAIR(background_color_pair as u64),
        );

        db.get_entity_component_mut::<PancursesWindowComponent>(entity_id)?
            .set_window(Some(window));

        Ok(())
    }
}

impl<T> SystemTrait<T> for PancursesWindowSystem
where
    T: EntityComponentDatabase,
{
    fn run(&mut self, db: &mut T) -> Result<(), SystemError> {
        // Get window entities, update internal window state
        let window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<WindowComponent>(entity_id)
                && db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && db.entity_has_component::<SizeComponent>(entity_id)
        });

        for entity_id in window_entities {
            self.try_create_window(db, entity_id)?;

            if let Some(window) = db
                .get_entity_component::<PancursesWindowComponent>(entity_id)?
                .get_window()
            {
                let (window_height, window_width) = window.get_max_yx();

                db.get_entity_component_mut::<SizeComponent>(entity_id)?
                    .set_size(IVector2(window_width as i64, window_height as i64));
            }
        }

        Ok(())
    }
}
