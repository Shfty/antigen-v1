use crate::systems;

use antigen::{
    components::DebugComponentDataList,
    components::DebugComponentList,
    components::DebugSceneTree,
    components::DebugSystemList,
    components::EventQueue,
    components::{self as antigen_components},
    core::events::AntigenInputEvent,
    core::palette::RGBArrangementPalette,
    entity_component_system::EntityAssembler,
    entity_component_system::{EntityID, SystemAssembler},
    primitive_types::{ColorRGB, ColorRGBF, Vector2I},
    systems as antigen_systems,
};
use antigen_components::{
    Anchors, CPUShader, Control, DebugEntityList, DebugExclude, EventTargets,
    LocalMousePositionData, Margins, Name, ParentEntity, Position, Size, ZIndex,
};
use antigen_curses::{components as curses_components, systems as curses_systems};

use antigen_systems::ListEvent;

use std::ops::Range;

pub fn system_assembler(assembler: SystemAssembler) -> SystemAssembler {
    use system_assemblers::*;

    assembler
        .assemble(event_consumers)
        .assemble(curses)
        .assemble(input)
        .system(antigen_systems::List::default())
        .assemble(event_processors)
        .system(antigen_systems::EventConsumer::<ListEvent>::default())
        .assemble(gameplay)
        .assemble(scene_tree)
        .assemble(rendering)
}

mod system_assemblers {
    use super::*;

    pub fn event_consumers(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(antigen_systems::EventConsumer::<AntigenInputEvent>::default())
            .system(antigen_systems::EventConsumer::<
                curses_components::CursesEvent,
            >::default())
    }

    pub fn curses(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(curses_systems::CursesInputBuffer)
            .system(curses_systems::CursesKeyboard)
            .system(curses_systems::CursesMouse::default())
            .system(curses_systems::CursesWindow)
    }

    pub fn input(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(antigen_systems::LocalMousePosition)
            .system(systems::QuitKey::new(antigen::core::keyboard::Key::Escape))
            .system(systems::InputAxis)
    }

    pub fn event_processors(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(antigen_systems::EventProcessor::<
                ListEvent,
                antigen_systems::EntityInspectorEvent,
            >::new(
                |list_event: ListEvent| match list_event {
                ListEvent::Pressed(index) => Some(
                    antigen_systems::EntityInspectorEvent::SetInspectedEntity(index),
                ),
                _ => None,
            }
            ))
            .system(antigen_systems::EventProcessor::<
                ListEvent,
                antigen_systems::ComponentInspectorEvent,
            >::new(
                |list_event: ListEvent| match list_event {
                ListEvent::Pressed(index) => {
                    Some(antigen_systems::ComponentInspectorEvent::SetInspectedComponent(index))
                }
                _ => None,
            }
            ))
            .system(antigen_systems::EventProcessor::<
                ListEvent,
                antigen_systems::SystemInspectorEvent,
            >::new(
                |list_event: ListEvent| match list_event {
                ListEvent::Pressed(index) => Some(
                    antigen_systems::SystemInspectorEvent::SetInspectedSystem(index),
                ),
                _ => None,
            }
            ))
    }

    pub fn gameplay(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(systems::InputVelocity)
            .system(antigen_systems::PositionIntegrator)
    }

    pub fn scene_tree(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(antigen_systems::AnchorsMargins)
            .system(antigen_systems::GlobalPosition)
            .system(antigen_systems::ChildEntities)
    }

    pub fn rendering(assembler: SystemAssembler) -> SystemAssembler {
        assembler
            .system(antigen_systems::SoftwareRenderer)
            .system(antigen_systems::StringRenderer)
            .system(curses_systems::CursesRenderer::new(
                RGBArrangementPalette::new_884(),
                curses_systems::TextColorMode::BlackWhite,
            ))
    }
}

pub fn entity_assembler(assembler: EntityAssembler) -> EntityAssembler {
    use entity_assemblers::*;

    let entity_inspector_entity = EntityID::next();
    let component_inspector_entity = EntityID::next();
    let system_inspector_entity = EntityID::next();

    // Inspectors
    assembler
        // Inspectors
        .assemble(entity_inspector(entity_inspector_entity))
        .assemble(component_inspector(component_inspector_entity))
        .assemble(system_inspector(component_inspector_entity))
        // Global Event Queues
        .assemble(global_event_queue)
        // Framebuffers
        .assemble(cpu_framebuffer)
        .assemble(string_framebuffer)
        // Main window
        .assemble(main_window(
            Some("Main Window"),
            Position::default(),
            Size(Vector2I(256, 64)),
        ))
        // Subwindows
        .assemble(|assembler: EntityAssembler| {
            let main_window_entity = assembler.current_key();

            assembler
                .assemble(game_window(main_window_entity))
                .assemble(|assembler: EntityAssembler| {
                    let game_window_entity = assembler.current_key();

                    assembler
                        .assemble(test_rect(game_window_entity))
                        .assemble(player(Position(Vector2I::ONE), game_window_entity))
                        .assemble(|assembler: EntityAssembler| {
                            let test_player_entity = assembler.current_key();
                            assembler.assemble(test_string(test_player_entity))
                        })
                })
                .assemble(entity_list_window(
                    main_window_entity,
                    entity_inspector_entity,
                ))
                .assemble(scene_tree_window(main_window_entity))
                .assemble(component_list_window(
                    main_window_entity,
                    component_inspector_entity,
                ))
                .assemble(component_data_list_window(main_window_entity))
                .assemble(system_list_window(
                    main_window_entity,
                    system_inspector_entity,
                ))
        })
}

mod component_assemblers {
    use super::*;

    pub type StringAssemblage = (Name, Control, String, Position);

    pub fn string_control(name: String, string: String, position: Vector2I) -> StringAssemblage {
        (Name(name), Control, string, Position(position))
    }

    pub type RectAssemblage = (Name, Control, Position, Size, char, ColorRGBF);

    pub fn rect_control(name: String, color: ColorRGBF) -> RectAssemblage {
        (
            Name(name),
            Control,
            Position::default(),
            Size::default(),
            char::default(),
            color,
        )
    }

    pub type BorderAssemblage = (Name, Control, Position, Size, CPUShader, ColorRGBF);

    pub fn border_control(name: String) -> BorderAssemblage {
        (
            Name(name),
            Control,
            Position::default(),
            Size::default(),
            CPUShader(CPUShader::rect),
            ColorRGB(0.753f32, 0.753f32, 0.753f32),
        )
    }
}

mod entity_assemblers {
    use super::*;
    use antigen::{components::GlobalPositionData, entity_component_system::MapEntityAssembler};
    use component_assemblers::*;

    pub fn global_event_queue(assembler: EntityAssembler) -> EntityAssembler {
        assembler.key(EntityID::next()).fields((
            Name("Global Event Queues".into()),
            antigen_components::EventQueue::<curses_components::CursesEvent>::default(),
            antigen_components::EventQueue::<AntigenInputEvent>::default(),
        ))
    }

    pub fn entity_inspector(key: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(key).fields((
                Name("Entity Inspector".into()),
                antigen_components::EventQueue::<antigen_systems::EntityInspectorEvent>::default(),
                antigen_components::IntRange::new(-1..0),
            ))
        }
    }

    pub fn component_inspector(key: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(key).fields((
                Name("Component Inspector".into()),
                antigen_components::EventQueue::<antigen_systems::ComponentInspectorEvent>::default(
                ),
                antigen_components::IntRange::new(-1..0),
            ))
        }
    }

    pub fn system_inspector(key: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(key).fields((
                Name("System Inspector".into()),
                antigen_components::EventQueue::<antigen_systems::SystemInspectorEvent>::default(),
                antigen_components::IntRange::new(-1..0),
            ))
        }
    }

    pub fn cpu_framebuffer(assembler: EntityAssembler) -> EntityAssembler {
        assembler.key(EntityID::next()).fields((
            Name("CPU Framebuffer".into()),
            antigen_components::SoftwareFramebuffer::new(ColorRGB(0.0f32, 0.0f32, 0.0f32)),
        ))
    }

    pub fn string_framebuffer(assembler: EntityAssembler) -> EntityAssembler {
        assembler.key(EntityID::next()).fields((
            Name("String Framebuffer".into()),
            antigen_components::SoftwareFramebuffer::new(' '),
        ))
    }

    pub fn main_window(
        debug_label: Option<&'static str>,
        position: Position,
        size: Size,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(EntityID::next()).fields((
                Name(debug_label.unwrap().to_string()),
                antigen_components::Window,
                curses_components::CursesWindowData::default(),
                position,
                size,
            ))
        }
    }

    pub fn game_window(parent_window_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(EntityID::next()).fields((
                Name("Game".into()),
                Position::default(),
                Size::default(),
                ParentEntity(parent_window_entity),
                Anchors::new(0.0..0.25, 0.0..1.0),
            ))
        }
    }

    pub fn test_rect(parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler
                .key(EntityID::next())
                .fields(rect_control(
                    "Test Rect".into(),
                    ColorRGB(0.753f32, 0.753f32, 0.753f32),
                ))
                .fields((
                    Anchors::new(0.0..1.0, 0.0..1.0),
                    antigen_components::CPUShader(antigen_components::CPUShader::hsv),
                    ParentEntity(parent_entity),
                    antigen_components::GlobalPositionData::default(),
                ))
        }
    }

    pub fn player(position: Position, parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler.key(EntityID::next()).fields((
                Control,
                position,
                antigen_components::Velocity(Vector2I::ZERO),
                '@',
                ColorRGB(1.0f32, 0.6f32, 1.0f32),
                ParentEntity(parent_entity),
                GlobalPositionData::default(),
            ))
        }
    }

    pub fn test_string(parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler
                .key(EntityID::next())
                .fields(string_control(
                    "Test String".into(),
                    "Testing One Two Three".into(),
                    Vector2I::ONE,
                ))
                .fields((
                    ParentEntity(parent_entity),
                    antigen_components::GlobalPositionData::default(),
                ))
        }
    }

    pub fn debug_window(
        window_name: &'static str,
        anchor_horizontal: Range<f32>,
        anchor_vertical: Range<f32>,
        parent_entity: EntityID,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler|
    // Window Entity
    assembler
        .assemble(window(anchor_horizontal, anchor_vertical, parent_entity))
        .assemble(|assembler: EntityAssembler| {
            let window_entity = assembler.current_key();

            // Border Entity
            assembler
                .key(EntityID::next())
                .fields(border_control("Border".into()))
                .fields((
                    Anchors::new(0.0..1.0, 0.0..1.0),
                    ParentEntity(window_entity),
                ))
                .assemble(|assembler: EntityAssembler| {
                    let border_entity = assembler.current_key();

                    assembler
                        .assemble(window_title(window_name, border_entity))
                        .assemble(list(window_name.into(), border_entity))
                })
        })
    }
    pub fn window(
        anchor_horizontal: Range<f32>,
        anchor_vertical: Range<f32>,
        parent_entity: EntityID,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler
                .key(EntityID::next())
                .fields(rect_control(
                    "Debug Window".into(),
                    ColorRGB(0.0f32, 0.0f32, 0.0f32),
                ))
                .fields((
                    Anchors::new(anchor_horizontal, anchor_vertical),
                    ZIndex(1),
                    ParentEntity(parent_entity),
                ))
        }
    }

    pub fn window_title(title: &'static str, parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler
                .key(EntityID::next())
                .fields(string_control(
                    "Debug Window Title".into(),
                    format!("{}\n========", title),
                    Vector2I(2, 1),
                ))
                .fields((
                    ParentEntity(parent_entity),
                    antigen_components::GlobalPositionData::default(),
                ))
        }
    }

    pub fn list(title: String, parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            assembler
                .key(EntityID::next())
                .assemble(|assembler: EntityAssembler| {
                    let list_entity = assembler.current_key();

                    assembler.key(list_entity).fields((
                        Name(title),
                        Position::default(),
                        Size::default(),
                        ParentEntity(parent_entity),
                        Anchors::new(0.0..1.0, 0.0..1.0),
                        Margins::new(2, 2, 3, 1),
                        Vec::<String>::new(),
                        LocalMousePositionData::default(),
                        antigen_components::ListData::new(Some(list_entity)),
                    ))
                })
        }
    }

    pub fn entity_list_window(
        parent_entity: EntityID,
        entity_inspector_entity: EntityID,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            debug_window("Entities", 0.25..0.5, 0.0..0.5, parent_entity)(assembler).fields((
                DebugEntityList,
                EventQueue::<ListEvent>::default(),
                EventTargets::new(vec![entity_inspector_entity]),
            ))
        }
    }

    pub fn scene_tree_window(parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            debug_window("Scene Tree", 0.25..0.5, 0.5..1.0, parent_entity)(assembler)
                .field(DebugSceneTree)
        }
    }

    pub fn component_list_window(
        parent_entity: EntityID,
        component_inspector_entity: EntityID,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            debug_window("Components", 0.5..0.75, 0.0..0.5, parent_entity)(assembler).fields((
                DebugComponentList,
                DebugExclude,
                EventQueue::<ListEvent>::default(),
                EventTargets::new(vec![component_inspector_entity]),
            ))
        }
    }

    pub fn component_data_list_window(parent_entity: EntityID) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            debug_window("Component Data", 0.75..1.0, 0.0..1.0, parent_entity)(assembler)
                .field(DebugComponentDataList)
        }
    }

    pub fn system_list_window(
        parent_entity: EntityID,
        system_inspector_entity: EntityID,
    ) -> impl MapEntityAssembler {
        move |assembler: EntityAssembler| {
            debug_window("Systems", 0.5..0.75, 0.5..1.0, parent_entity)(assembler).fields((
                DebugSystemList,
                EventQueue::<ListEvent>::default(),
                EventTargets::new(vec![system_inspector_entity]),
            ))
        }
    }
}
