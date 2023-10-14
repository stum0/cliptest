//! This example illustrates how to create a button that changes color and text based on its
//! interaction state.

// This lint usually gives bad advice in the context of Bevy -- hiding complex queries behind
// type aliases tends to obfuscate code while offering no improvement in code cleanliness.
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use gloo::events;
use gloo::{events::EventListener, timers::callback::Timeout};
use wasm_bindgen::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use web_sys::window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "cliptest".to_string(),
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, button_system)
        .run();
}

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mouse: Res<Input<MouseButton>>,
    touches: Res<Touches>,
) {

    // for (interaction, mut color, mut border_color, children) in &mut interaction_query {
    //     let mut text = text_query.get_mut(children[0]).unwrap();
    //     if mouse.just_pressed(MouseButton::Left) || mouse.just_pressed(MouseButton::Right) {
    //         copyToClipboard("Text to be copied");
    //     }
    //     for touch in touches.iter_just_pressed() {
    //         copyToClipboard("Text to be copied");
    //     }
    //     // match *interaction {
    //     //     Interaction::Pressed => {
    //     //         text.sections[0].value = "Press".to_string();
    //     //         *color = PRESSED_BUTTON.into();
    //     //         border_color.0 = Color::RED;

    //     //         copyToClipboard("Text to be copied");
    //     //     }
    //     //     Interaction::Hovered => {
    //     //         text.sections[0].value = "Hover".to_string();
    //     //         *color = HOVERED_BUTTON.into();
    //     //         border_color.0 = Color::WHITE;
    //     //     }
    //     //     Interaction::None => {
    //     //         text.sections[0].value = "Button".to_string();
    //     //         *color = NORMAL_BUTTON.into();
    //     //         border_color.0 = Color::BLACK;
    //     //     }
    //     // }
    // }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // // ui camera
    // commands.spawn(Camera2dBundle::default());
    // commands
    //     .spawn(NodeBundle {
    //         style: Style {
    //             width: Val::Percent(100.0),
    //             height: Val::Percent(100.0),
    //             align_items: AlignItems::Center,
    //             justify_content: JustifyContent::Center,
    //             ..default()
    //         },
    //         ..default()
    //     })
    //     .with_children(|parent| {
    //         parent
    //             .spawn(ButtonBundle {
    //                 style: Style {
    //                     width: Val::Px(150.0),
    //                     height: Val::Px(65.0),
    //                     border: UiRect::all(Val::Px(5.0)),
    //                     // horizontally center child text
    //                     justify_content: JustifyContent::Center,
    //                     // vertically center child text
    //                     align_items: AlignItems::Center,
    //                     ..default()
    //                 },
    //                 border_color: BorderColor(Color::BLACK),
    //                 background_color: NORMAL_BUTTON.into(),
    //                 ..default()
    //             })
    //             .with_children(|parent| {
    //                 parent.spawn(TextBundle::from_section(
    //                     "Button",
    //                     TextStyle {
    //                         font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                         font_size: 40.0,
    //                         color: Color::rgb(0.9, 0.9, 0.9),
    //                     },
    //                 ));
    //             });
    //     });
}

// #[cfg(target_arch = "wasm32")]
// fn copy_to_clipboard(text: &str) {
//     #[wasm_bindgen]
//     extern "C" {
//         #[wasm_bindgen(js_namespace = ["navigator", "clipboard"], js_name = writeText)]
//         fn write_text(s: &str);
//     }

//     write_text(text);
// }

#[wasm_bindgen]
extern "C" {
    fn copyToClipboard(text: &str);
}

#[wasm_bindgen]
pub fn copy_text(text: &str) {
    copyToClipboard(text);
}
