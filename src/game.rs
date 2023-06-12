use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Game".into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_system(setup.on_startup());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
