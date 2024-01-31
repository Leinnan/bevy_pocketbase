use std::time::Duration;
use bevy::{app::ScheduleRunnerPlugin, prelude::*};
use bevy_http_client::{typed::TypedResponse, HttpClientPlugin};
use bevy_pocketbase::*;
use ehttp_pocketbase::client::{AuthSuccessResponse, HealthCheckResponse};

fn main() {
    App::new()
            .add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(
                Duration::from_secs_f64(1.0 / 60.0),
            )))
            .add_plugins((HttpClientPlugin,PocketBasePlugin))
            .init_resource::<ApiTimer>()
            .add_systems(Update, (send_request, handle_response,handle_login))
            .add_systems(Startup, |mut ev : EventWriter<PocketBaseInit>|{ ev.send_default()})
            .add_systems(Startup, |mut ev : EventWriter<PocketBaseLogin>|{ ev.send(PocketBaseLogin{user_name_or_mail:"users91599".to_string(),password: "121113456789".to_string()})})
            .run();
}

#[derive(Resource, Deref, DerefMut)]
pub struct ApiTimer(pub Timer);

impl Default for ApiTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(3.0, TimerMode::Repeating))
    }
}

fn send_request(mut _cmd: Commands, time: Res<Time>, mut timer: ResMut<ApiTimer>) {
    timer.tick(time.delta());

    if timer.just_finished() {
        //
    }
}

fn handle_response(
    mut commands: Commands,
    responses: Query<(Entity, &TypedResponse<HealthCheckResponse>)>,
) {
    for (entity, response) in responses.iter() {
        match response.parse() {
            Some(v) => {
                println!("response: {:?}", v);
            }
            None => {
                println!("Failed to parse: {:?}", response.response.text());
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn handle_login(
    mut commands: Commands,
    responses: Query<(Entity, &TypedResponse<AuthSuccessResponse>)>,
) {
    for (entity, response) in responses.iter() {
        match response.parse() {
            Some(v) => {
                println!("response: {:?}", v);
            }
            None => {
                println!("Failed to parse: {:?}", response.response.status);
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}
