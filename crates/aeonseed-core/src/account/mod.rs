use bevy::prelude::*;

pub mod db;
pub mod session;

pub struct AccountPlugin;

impl Plugin for AccountPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<db::AccountManager>()
            .init_resource::<session::SessionStore>();
    }
}
