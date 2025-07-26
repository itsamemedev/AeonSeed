pub mod cluster_dashboard;

pub struct ToolsPlugin;

impl bevy::prelude::Plugin for ToolsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((cluster_dashboard::DashboardPlugin,));
    }
}
