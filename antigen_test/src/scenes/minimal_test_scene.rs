use std::{collections::HashMap, ops::Range};

use antigen::{
    components::{self as antigen_components},
    core::events::AntigenInputEvent,
    core::palette::RGBArrangementPalette,
    entity_component_system::{
        system_interface::SystemInterface, system_storage::SystemStorage, Assemblage,
        EntityComponentDirectory, EntityComponentSystem, EntityID, Scene, SystemRunner,
    },
    primitive_types::{ColorRGB, ColorRGBF, Vector2I},
    systems as antigen_systems,
};
use antigen_curses::{components as curses_components, systems as curses_systems};

use crate::components;
use crate::systems;

#[derive(Eq, PartialEq, Hash)]
enum EntityAssemblage {
    Player = 0,
    StringControl = 1,
    RectControl = 2,
    BorderControl = 3,
    DestructionTest = 4,
}

pub struct MinimalTestScene;

impl Scene for MinimalTestScene {
    fn register_systems<CD, SS, SR>(
        ecs: &mut EntityComponentSystem<CD, SS, SR>,
    ) -> Result<(), String>
    where
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CD> + 'static,
        SR: SystemRunner + 'static,
    {
        ecs.push_system(antigen_systems::EventConsumer::<AntigenInputEvent>::new());
        ecs.push_system(antigen_systems::EventConsumer::<
            curses_components::CursesEvent,
        >::new());

        ecs.push_system(curses_systems::CursesInputBuffer);
        ecs.push_system(curses_systems::CursesKeyboard);
        ecs.push_system(curses_systems::CursesMouse::new());
        let pancurses_window_system = curses_systems::CursesWindow;
        ecs.push_system(pancurses_window_system);

        ecs.push_system(systems::QuitKey::new(antigen::core::keyboard::Key::Escape));

        ecs.push_system(systems::InputVelocity::new());

        ecs.push_system(antigen_systems::PositionIntegrator);
        ecs.push_system(antigen_systems::GlobalPosition);
        ecs.push_system(antigen_systems::ChildEntities::new());
        ecs.push_system(antigen_systems::SoftwareRenderer);
        ecs.push_system(antigen_systems::StringRenderer);
        ecs.push_system(curses_systems::CursesRenderer::new(
            RGBArrangementPalette::new_884(),
            curses_systems::TextColorMode::BlackWhite,
        ));

        Ok(())
    }

    fn create_entities<CD>(db: &mut SystemInterface<CD>) -> Result<(), String>
    where
        CD: EntityComponentDirectory,
    {
        let mut assemblages = create_assemblages()?;

        // Create global event queues
        let global_event_queues_entity = db.create_entity("Global Event Queues".into())?;
        db.insert_entity_component(
            global_event_queues_entity,
            antigen_components::EventQueue::<curses_components::CursesEvent>::default(),
        )?;

        db.insert_entity_component(
            global_event_queues_entity,
            antigen_components::EventQueue::<AntigenInputEvent>::default(),
        )?;

        // Create main window
        let cpu_framebuffer_entity = db.create_entity("CPU Framebuffer".into())?;
        db.insert_entity_component(
            cpu_framebuffer_entity,
            antigen_components::SoftwareFramebuffer::new(ColorRGB(0.0f32, 0.0f32, 0.0f32)),
        )?;

        let string_framebuffer_entity = db.create_entity("String Framebuffer".into())?;
        db.insert_entity_component(
            string_framebuffer_entity,
            antigen_components::SoftwareFramebuffer::new(' '),
        )?;

        let main_window_entity = create_window_entity(
            db,
            Some("Main Window"),
            antigen_components::Position::default(),
            antigen_components::Size(Vector2I(256, 64)),
            None,
        )?;

        create_game_window(db, &mut assemblages, main_window_entity)?;

        Ok(())
    }

    fn load<'a, CD, SS, SR>(ecs: &'a mut EntityComponentSystem<CD, SS, SR>) -> Result<(), String>
    where
        CD: EntityComponentDirectory + 'static,
        SS: SystemStorage<CD> + 'static,
        SR: SystemRunner + 'static,
    {
        Self::register_systems(ecs)?;
        let mut system_interface = ecs.get_system_interface();
        Self::create_entities(&mut system_interface)?;
        Ok(())
    }
}

fn create_window_entity<D>(
    db: &mut SystemInterface<D>,
    debug_label: Option<&str>,
    position: antigen_components::Position,
    size: antigen_components::Size,
    parent_window_entity_id: Option<EntityID>,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let entity_id = db.create_entity(debug_label)?;
    db.insert_entity_component(entity_id, antigen_components::Window)?;
    db.insert_entity_component(entity_id, curses_components::CursesWindowData::default())?;
    db.insert_entity_component(entity_id, position)?;
    db.insert_entity_component(entity_id, size)?;
    if let Some(parent_window_entity_id) = parent_window_entity_id {
        db.insert_entity_component(
            entity_id,
            antigen_components::ParentEntity(parent_window_entity_id),
        )?;
    }
    Ok(entity_id)
}

fn create_assemblages<D>() -> Result<HashMap<EntityAssemblage, Assemblage<D>>, String>
where
    D: EntityComponentDirectory,
{
    let mut assemblages: HashMap<EntityAssemblage, Assemblage<D>> = HashMap::new();

    assemblages.insert(
        EntityAssemblage::Player,
        Assemblage::build(
            "Player Entity",
            "Controllable ASCII character with position and velocity",
        )
        .add_component(antigen_components::Control)?
        .add_component(ColorRGB(1.0f32, 0.6f32, 1.0f32))?
        .add_component('@')?
        .add_component(antigen_components::Position(Vector2I(1, 1)))?
        .add_component(antigen_components::Velocity::default())?
        .finish(),
    );

    assemblages.insert(
        EntityAssemblage::StringControl,
        Assemblage::build("String Entity", "ASCII string control")
            .add_component(antigen_components::Control)?
            .add_component(String::default())?
            .add_component(antigen_components::Position::default())?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::RectControl,
        Assemblage::build("Rect Entity", "ASCII Rectangle control")
            .add_component(antigen_components::Control)?
            .add_component(antigen_components::Position::default())?
            .add_component(antigen_components::Size::default())?
            .add_component(char::default())?
            .add_component(ColorRGB(0.753f32, 0.753f32, 0.753f32))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::BorderControl,
        Assemblage::build("Border Entity", "ASCII Border control")
            .add_component(antigen_components::Control)?
            .add_component(antigen_components::Position::default())?
            .add_component(antigen_components::Size::default())?
            .add_component(char::default())?
            .add_component(antigen_components::CPUShader(
                antigen_components::CPUShader::rect,
            ))?
            .add_component(ColorRGB(0.753f32, 0.753f32, 0.753f32))?
            .finish(),
    );

    assemblages.insert(
        EntityAssemblage::DestructionTest,
        Assemblage::build(
            "Destruction Test",
            "Assemblage for destroying entities when space is pressed",
        )
        .add_component(components::DestructionTestInputData(
            antigen::core::keyboard::Key::Space,
        ))?
        .finish(),
    );

    Ok(assemblages)
}

fn create_game_window<CD>(
    db: &mut SystemInterface<CD>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<CD>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    CD: EntityComponentDirectory,
{
    // Create Game Window
    let game_window_entity = db.create_entity(Some("Game"))?;
    db.insert_entity_component(game_window_entity, antigen_components::Position::default())?;
    db.insert_entity_component(game_window_entity, antigen_components::Size::default())?;
    db.insert_entity_component(
        game_window_entity,
        antigen_components::ParentEntity(parent_window_entity),
    )?;
    db.insert_entity_component(
        game_window_entity,
        antigen_components::Anchors::new(0.0..1.0, 0.0..1.0),
    )?;

    // Create Test Rects
    let test_rect_entity = assemblages
        .get_mut(&EntityAssemblage::RectControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Rect Control"))?;
    {
        db.insert_entity_component(
            test_rect_entity,
            antigen_components::ParentEntity(game_window_entity),
        )?;
        db.insert_entity_component(
            test_rect_entity,
            antigen_components::GlobalPositionData::default(),
        )?;
        db.insert_entity_component(
            test_rect_entity,
            antigen_components::Anchors::new(0.0..1.0, 0.0..1.0),
        )?;
        db.insert_entity_component(
            test_rect_entity,
            antigen_components::CPUShader(antigen_components::CPUShader::hsv),
        )?;
    }

    // Create Test Player
    let test_player_entity = assemblages
        .get_mut(&EntityAssemblage::Player)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test Player"))?;
    db.insert_entity_component(
        test_player_entity,
        antigen_components::ParentEntity(game_window_entity),
    )?;
    db.insert_entity_component(
        test_player_entity,
        antigen_components::GlobalPositionData::default(),
    )?;

    // Create Test String
    let test_string_entity = assemblages
        .get_mut(&EntityAssemblage::StringControl)
        .unwrap()
        .create_and_assemble_entity(db, Some("Test String Control"))?;
    {
        **db.get_entity_component_mut::<antigen_components::Position>(test_string_entity)? =
            Vector2I(1, 1);
        *db.get_entity_component_mut::<String>(test_string_entity)? =
            "Testing One Two Three".into();

        db.insert_entity_component(
            test_string_entity,
            antigen_components::ParentEntity(test_player_entity),
        )?;
        db.insert_entity_component(
            test_string_entity,
            antigen_components::GlobalPositionData::default(),
        )?;
    }

    Ok(game_window_entity)
}
