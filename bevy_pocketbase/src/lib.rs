use bevy::prelude::*;
use bevy_http_client::typed::TypedRequest;
use ehttp_pocketbase::client::{AuthSuccessResponse, Client, HealthCheckResponse};

/// Add the plugin to bevy to support send http request and handle response.
///
/// # Example
/// ```no_run
/// # use bevy::prelude::*;
/// # use bevy_pocketbase::HttpClientPlugin;
///
/// App::new()
/// .add_plugins(DefaultPlugins)
/// .add_plugins(HttpClientPlugin).run();
/// ```
#[derive(Default)]
pub struct PocketBasePlugin;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PocketbaseClient(pub Client);

#[derive(Event,Default)]
pub struct PocketBaseInit;

#[derive(Event)]
pub struct PocketBaseLogin {
    pub user_name_or_mail: String,
    pub password: String
}

impl Plugin for PocketBasePlugin {
    fn build(&self, app: &mut App) {
        bevy_http_client::register_request_type::<HealthCheckResponse>(app);
        bevy_http_client::register_request_type::<AuthSuccessResponse>(app);
        app.add_event::<PocketBaseInit>();
        app.add_event::<PocketBaseLogin>();
        app.add_systems(Update, init);
        if !app.world.contains_resource::<PocketbaseClient>() {
            app.init_resource::<PocketbaseClient>();
        }
    }
}

fn init(mut commands: Commands,
        mut ev: EventReader<PocketBaseInit>,
        mut login_ev: EventReader<PocketBaseLogin>,
        client: Res<PocketbaseClient>) {
    for _ in ev.read() {
        commands.spawn(TypedRequest::<HealthCheckResponse>::new(
            client.health_check(),
        ));
    }
    for ev in login_ev.read() {
        commands.spawn(TypedRequest::<AuthSuccessResponse>::new(client.auth_with_password("users", &ev.user_name_or_mail, &ev.password)));
    }
}