use bevy::prelude::*;
use crate::account::{db::AccountManager, session::{SessionStore, SessionData}};

#[derive(Default, Resource)]
pub struct AuthState {
    pub username: String,
    pub password: String,
    pub logged_in: bool,
}

pub fn auth_ui_system(
    mut commands: Commands,
    mut state: Local<AuthState>,
    mut session: ResMut<SessionStore>,
    mgr: Res<AccountManager>,
) {
    if !state.logged_in {
        // simplistic auto login via console
        if !state.username.is_empty() && !state.password.is_empty() {
            if mgr.verify(&state.username, &state.password) {
                state.logged_in = true;
                session.data = Some(SessionData{token: "dev".into(), username: state.username.clone()});
                session.save();
            }
        }
    }
}
