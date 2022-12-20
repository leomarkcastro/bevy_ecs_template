// To describe how the SimpleText component/entity should behave.
// WILL: contain pure logic that interacts with the component

use bevy::prelude::*;

use crate::entity_factory::factory::data::{
    SpawnEntityEvent, SpawnUIEvent, UIEntity, UIEntityData,
};

use super::{SimpleTextComponent, SimpleTextEntity};

pub struct SimpleTextPlugin;

impl Plugin for SimpleTextPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn simple_text_spawn(
    mut commands: &mut Commands,
    spawn_entity_event: &SpawnUIEvent,
    asset_server: &Res<AssetServer>,
) {
    let extract_spawn_data = &spawn_entity_event.entitydata;

    let UIEntityData::SimpleText {
        text,
        font,
        font_size,
        alignment,
        color,
    } = extract_spawn_data;

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            text.to_string(),
            TextStyle {
                font: asset_server.load(font.to_string()),
                font_size: *font_size,
                color: *color,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(*alignment)
        // Set the style of the TextBundle itself.
        .with_style(spawn_entity_event.style.to_owned()),
    ));
}
