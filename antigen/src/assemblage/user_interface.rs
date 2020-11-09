use crate::{
    components::EventQueue, components::GlobalPosition, components::GlobalZIndex,
    components::ListData, components::LocalMousePositionData, components::Position,
    components::Size, components::SoftwareShader, components::StringShader,
    systems::LocalMouseMove, systems::LocalMousePress,
};

use super::ComponentBuilder;

fn rect(builder: ComponentBuilder) -> ComponentBuilder {
    builder.fields((Position::default(), Size::default()))
}

fn global_position_z(builder: ComponentBuilder) -> ComponentBuilder {
    builder.fields((GlobalPosition::default(), GlobalZIndex::default()))
}

pub fn string_control(builder: ComponentBuilder) -> ComponentBuilder {
    builder.map(global_position_z).fields((
        Position(Default::default()),
        String::default(),
        StringShader,
    ))
}

pub fn rect_control(builder: ComponentBuilder) -> ComponentBuilder {
    builder
        .map(rect)
        .map(global_position_z)
        .fields((SoftwareShader::color(Default::default()),))
}

pub fn button_control(builder: ComponentBuilder) -> ComponentBuilder {
    builder.map(rect).map(global_position_z).fields((
        EventQueue::<LocalMousePress>::default(),
        EventQueue::<LocalMouseMove>::default(),
        LocalMousePositionData::default(),
    ))
}

pub fn list_control(builder: ComponentBuilder) -> ComponentBuilder {
    builder
        .map(rect)
        .map(global_position_z)
        .fields((Vec::<String>::new(), ListData::default()))
}
