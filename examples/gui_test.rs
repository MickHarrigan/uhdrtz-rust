use bevy::prelude::App;
use bevy_egui::EguiPlugin;
use uhdrtz::prelude::*;
use bevy::DefaultPlugins;
use bevy::ecs::system::ResMut;

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_ui_state)
        .add_system(ui_test)
        .run();
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}