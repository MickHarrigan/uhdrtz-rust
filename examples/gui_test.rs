use bevy::ecs::system::ResMut;
use bevy::prelude::App;
use bevy::DefaultPlugins;
use bevy_egui::EguiPlugin;
use uhdrtz::prelude::*;

fn main() {
    App::new()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(configure_ui_state)
        .add_system(ui_test)
        .add_system(open_window)
        .run();
}

fn configure_ui_state(mut ui_state: ResMut<UiState>) {
    ui_state.is_window_open = true;
}
