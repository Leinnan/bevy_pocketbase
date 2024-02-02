use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_http_client::HttpClientPlugin;
use bevy_pocketbase::*;
use bevy_pocketbase::state::PocketbaseStatus;

use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            ))),
        )
        .add_plugins((HttpClientPlugin, PocketBasePlugin))
        .init_resource::<ApiTimer>()
        .add_systems(Update, send_request)
        .add_systems(
            OnEnter(PocketbaseStatus::WaitingForCredentials),
            try_login,
        )
        // .add_systems(Startup, |mut ev : EventWriter<PocketBaseLogin>|{ ev.send(PocketBaseLogin{user_name_or_mail:"users91599".to_string(),password: "121113456789".to_string()})})
        .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct ApiTimer(pub Timer);

impl Default for ApiTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Once))
    }
}

fn send_request(mut _cmd: Commands, time: Res<Time>, mut timer: ResMut<ApiTimer>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        _cmd.insert_resource(PocketbaseClient::default());
    }
}

fn try_login(mut ev: EventWriter<PocketBaseLogin>) {
    ev.send(PocketBaseLogin {
        user_name_or_mail: "users91599".to_string(),
        password: "121113456789".to_string(),
    });
}
