use crate::{
    assemblage::{color_framebuffer, string_framebuffer},
    systems,
};

use antigen::{
    assemblage::{button_control, list_control, string_control, EntityBuilder},
    components::{
        self as antigen_components, DebugComponentDataList, DebugComponentList, DebugSceneTree,
        DebugSystemList, EventQueue,
    },
    core::palette::RGBArrangementPalette,
    entity_component_system::{EntityID, SystemBuilder},
    primitive_types::{ColorRGB, ColorRGBF, Vector2I},
    systems as antigen_systems,
    systems::LocalMousePress,
};
use antigen_components::{
    Anchors, Connection, DebugEntityList, DebugExclude, LocalMousePositionData, Margins, Name,
    ParentEntity, Position, Size, SoftwareShader, StringShader, ZIndex,
};
use antigen_curses::{
    assemblage::curses,
    components::{self as curses_components, CursesPalette},
    systems as curses_systems,
    systems::CursesColorsEvent,
};

use antigen_systems::ListEvent;

use std::ops::Range;

use self::component_builders::*;

pub fn system_assembler(builder: SystemBuilder) -> SystemBuilder {
    use system_builders::*;

    builder
        .map(event_consumers)
        .map(curses)
        .map(input)
        .map(user_interface)
        .map(event_processors)
        .map(gameplay)
        .map(scene_tree)
        .map(rendering)
}

pub fn entity_assembler(builder: EntityBuilder) -> EntityBuilder {
    use entity_builders::*;

    let entity_inspector_entity = EntityID::next();
    let component_inspector_entity = EntityID::next();
    let system_inspector_entity = EntityID::next();

    let curses_palette = EntityID::next();

    // Inspectors
    builder
        // Inspectors
        .map_key(entity_inspector_entity, entity_inspector)
        .map_key(component_inspector_entity, component_inspector)
        .map_key(system_inspector_entity, system_inspector)
        // Curses palette
        .key_fields(
            curses_palette,
            (
                Name("Curses Palette".into()),
                CursesPalette::default(),
                EventQueue::<CursesColorsEvent>::new(vec![CursesColorsEvent::SetPalette(
                    RGBArrangementPalette::new_884(),
                )]),
            ),
        )
        // Global Event Queues
        .map_key(EntityID::next(), global_event_queue)
        // Framebuffers
        .map_key(EntityID::next(), color_framebuffer)
        .map_key(EntityID::next(), string_framebuffer)
        // Main window
        .key(EntityID::next())
        .map(main_window)
        .fields((Name("Main Window".into()), Size(Vector2I(256, 64))))
        .finish()
        // Subwindows
        .map_current_key(|builder: EntityBuilder, main_window_entity: EntityID| {
            builder
                // Game window
                .key(EntityID::next())
                .map(game_window)
                .field(ParentEntity(main_window_entity))
                .finish()
                .map_current_key(|builder: EntityBuilder, game_window_entity: EntityID| {
                    builder
                        // Test Rect
                        .key(EntityID::next())
                        .map(test_rect)
                        .field(ParentEntity(game_window_entity))
                        .finish()
                        // Player
                        .key(EntityID::next())
                        .map(player)
                        .fields((
                            Name("Player".into()),
                            Position(Vector2I::ONE),
                            ParentEntity(game_window_entity),
                        ))
                        .finish()
                        .map_current_key(|builder: EntityBuilder, test_player_entity: EntityID| {
                            // Test String
                            builder
                                .key(EntityID::next())
                                .map(string_control)
                                .fields((
                                    Name("Test String".into()),
                                    Position(Vector2I(2, 1)),
                                    "Testing One Two Three...".to_string(),
                                    ParentEntity(test_player_entity),
                                ))
                                .finish()
                        })
                        // Button
                        .map(palette_button(
                            0.0..0.25,
                            1.0..1.0,
                            "RGB666",
                            Connection::new(vec![curses_palette], true, |_: LocalMousePress| {
                                Some(CursesColorsEvent::SetPalette(
                                    RGBArrangementPalette::new_666(),
                                ))
                            }),
                            game_window_entity,
                        ))
                        .map(palette_button(
                            0.25..0.5,
                            1.0..1.0,
                            "RGB676",
                            Connection::new(vec![curses_palette], true, |_: LocalMousePress| {
                                Some(CursesColorsEvent::SetPalette(
                                    RGBArrangementPalette::new_676(),
                                ))
                            }),
                            game_window_entity,
                        ))
                        .map(palette_button(
                            0.5..0.75,
                            1.0..1.0,
                            "RGB685",
                            Connection::new(vec![curses_palette], true, |_: LocalMousePress| {
                                Some(CursesColorsEvent::SetPalette(
                                    RGBArrangementPalette::new_685(),
                                ))
                            }),
                            game_window_entity,
                        ))
                        .map(palette_button(
                            0.75..1.0,
                            1.0..1.0,
                            "RGB884",
                            Connection::new(vec![curses_palette], true, |_: LocalMousePress| {
                                Some(CursesColorsEvent::SetPalette(
                                    RGBArrangementPalette::new_884(),
                                ))
                            }),
                            game_window_entity,
                        ))
                })
                // Entity List
                .map(entity_list_window(
                    main_window_entity,
                    entity_inspector_entity,
                ))
                // Scene Tree
                .map(scene_tree_window(main_window_entity))
                // Component List
                .map(component_list_window(
                    main_window_entity,
                    component_inspector_entity,
                ))
                // Component Data List
                .map(component_data_list_window(main_window_entity))
                // System List
                .map(system_list_window(
                    main_window_entity,
                    system_inspector_entity,
                ))
        })
}

mod system_builders {
    use antigen::{
        components::{GlobalPosition, GlobalZIndex},
        core::events::{KeyPress, KeyRelease, MouseMove, MousePress, MouseRelease, MouseScroll},
        systems::LocalMouseMove,
        systems::{
            ColorRenderer, EventConsumer, EventProcessor, LocalMousePress, LocalMouseRelease,
            LocalMouseScroll, SceneTreeData,
        },
    };
    use antigen_systems::{AnchorsMargins, ChildEntities, PositionIntegrator, StringRenderer};
    use curses_systems::CursesRenderer;
    use systems::InputVelocity;

    use super::*;

    pub fn event_consumers(builder: SystemBuilder) -> SystemBuilder {
        builder
            .system(EventConsumer::<MouseMove>::default())
            .system(EventConsumer::<MousePress>::default())
            .system(EventConsumer::<MouseRelease>::default())
            .system(EventConsumer::<MouseScroll>::default())
            .system(EventConsumer::<KeyPress>::default())
            .system(EventConsumer::<KeyRelease>::default())
            .system(EventConsumer::<LocalMouseMove>::default())
            .system(EventConsumer::<LocalMousePress>::default())
            .system(EventConsumer::<LocalMouseRelease>::default())
            .system(EventConsumer::<LocalMouseScroll>::default())
            .system(EventConsumer::<curses_components::CursesEvent>::default())
            .system(EventConsumer::<ListEvent>::default())
    }

    pub fn input(builder: SystemBuilder) -> SystemBuilder {
        builder
            .system(antigen_systems::LocalMousePosition)
            .system(antigen_systems::LocalMouseEvents)
            .system(systems::QuitKey::new(antigen::core::keyboard::Key::Escape))
            .system(systems::InputAxis)
    }

    pub fn user_interface(builder: SystemBuilder) -> SystemBuilder {
        builder
            .system(antigen_systems::List::new(|builder| {
                builder.field(DebugExclude)
            }))
            .system(antigen_systems::LocalMouseEvents)
    }

    pub fn event_processors(builder: SystemBuilder) -> SystemBuilder {
        builder.system(EventProcessor)
    }

    pub fn gameplay(builder: SystemBuilder) -> SystemBuilder {
        builder.system(InputVelocity).system(PositionIntegrator)
    }

    pub fn scene_tree(builder: SystemBuilder) -> SystemBuilder {
        builder
            .system(AnchorsMargins)
            .system(ChildEntities)
            .system(SceneTreeData::<Position, GlobalPosition>::default())
            .system(SceneTreeData::<ZIndex, GlobalZIndex>::default())
    }

    pub fn rendering(builder: SystemBuilder) -> SystemBuilder {
        builder
            .system(ColorRenderer::<ColorRGBF>::default())
            .system(StringRenderer)
            .system(CursesRenderer::default())
    }
}

mod component_builders {
    use super::*;
    use antigen::{
        assemblage::{rect_control, ComponentBuilder},
        components::{GlobalPosition, GlobalZIndex, IntRange, Window},
        core::events::MouseMove,
        core::events::{KeyPress, KeyRelease, MousePress, MouseRelease, MouseScroll},
    };
    use curses_components::{CursesEvent, CursesWindowData};

    pub fn player(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            StringShader,
            antigen_components::Velocity(Vector2I::ZERO),
            '@',
            ColorRGB(1.0f32, 0.6f32, 1.0f32),
            GlobalPosition::default(),
            GlobalZIndex::default(),
        ))
    }

    pub fn global_event_queue(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            Name("Global Event Queues".into()),
            EventQueue::<CursesEvent>::default(),
            EventQueue::<MouseMove>::default(),
            EventQueue::<MouseScroll>::default(),
            EventQueue::<MousePress>::default(),
            EventQueue::<MouseRelease>::default(),
            EventQueue::<KeyPress>::default(),
            EventQueue::<KeyRelease>::default(),
        ))
    }

    pub fn entity_inspector(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            Name("Entity Inspector".into()),
            EventQueue::<antigen_systems::EntityInspectorEvent>::default(),
            IntRange::new(-1..0),
        ))
    }

    pub fn component_inspector(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            Name("Component Inspector".into()),
            EventQueue::<antigen_systems::ComponentInspectorEvent>::default(),
            IntRange::new(-1..0),
        ))
    }

    pub fn system_inspector(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            Name("System Inspector".into()),
            EventQueue::<antigen_systems::SystemInspectorEvent>::default(),
            IntRange::new(-1..0),
        ))
    }

    pub fn main_window(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((Window, CursesWindowData::default()))
    }

    pub fn game_window(builder: ComponentBuilder) -> ComponentBuilder {
        builder.fields((
            Name("Game".into()),
            Position::default(),
            ZIndex::default(),
            Size::default(),
            Anchors::new(0.0..0.25, 0.0..1.0),
        ))
    }

    pub fn test_rect(builder: ComponentBuilder) -> ComponentBuilder {
        builder.map(rect_control).fields((
            Name("Test Rect".into()),
            Anchors::new(0.0..1.0, 0.0..1.0),
            SoftwareShader::hsv(),
        ))
    }
}

mod entity_builders {
    use super::*;
    use antigen::{
        assemblage::rect_control,
        assemblage::MapEntityBuilder,
        systems::{LocalMousePress, LocalMouseScroll},
    };

    pub fn palette_button(
        anchor_horizontal: Range<f32>,
        anchor_vertical: Range<f32>,
        text: &'static str,
        connection: Connection,
        parent_entity: EntityID,
    ) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .key(EntityID::next())
                .map(button_control)
                .fields((
                    Name(text.to_owned() + " Button"),
                    Anchors::new(anchor_horizontal, anchor_vertical),
                    Margins::new(1, 1, -4, 1),
                    SoftwareShader::color(ColorRGB(0.25, 0.25, 0.25)),
                    connection,
                    ParentEntity(parent_entity),
                ))
                .finish()
                .map_current_key(|builder: EntityBuilder, button_entity: EntityID| {
                    // Button Text
                    builder
                        .key(EntityID::next())
                        .map(string_control)
                        .fields((
                            Name("Button Text".into()),
                            Position(Vector2I(4, 1)),
                            text.to_string(),
                            ParentEntity(button_entity),
                        ))
                        .finish()
                })
        }
    }

    pub fn debug_window(
        window_name: &'static str,
        anchor_horizontal: Range<f32>,
        anchor_vertical: Range<f32>,
        parent_entity: EntityID,
    ) -> impl MapEntityBuilder {
        move |builder: EntityBuilder|
    // Window Entity
    builder
        .key(EntityID::next())
        .map(rect_control).fields((
            Name("Debug Window".into()),
            SoftwareShader::color(ColorRGB(0.0f32, 0.0f32, 0.0f32)),
            ZIndex(1),
            Anchors::new(anchor_horizontal, anchor_vertical),
            ParentEntity(parent_entity),
        ))
        .finish()
        .map_current_key(|builder: EntityBuilder, window_entity: EntityID| {
            // Border Entity
            builder
                .key(EntityID::next())
                .map(rect_control)
                .fields((
                    Name("Border".into()),
                    SoftwareShader::rect(ColorRGB(0.753f32, 0.753f32, 0.753f32)),
                    ZIndex::default(),
                    Anchors::new(0.0..1.0, 0.0..1.0),
                    ParentEntity(window_entity),
                ))
                .finish()
                .map_current_key(|builder: EntityBuilder, border_entity| {
                    builder
                        // Title
                        .key(EntityID::next())
                        .map(string_control)
                        .fields((
                            Name("Debug Window Title".into()),
                            Position(Vector2I(2, 1)),
                            format!("{}\n========", window_name),
                            ParentEntity(border_entity),
                        ))
                        .finish()
                        // List
                        .key(EntityID::next())
                        .map(list_control)
                        .fields((
                            Name(window_name.into()),
                            Anchors::new(0.0..1.0, 0.0..1.0),
                            Margins::new(2, 2, 3, 1),
                            LocalMousePositionData::default(),
                            EventQueue::<LocalMousePress>::default(),
                            EventQueue::<LocalMouseScroll>::default(),
                            ParentEntity(border_entity),
                        ))
                        .finish()
                    })
        })
    }

    pub fn entity_list_window(
        parent_entity: EntityID,
        entity_inspector_entity: EntityID,
    ) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .map(debug_window("Entities", 0.25..0.5, 0.0..0.5, parent_entity))
                .map_current_key(|builder: EntityBuilder, list_entity: EntityID| {
                    builder.key_fields(
                        list_entity,
                        (
                            DebugEntityList,
                            EventQueue::<ListEvent>::default(),
                            Connection::new(
                                vec![entity_inspector_entity],
                                false,
                                |list_event: ListEvent| match list_event {
                                    ListEvent::Pressed(index) => Some(
                                        antigen_systems::EntityInspectorEvent::SetInspectedEntity(
                                            index,
                                        ),
                                    ),
                                    _ => None,
                                },
                            ),
                        ),
                    )
                })
        }
    }

    pub fn scene_tree_window(parent_entity: EntityID) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .map(debug_window(
                    "Scene Tree",
                    0.25..0.5,
                    0.5..1.0,
                    parent_entity,
                ))
                .map_current_key(|builder: EntityBuilder, list_entity: EntityID| {
                    builder.key_field(list_entity, DebugSceneTree)
                })
        }
    }

    pub fn component_list_window(
        parent_entity: EntityID,
        component_inspector_entity: EntityID,
    ) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .map(debug_window(
                    "Components",
                    0.5..0.75,
                    0.0..0.5,
                    parent_entity,
                ))
                .map_current_key(|builder: EntityBuilder, list_entity: EntityID| {
                    builder.key_fields(
                        list_entity,
                        (
                            DebugComponentList,
                            DebugExclude,
                            EventQueue::<ListEvent>::default(),
                            Connection::new(vec![component_inspector_entity], false, |list_event: ListEvent| match list_event {
                                ListEvent::Pressed(index) => {
                                    Some(antigen_systems::ComponentInspectorEvent::SetInspectedComponent(index))
                                }
                                _ => None,
                            }),
                        ),
                    )
                })
        }
    }

    pub fn component_data_list_window(parent_entity: EntityID) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .map(debug_window(
                    "Component Data",
                    0.75..1.0,
                    0.0..1.0,
                    parent_entity,
                ))
                .map_current_key(|builder: EntityBuilder, list_entity: EntityID| {
                    builder.key_field(list_entity, DebugComponentDataList)
                })
        }
    }

    pub fn system_list_window(
        parent_entity: EntityID,
        system_inspector_entity: EntityID,
    ) -> impl MapEntityBuilder {
        move |builder: EntityBuilder| {
            builder
                .map(debug_window("Systems", 0.5..0.75, 0.5..1.0, parent_entity))
                .map_current_key(|builder: EntityBuilder, list_entity: EntityID| {
                    builder.key_fields(
                        list_entity,
                        (
                            DebugSystemList,
                            EventQueue::<ListEvent>::default(),
                            Connection::new(
                                vec![system_inspector_entity],
                                false,
                                |list_event: ListEvent| match list_event {
                                    ListEvent::Pressed(index) => Some(
                                        antigen_systems::SystemInspectorEvent::SetInspectedSystem(
                                            index,
                                        ),
                                    ),
                                    _ => None,
                                },
                            ),
                        ),
                    )
                })
        }
    }
}
