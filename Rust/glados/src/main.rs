#[cfg(feature = "ssr")]
use axum::{ extract::Path, Form, Json, response::IntoResponse };
#[cfg(feature = "ssr")]
use serde::*;
#[cfg(feature = "ssr")]
use leptos::*;
use log::error;
use glados::api::handlers;
use crate::state::ServerState;
// use crate::database;
// use serde::Deserialize;

pub mod database;
pub mod structs;
mod api;
mod error_template;
pub mod state;

#[cfg(feature = "ssr")]
#[derive(Deserialize, Debug)]
struct Server {
    uuid: String,
    name: String,
    ip: String,
    port: u16
}

pub fn context() -> ServerState {
    ServerState {}
}

#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::{post, get, put, delete}, Router};
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use glados::app::{*};
    use glados::fileserv::file_and_error_handler;

    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let db = handlers::db_setup().await;
    if let Err(err) = db {
        error!("Failed to setup database. {}", err);
    }

    database::check_if_file_exists();

    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    // _ = ListServers::register();
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let state = ServerState {};

    let addr = leptos_options.site_addr.clone();
    let routes = generate_route_list(App);

    // build our application with a route
    let app = Router::new()
        // .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .route("/api/server", get(api::handlers::get_list_servers).post(api::handlers::post_json_register_server))
        .route("/api/server/:id", get(api::handlers::get_server_by_id).put(api::handlers::put_json_update_server).delete(api::handlers::delete_unregister_server))
        .route("/api/server/status/:id", get(api::handlers::get_get_server_status))
        .route("/api/portal/config", get(api::handlers::get_list_portal_configs).post(api::handlers::post_create_portal_config))
        .route("/api/portal/config/:id", get(api::handlers::get_portal_config).put(api::handlers::put_update_portal_config).delete(api::handlers::delete_portal_config))
        .route("/api/portal", get(api::handlers::get_list_portals).post(api::handlers::post_create_portal))
        .route("/api/portal/:id", get(api::handlers::get_portal).put(api::handlers::put_update_portal).delete(api::handlers::delete_portal))
        // .route("/api/portal", get(api::))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log::info!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
