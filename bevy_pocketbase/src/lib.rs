use bevy::prelude::*;
use bevy_ehttp::prelude::*;
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
        app.register_request_type::<HealthCheckResponse>()
            .register_request_type::<AuthSuccessResponse<User>>();
        app.add_event::<events::PocketBaseLoginEvent>();
        app.init_state::<state::PocketbaseStatus>();
        app.add_systems(
            Update,
            (|mut next_state: ResMut<NextState<state::PocketbaseStatus>>| {
                next_state.set(state::PocketbaseStatus::CheckInternet);
            })
            .run_if(resource_added::<PocketbaseClient>),
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
    commands.spawn(RequestBundle::<HealthCheckResponse>::new(
        client.health_check(),
    ));
}

fn connection_response(
    mut commands: Commands,
    client: Res<PocketbaseClient>,
    mut events: EventReader<TypedResponseEvent<HealthCheckResponse>>,
    mut next_state: ResMut<NextState<state::PocketbaseStatus>>,
) {
    for response in events.read() {
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
                commands.spawn(RequestBundle::<HealthCheckResponse>::new(
                    client.health_check(),
                ));
            }
        }
    }
}

fn handle_login(
    mut events: EventReader<TypedResponseEvent<AuthSuccessResponse<User>>>,
    mut next_state: ResMut<NextState<state::PocketbaseStatus>>,
    mut client: ResMut<PocketbaseClient>,
) {
    for response in events.read() {
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
    }
}

fn try_login(
    mut commands: Commands,
    client: Res<PocketbaseClient>,
    mut login_ev: EventReader<events::PocketBaseLoginEvent>,
) {
    for ev in login_ev.read() {
        commands.spawn(RequestBundle::<AuthSuccessResponse<User>>::new(
            client.auth().login(&ev.user_name_or_mail, &ev.password),
        ));
    }
}
