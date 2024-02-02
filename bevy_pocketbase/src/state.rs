use bevy::prelude::*;

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum PocketbaseStatus {
    #[default]
    None,
    CheckInternet,
    WaitingForCredentials,
    LoggedIn,
}
