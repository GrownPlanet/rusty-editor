use bevy::{prelude::*, sprite::Anchor, window::WindowResized};

pub fn run_frontend(screen_dimensions: (u16, u16), title: &str) {
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
        .add_systems(Startup, setup)
        .add_systems(Update, on_resize_event)
        .run();
    // .add_systems(Update, (listen_keyboard_input_events, listen_received_character_events, on_resize_system));
}

#[derive(Component)]
struct EditableText;

#[derive(Component)]
struct Cursor;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "How do I do this???? :(", // https://github.com/bevyengine/bevy/blob/main/examples/games/alien_cake_addict.rs this could help
                TextStyle {
                    font: asset_server.load("ubuntu.ttf"),
                    font_size: 11.,
                    ..Default::default()
                },
            )
            .with_justify(JustifyText::Left),
            text_anchor: Anchor::TopLeft,
            ..Default::default()
        },
        EditableText,
    ));
}

fn on_resize_event(
    mut resize_reader: EventReader<WindowResized>,
    mut edit_text: Query<&mut Transform, With<EditableText>>,
) {
    let offset = 15.;
    for e in resize_reader.read() {
        let trans_x = -(e.width / 2. - offset);
        let trans_y = e.height / 2. - offset;

        for mut transform in &mut edit_text {
            transform.translation.x = trans_x;
            transform.translation.y = trans_y;
        }
    }
}
//     let f = fs::read_to_string("text.txt").unwrap();
//
//     commands.spawn(Camera2dBundle::default());
//
//     commands.spawn((
//         Text2dBundle {
//             text: Text::from_section(
//                 f,
//                 TextStyle {
//                     font: asset_server.load("FiraMono-Medium.ttf"),
//                     font_size: 11.0,
//                     ..Default::default()
//                 }
//             )
//             .with_justify(JustifyText::Left),
//             text_anchor: Anchor::TopLeft,
//             ..Default::default()
//         },
//         EditableText
//     ));
// }
//
// fn on_resize_system(
//     mut resize_reader: EventReader<WindowResized>,
//     mut edit_text: Query<&mut Transform, With<EditableText>>,
// ) {
//     let offset = 15.;
//     for e in resize_reader.read() {
//         let trans_x = -(e.width / 2. - offset);
//         let trans_y = e.height / 2. - offset;
//
//         for mut transform in &mut edit_text {
//             transform.translation.x = trans_x;
//             transform.translation.y = trans_y;
//         }
//     }
// }
//
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
