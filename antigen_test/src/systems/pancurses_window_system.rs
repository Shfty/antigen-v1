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
    entity_component_system::create_entity,
    entity_component_system::entity_component_database::ComponentStorage,
    entity_component_system::entity_component_database::EntityComponentDatabase,
    entity_component_system::entity_component_database::EntityComponentDirectory,
    entity_component_system::get_entity_component,
    entity_component_system::get_entity_component_mut,
    entity_component_system::insert_entity_component,
    entity_component_system::ComponentTrait,
    entity_component_system::EntityID,
    entity_component_system::{SystemError, SystemTrait},
    primitive_types::IVector2,
};
use pancurses::ToChtype;
use std::collections::HashMap;

// TODO: Properly delete windows when their component is removed

#[derive(Debug)]
pub struct PancursesWindowSystem;

impl PancursesWindowSystem {
    pub fn new<CS, CD>(db: &mut EntityComponentDatabase<CS, CD>) -> Self
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        fn drop_callback(_: &mut dyn ComponentTrait) {
            pancurses::endwin();
        }

        db.register_component_drop_callback::<PancursesWindowComponent>(drop_callback);

        PancursesWindowSystem
    }

    fn try_create_window<CS, CD>(
        &mut self,
        db: &mut EntityComponentDatabase<CS, CD>,
        entity_id: EntityID,
    ) -> Result<(), String>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        let pancurses_window_component = get_entity_component::<CS, CD, PancursesWindowComponent>(
            &db.component_storage,
            &db.entity_component_directory,
            entity_id,
        )?;

        if pancurses_window_component.get_window().is_some() {
            return Ok(());
        }

        let IVector2(width, height) = get_entity_component::<CS, CD, SizeComponent>(
            &db.component_storage,
            &db.entity_component_directory,
            entity_id,
        )?
        .get_size();

        let background_char = match get_entity_component::<CS, CD, CharComponent>(
            &db.component_storage,
            &db.entity_component_directory,
            entity_id,
        ) {
            Ok(char_component) => *char_component.get_data(),
            Err(_) => ' ',
        };

        let background_color_pair = match get_entity_component::<CS, CD, PancursesColorPairComponent>(
            &db.component_storage,
            &db.entity_component_directory,
            entity_id,
        ) {
            Ok(pancurses_color_pair_component) => *pancurses_color_pair_component.get_data(),
            Err(_) => PancursesColorPair::default(),
        };

        let title = match get_entity_component::<CS, CD, StringComponent>(
            &db.component_storage,
            &db.entity_component_directory,
            entity_id,
        ) {
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

        let color_entity = create_entity(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            &mut db.callback_manager,
            Some("Pancurses Colors"),
        )?;
        insert_entity_component(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            color_entity,
            PancursesColorSetComponent::new(colors, color_pairs),
        )?;

        let mouse_entity = create_entity(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            &mut db.callback_manager,
            Some("Pancurses Mouse"),
        )?;
        insert_entity_component(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            mouse_entity,
            PancursesMouseComponent::new(),
        )?;

        let pancurses_color_set_entity = db.get_entity_by_predicate(|entity_id| {
            db.entity_has_component::<PancursesColorSetComponent>(entity_id)
        });
        let background_color_pair = if let Some(entity_id) = pancurses_color_set_entity {
            get_entity_component_mut::<CS, CD, PancursesColorSetComponent>(
                &mut db.component_storage,
                &mut db.entity_component_directory,
                entity_id,
            )?
            .get_color_pair_idx(&background_color_pair)
        } else {
            return Err("No pancurses color set entity".into());
        };

        window.bkgdset(
            background_char.to_chtype() | pancurses::COLOR_PAIR(background_color_pair as u64),
        );

        get_entity_component_mut::<CS, CD, PancursesWindowComponent>(
            &mut db.component_storage,
            &mut db.entity_component_directory,
            entity_id,
        )?
        .set_window(Some(window));

        Ok(())
    }
}

impl<CS, CD> SystemTrait<CS, CD> for PancursesWindowSystem
where
    CS: ComponentStorage,
    CD: EntityComponentDirectory,
{
    fn run(&mut self, db: &mut EntityComponentDatabase<CS, CD>) -> Result<(), SystemError>
    where
        CS: ComponentStorage,
        CD: EntityComponentDirectory,
    {
        // Get window entities, update internal window state
        let window_entities = db.get_entities_by_predicate(|entity_id| {
            db.entity_has_component::<WindowComponent>(entity_id)
                && db.entity_has_component::<PancursesWindowComponent>(entity_id)
                && db.entity_has_component::<SizeComponent>(entity_id)
        });

        for entity_id in window_entities {
            self.try_create_window(db, entity_id)?;

            if let Some(window) = get_entity_component::<CS, CD, PancursesWindowComponent>(
                &db.component_storage,
                &db.entity_component_directory,
                entity_id,
            )?
            .get_window()
            {
                let (window_height, window_width) = window.get_max_yx();

                get_entity_component_mut::<CS, CD, SizeComponent>(
                    &mut db.component_storage,
                    &mut db.entity_component_directory,
                    entity_id,
                )?
                .set_size(IVector2(window_width as i64, window_height as i64));
            }
        }

        Ok(())
    }
}
