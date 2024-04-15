use bevy::{
    prelude::*,
    sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle},
    window::WindowResized,
};

use crate::editor::Editor;

pub fn run_frontend(screen_dimensions: (u16, u16), title: &str, editor: Editor) {
    App::new()
        .add_plugins((DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: title.into(),
                name: Some(title.into()),
                resolution: screen_dimensions.into(),
                ..default()
            }),
            ..default()
        }),))
        .insert_resource(ClearColor(Color::rgb(0.153, 0.173, 0.208)))
        .insert_resource(editor)
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_event)
        .run();
    // .add_systems(Update, (listen_keyboard_input_events, listen_received_character_events, on_resize_system));
}

#[derive(Component)]
struct EditableText;

#[derive(Component)]
struct Cursor;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    editor: ResMut<Editor>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    let foreground_color = Color::rgb(0.667, 0.698, 0.753);

    let font = asset_server.load("JetBrainsMono.ttf");
    let font_size = 12.;

    // the text we are editing
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                editor.get_text(),
                TextStyle {
                    font,
                    font_size,
                    color: foreground_color,
                    ..default()
                },
            )
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::TopLeft,
            ..default()
        },
        EditableText,
    ));

    // the cursor
    let cursor_size = (font_size / 10., font_size);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(cursor_size.0, cursor_size.1))),
            material: materials.add(foreground_color),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Cursor
    ));
}

fn on_resize_event(
    mut resize_reader: EventReader<WindowResized>,
    mut edit_text: Query<&mut Transform, With<EditableText>>,
    mut cursor: Query<&mut Transform, (With<Cursor>, Without<EditableText>)>,
    editor: ResMut<Editor>,
) {
    for e in resize_reader.read() {
        let transformation_x = -(e.width / 2.);
        let transformation_y = e.height / 2.;

        for mut transform in &mut edit_text {
            transform.translation.x = transformation_x;
            transform.translation.y = transformation_y;
        }

        // TODO: 
        // - center the cursor
        // - give cursor_pos the width and height of characters so it's positioned properly
        let cursor_pos = editor.get_cursor_pos();
        for mut transform in &mut cursor {
            transform.translation.x = transformation_x + cursor_pos.0 as f32;
            transform.translation.y = transformation_y - cursor_pos.1 as f32;
        }
    }
}

// fn listen_received_character_events(
//     mut events: EventReader<ReceivedCharacter>,
//     mut edit_text: Query<&mut Text, With<EditableText>>,
// ) {
//     for event in events.read() {
//         edit_text.single_mut().sections[0]
//             .value
//             .push_str(&event.char);
//     }
// }
//
// fn listen_keyboard_input_events(
//     mut events: EventReader<KeyboardInput>,
//     mut edit_text: Query<&mut Text, With<EditableText>>,
// ) {
//     for event in events.read() {
//         match event.key_code {
//             KeyCode::Backspace => {
//                 edit_text.single_mut().sections[0].value.pop();
//             }
//             _ => continue,
//         }
//     }
// }
