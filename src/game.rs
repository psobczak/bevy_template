use bevy::prelude::*;

#[cfg(debug_assertions)]
use bevy::input::common_conditions::input_toggle_active;
#[cfg(debug_assertions)]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".into(),
                ..Default::default()
            }),
            ..Default::default()
        }));

        #[cfg(debug_assertions)]
        app.add_plugin(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Slash)),
        );

        app.add_system(setup.on_startup());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
