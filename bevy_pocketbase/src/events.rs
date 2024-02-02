use bevy::prelude::*;

#[derive(Event)]
pub struct PocketBaseLoginEvent {
    pub user_name_or_mail: String,
    pub password: String,
}
