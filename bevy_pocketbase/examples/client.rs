use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_http_client::HttpClientPlugin;
use bevy_pocketbase::prelude::*;

use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins((HttpClientPlugin, PocketBasePlugin))
        .init_resource::<PocketbaseClient>()
        .add_systems(OnEnter(PocketbaseStatus::WaitingForCredentials), try_login)
        .run();
}

fn try_login(mut ev: EventWriter<PocketBaseLoginEvent>) {
    ev.send(PocketBaseLoginEvent {
        user_name_or_mail: "users91599".to_string(),
        password: "121113456789".to_string(),
    });
}
