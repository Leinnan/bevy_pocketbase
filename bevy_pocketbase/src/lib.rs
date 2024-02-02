use bevy::prelude::*;
use bevy_http_client::typed::{TypedRequest, TypedResponse};
use ehttp_pocketbase::prelude::*;

mod events;
mod state;

pub mod prelude {
    pub use super::events::*;
    pub use super::state::*;
    pub use super::PocketBasePlugin;
    pub use super::PocketbaseClient;
}

#[derive(Default)]
pub struct PocketBasePlugin;

#[derive(Resource, Deref, DerefMut, Default)]
pub struct PocketbaseClient(pub Client<User>);

impl Plugin for PocketBasePlugin {
    fn build(&self, app: &mut App) {
        bevy_http_client::register_request_type::<HealthCheckResponse>(app);
        bevy_http_client::register_request_type::<AuthSuccessResponse<User>>(app);
        app.add_event::<events::PocketBaseLoginEvent>();
        app.add_state::<state::PocketbaseStatus>();
        app.add_systems(
            Update,
            (|mut next_state: ResMut<NextState<state::PocketbaseStatus>>| {
                next_state.set(state::PocketbaseStatus::CheckInternet);
            })
            .run_if(resource_added::<PocketbaseClient>()),
        );
        app.add_systems(
            OnEnter(state::PocketbaseStatus::CheckInternet),
            check_connection,
        );
        app.add_systems(
            Update,
            connection_response.run_if(in_state(state::PocketbaseStatus::CheckInternet)),
        );
        app.add_systems(
            Update,
            (handle_login, try_login)
                .run_if(in_state(state::PocketbaseStatus::WaitingForCredentials)),
        );
    }
}

fn check_connection(mut commands: Commands, client: Res<PocketbaseClient>) {
    commands.spawn(TypedRequest::<HealthCheckResponse>::new(
        client.health_check(),
    ));
}

fn connection_response(
    mut commands: Commands,
    client: Res<PocketbaseClient>,
    q: Query<
        (Entity, &TypedResponse<HealthCheckResponse>),
        Added<TypedResponse<HealthCheckResponse>>,
    >,
    mut next_state: ResMut<NextState<state::PocketbaseStatus>>,
) {
    for (e, response) in q.iter() {
        match response.parse() {
            Some(v) => {
                println!("response: {:?}", v);
                if client.auth_token.is_some() {
                    next_state.set(state::PocketbaseStatus::LoggedIn);
                } else {
                    next_state.set(state::PocketbaseStatus::WaitingForCredentials);
                }
            }
            None => {
                println!("Failed to parse: {:?}", response.result);
                commands.spawn(TypedRequest::<HealthCheckResponse>::new(
                    client.health_check(),
                ));
            }
        }
        commands.entity(e).despawn_recursive();
    }
}

fn handle_login(
    mut commands: Commands,
    responses: Query<(Entity, &TypedResponse<AuthSuccessResponse<User>>)>,
    mut next_state: ResMut<NextState<state::PocketbaseStatus>>,
    mut client: ResMut<PocketbaseClient>,
) {
    for (entity, response) in responses.iter() {
        match response.parse() {
            Some(v) => {
                println!("response: {:?}", v);
                client.auth_token = Some(v.token);
                client.user = Some(v.record);
                next_state.set(state::PocketbaseStatus::LoggedIn);
            }
            None => {
                println!("Failed to parse: {:?}", response.result);
            }
        }
        commands.entity(entity).despawn_recursive();
    }
}

fn try_login(
    mut commands: Commands,
    client: Res<PocketbaseClient>,
    mut login_ev: EventReader<events::PocketBaseLoginEvent>,
) {
    for ev in login_ev.read() {
        commands.spawn(TypedRequest::<AuthSuccessResponse<User>>::new(
            client.auth().login(&ev.user_name_or_mail, &ev.password),
        ));
    }
}
