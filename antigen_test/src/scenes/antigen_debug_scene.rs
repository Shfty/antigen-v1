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

pub struct AntigenDebugScene;

impl Scene for AntigenDebugScene {
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
        ecs.push_system(systems::InputAxis);
        ecs.push_system(systems::DestructionTestInput::new());
        ecs.push_system(antigen_systems::LocalMousePosition::new());

        ecs.push_system(antigen_systems::List::new());

        ecs.push_system(antigen_systems::EventProcessor::<
            antigen_systems::ListEvent,
            antigen_systems::EntityInspectorEvent,
        >::new(
            |list_event: antigen_systems::ListEvent| match list_event {
                antigen_systems::ListEvent::Pressed(index) => Some(
                    antigen_systems::EntityInspectorEvent::SetInspectedEntity(index),
                ),
                _ => None,
            },
        ));

        ecs.push_system(antigen_systems::EventProcessor::<
            antigen_systems::ListEvent,
            antigen_systems::ComponentInspectorEvent,
        >::new(
            |list_event: antigen_systems::ListEvent| match list_event {
                antigen_systems::ListEvent::Pressed(index) => {
                    Some(antigen_systems::ComponentInspectorEvent::SetInspectedComponent(index))
                }
                _ => None,
            },
        ));

        ecs.push_system(antigen_systems::EventProcessor::<
            antigen_systems::ListEvent,
            antigen_systems::SystemInspectorEvent,
        >::new(
            |list_event: antigen_systems::ListEvent| match list_event {
                antigen_systems::ListEvent::Pressed(index) => Some(
                    antigen_systems::SystemInspectorEvent::SetInspectedSystem(index),
                ),
                _ => None,
            },
        ));

        ecs.push_system(antigen_systems::EventConsumer::<antigen_systems::ListEvent>::new());

        ecs.push_system(systems::InputVelocity::new());

        ecs.push_system(antigen_systems::PositionIntegrator);
        ecs.push_system(antigen_systems::AnchorsMargins::new());
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
        // FIXME: Automatic storage population
        /*
        {
            db.component_store.add_storage_for::<ChildEntitiesData>();

            db.component_store.add_storage_for::<InputAxisData>();

            db.component_store
                .add_storage_for::<DestructionTestInputData>();
        }
        */

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

        let entity_inspector_entity = db.create_entity(Some("Entity Inspector"))?;
        db.insert_entity_component(
            entity_inspector_entity,
            antigen_components::EventQueue::<antigen_systems::EntityInspectorEvent>::default(),
        )?;
        db.insert_entity_component(
            entity_inspector_entity,
            antigen_components::IntRange::new(-1..0),
        )?;

        let component_inspector_entity = db.create_entity(Some("Component Inspector"))?;
        db.insert_entity_component(
            component_inspector_entity,
            antigen_components::EventQueue::<antigen_systems::ComponentInspectorEvent>::default(),
        )?;
        db.insert_entity_component(
            component_inspector_entity,
            antigen_components::IntRange::new(-1..0),
        )?;

        let system_inspector_entity = db.create_entity(Some("System Inspector"))?;
        db.insert_entity_component(
            system_inspector_entity,
            antigen_components::EventQueue::<antigen_systems::SystemInspectorEvent>::default(),
        )?;
        db.insert_entity_component(
            system_inspector_entity,
            antigen_components::IntRange::new(-1..0),
        )?;

        create_game_window(db, &mut assemblages, main_window_entity)?;

        create_entity_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            entity_inspector_entity,
        )?;
        create_scene_tree_window(
            db,
            &mut assemblages,
            main_window_entity,
            entity_inspector_entity,
        )?;
        create_component_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            component_inspector_entity,
        )?;
        create_component_data_list_window(db, &mut assemblages, main_window_entity)?;
        create_system_list_window(
            db,
            &mut assemblages,
            main_window_entity,
            system_inspector_entity,
        )?;

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

fn create_string_control<CD>(
    db: &mut SystemInterface<CD>,
    string_assemblage: &mut Assemblage<CD>,
    debug_label: Option<&str>,
    text: &str,
    (x, y): (i64, i64),
) -> Result<EntityID, String>
where
    CD: EntityComponentDirectory,
{
    let entity_id = string_assemblage.create_and_assemble_entity(db, debug_label)?;

    *db.get_entity_component_mut::<String>(entity_id)? = text.into();

    **db.get_entity_component_mut::<antigen_components::Position>(entity_id)? = Vector2I(x, y);

    Ok(entity_id)
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
        antigen_components::Anchors::new(0.0..0.25, 0.0..1.0),
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

fn create_debug_window<CD>(
    db: &mut SystemInterface<CD>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<CD>>,
    parent_window_entity: EntityID,
    window_name: &str,
    anchor_horizontal: Range<f32>,
    anchor_vertical: Range<f32>,
) -> Result<EntityID, String>
where
    CD: EntityComponentDirectory,
{
    let entity_list_window_entity = assemblages
        .get_mut(&EntityAssemblage::RectControl)
        .unwrap()
        .create_and_assemble_entity(db, Some(&format!("{} Window", window_name)))?;
    {
        *db.get_entity_component_mut::<ColorRGBF>(entity_list_window_entity)? =
            ColorRGB(0.0, 0.0, 0.0);

        db.insert_entity_component(
            entity_list_window_entity,
            antigen_components::Position::default(),
        )?;
        db.insert_entity_component(entity_list_window_entity, antigen_components::ZIndex(1))?;
        db.insert_entity_component(
            entity_list_window_entity,
            antigen_components::Size::default(),
        )?;
        db.insert_entity_component(
            entity_list_window_entity,
            antigen_components::ParentEntity(parent_window_entity),
        )?;
        db.insert_entity_component(
            entity_list_window_entity,
            antigen_components::Anchors::new(anchor_horizontal, anchor_vertical),
        )?;
    }

    let entity_list_border_entity = assemblages
        .get_mut(&EntityAssemblage::BorderControl)
        .unwrap()
        .create_and_assemble_entity(db, Some(&format!("{} Border", window_name)))?;
    {
        db.insert_entity_component(
            entity_list_border_entity,
            antigen_components::ParentEntity(entity_list_window_entity),
        )?;
        db.insert_entity_component(
            entity_list_border_entity,
            antigen_components::Anchors::new(0.0..1.0, 0.0..1.0),
        )?;
    }

    // Create Debug Window Title
    let entity_list_title_entity = create_string_control(
        db,
        assemblages
            .get_mut(&EntityAssemblage::StringControl)
            .unwrap(),
        Some(&format!("{} Title", window_name)),
        &format!("{}\n========", window_name),
        (2, 1),
    )?;
    {
        db.insert_entity_component(
            entity_list_title_entity,
            antigen_components::ParentEntity(entity_list_border_entity),
        )?;
        db.insert_entity_component(
            entity_list_title_entity,
            antigen_components::GlobalPositionData::default(),
        )?;
    }

    // Create Entity List
    let entity_list_entity = db.create_entity(Some(window_name))?;
    {
        let list_component = antigen_components::ListData::new(Some(entity_list_entity));

        db.insert_entity_component(entity_list_entity, list_component)?;
        db.insert_entity_component(entity_list_entity, antigen_components::Position::default())?;
        db.insert_entity_component(entity_list_entity, antigen_components::Size::default())?;
        db.insert_entity_component(
            entity_list_entity,
            antigen_components::ParentEntity(entity_list_border_entity),
        )?;
        db.insert_entity_component(
            entity_list_entity,
            antigen_components::Anchors::new(0.0..1.0, 0.0..1.0),
        )?;
        db.insert_entity_component(
            entity_list_entity,
            antigen_components::Margins::new(2, 2, 3, 1),
        )?;
        db.insert_entity_component(entity_list_entity, Vec::<String>::new())?;
        db.insert_entity_component(
            entity_list_entity,
            antigen_components::LocalMousePositionData::default(),
        )?;
    }

    Ok(entity_list_entity)
}

fn create_entity_list_window<D>(
    db: &mut SystemInterface<D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<D>>,
    parent_window_entity: EntityID,
    entity_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let entity_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        "Entities",
        0.25..0.5,
        0.0..0.5,
    )?;

    db.insert_entity_component(entity_list_entity, antigen_components::DebugEntityList)?;
    db.insert_entity_component(
        entity_list_entity,
        antigen_components::EventQueue::<antigen_systems::ListEvent>::default(),
    )?;
    db.insert_entity_component(
        entity_list_entity,
        antigen_components::EventTargets::new(vec![entity_inspector_entity]),
    )?;

    Ok(entity_list_entity)
}

fn create_scene_tree_window<D>(
    db: &mut SystemInterface<D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<D>>,
    parent_window_entity: EntityID,
    entity_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let entity_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        "Scene Tree",
        0.25..0.5,
        0.5..1.0,
    )?;
    db.insert_entity_component(entity_list_entity, antigen_components::DebugSceneTree)?;
    Ok(entity_list_entity)
}

fn create_component_list_window<D>(
    db: &mut SystemInterface<D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<D>>,
    parent_window_entity: EntityID,
    component_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        "Components",
        0.5..0.75,
        0.0..0.5,
    )?;
    db.insert_entity_component(
        component_list_entity,
        antigen_components::DebugComponentList,
    )?;
    db.insert_entity_component(component_list_entity, antigen_components::DebugExclude)?;
    db.insert_entity_component(
        component_list_entity,
        antigen_components::EventQueue::<antigen_systems::ListEvent>::default(),
    )?;
    db.insert_entity_component(
        component_list_entity,
        antigen_components::EventTargets::new(vec![component_inspector_entity]),
    )?;

    Ok(component_list_entity)
}

fn create_component_data_list_window<D>(
    db: &mut SystemInterface<D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<D>>,
    parent_window_entity: EntityID,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let component_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        "Component Data",
        0.75..1.0,
        0.0..1.0,
    )?;
    db.insert_entity_component(
        component_list_entity,
        antigen_components::DebugComponentDataList,
    )?;
    Ok(component_list_entity)
}

fn create_system_list_window<D>(
    db: &mut SystemInterface<D>,
    assemblages: &mut HashMap<EntityAssemblage, Assemblage<D>>,
    parent_window_entity: EntityID,
    system_inspector_entity: EntityID,
) -> Result<EntityID, String>
where
    D: EntityComponentDirectory,
{
    let system_list_entity = create_debug_window(
        db,
        assemblages,
        parent_window_entity,
        "Systems",
        0.5..0.75,
        0.5..1.0,
    )?;
    db.insert_entity_component(system_list_entity, antigen_components::DebugSystemList)?;
    db.insert_entity_component(
        system_list_entity,
        antigen_components::EventQueue::<antigen_systems::ListEvent>::default(),
    )?;
    db.insert_entity_component(
        system_list_entity,
        antigen_components::EventTargets::new(vec![system_inspector_entity]),
    )?;
    Ok(system_list_entity)
}
