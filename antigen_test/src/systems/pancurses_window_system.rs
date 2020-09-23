use crate::{
    components::pancurses_color_set_component::PancursesColorSetComponent,
    components::pancurses_window_component::PancursesWindowComponent,
    pancurses_color::{PancursesColor, PancursesColorPair},
};
use antigen::{
    components::ColorComponent,
    components::EventQueueComponent,
    components::{CharComponent, SizeComponent, StringComponent, WindowComponent},
    entity_component_system::{
        system_interface::SystemInterface, ComponentStorage, ComponentTrait,
        EntityComponentDirectory, EntityID, SystemDebugTrait, SystemError, SystemTrait,
    },
    primitive_types::ColorRGB,
    primitive_types::Vector2I,
};
use pancurses::ToChtype;
use std::collections::HashMap;

// TODO: Properly delete windows when their component is removed

#[derive(Debug)]
pub struct PancursesWindowSystem;

impl PancursesWindowSystem {
    pub fn new<CS>(component_storage: &mut CS) -> Self
    where
        CS: ComponentStorage,
    {
        fn drop_callback(_: &mut dyn ComponentTrait) {
            pancurses::endwin();
        }

        component_storage
            .register_component_drop_callback::<PancursesWindowComponent>(drop_callback);

        PancursesWindowSystem
    }

    fn try_create_window<CS, CD>(
        &mut self,
        db: &mut SystemInterface<CS, CD>,
        entity_id: EntityID,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let pancurses_window_component =
            db.get_entity_component::<PancursesWindowComponent>(entity_id)?;

        if pancurses_window_component.get_window().is_some() {
            return Ok(());
        }

        let Vector2I(width, height) = db
            .get_entity_component::<SizeComponent>(entity_id)?
            .get_size();

        let background_char = match db.get_entity_component::<CharComponent>(entity_id) {
            Ok(char_component) => *char_component.get_data(),
            Err(_) => ' ',
        };

        let background_color = match db.get_entity_component::<ColorComponent>(entity_id) {
            Ok(color_component) => *color_component.get_data(),
            Err(_) => ColorRGB(0.0, 0.0, 0.0),
        };

        let title = match db.get_entity_component::<StringComponent>(entity_id) {
            Ok(string_component) => string_component.get_data(),
            Err(_) => "Antigen",
        };

        let window = pancurses::initscr();

        pancurses::resize_term(height as i32, width as i32);
        pancurses::set_title(&title);
        pancurses::mousemask(
            pancurses::ALL_MOUSE_EVENTS | pancurses::REPORT_MOUSE_POSITION,
            std::ptr::null_mut(),
        );
        pancurses::mouseinterval(0);
        pancurses::curs_set(0);
        pancurses::noecho();
        pancurses::start_color();

        window.keypad(true);
        window.timeout(0);

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
        db.insert_entity_component(
            color_entity,
            PancursesColorSetComponent::new(colors, color_pairs),
        )?;

        let pancurses_color_set_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<PancursesColorSetComponent>(entity_id)
                });
        let background_color_pair = if let Some(entity_id) = pancurses_color_set_entity {
            db.get_entity_component_mut::<PancursesColorSetComponent>(entity_id)?
                .get_color_pair_idx(ColorRGB(1.0, 1.0, 1.0), background_color)
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

impl<CS, CD> SystemTrait<CS, CD> for PancursesWindowSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut SystemInterface<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let pancurses_event_queue_entity =
            db.entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueueComponent<pancurses::Input>>(entity_id)
                });

        if let Some(pancurses_event_queue_entity) = pancurses_event_queue_entity {
            let pancurses_event_queue_component = db
                .get_entity_component::<EventQueueComponent<pancurses::Input>>(
                    pancurses_event_queue_entity,
                )?;

            for event in pancurses_event_queue_component.get_events() {
                if let pancurses::Input::KeyResize = event {
                    pancurses::resize_term(0, 0);
                }
            }
        }

        // Get window entities, update internal window state
        let window_entity = db
            .entity_component_directory
            .get_entity_by_predicate(|entity_id| {
                db.entity_component_directory
                    .entity_has_component::<WindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<PancursesWindowComponent>(entity_id)
                    && db
                        .entity_component_directory
                        .entity_has_component::<SizeComponent>(entity_id)
            });

        if let Some(window_entity) = window_entity {
            // Make sure the window exists
            self.try_create_window(db, window_entity)?;

            // Process any pending resize inputs
            let pancurses_event_queue_entity = db
                .entity_component_directory
                .get_entity_by_predicate(|entity_id| {
                    db.entity_component_directory
                        .entity_has_component::<EventQueueComponent<pancurses::Input>>(entity_id)
                });

            if let Some(pancurses_event_queue_entity) = pancurses_event_queue_entity {
                let pancurses_event_queue_component = db
                    .get_entity_component::<EventQueueComponent<pancurses::Input>>(
                        pancurses_event_queue_entity,
                    )?;

                for event in pancurses_event_queue_component.get_events() {
                    if let pancurses::Input::KeyResize = event {
                        pancurses::resize_term(0, 0);
                    }
                }
            }

            // Update window component size
            if let Some(window) = db
                .get_entity_component::<PancursesWindowComponent>(window_entity)?
                .get_window()
            {
                let (window_height, window_width) = window.get_max_yx();

                db.get_entity_component_mut::<SizeComponent>(window_entity)?
                    .set_size(Vector2I(window_width as i64, window_height as i64));
            }
        }

        Ok(())
    }
}

impl SystemDebugTrait for PancursesWindowSystem {
    fn get_name() -> &'static str {
        "Pancurses Window"
    }
}
