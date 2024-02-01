use serde::{Deserialize, Serialize};
use std::{fs, net::SocketAddr};
use teleport_hub::Exit;
use thiserror::Error;

use axum::{
    extract::{ConnectInfo, Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Redirect},
    routing::get,
    Router,
};

#[derive(Serialize, Deserialize, Clone)]
struct Config {
    listen: String,
    exits: Vec<Exit>,
}

#[derive(Debug, Error)]
enum Error {
    #[error("Not found")]
    NotFound,
}

fn set_host_exit(config: &Config, exit_id: &str, host_ip: &str) -> Result<(), Error> {
    match config.exits.iter().find(|e| e.pf_id == exit_id) {
        Some(e) => {
            for exit in &config.exits {
                exit.delete_host(host_ip);
            }
            e.add_host(host_ip);
            Ok(())
        }
        None => Err(Error::NotFound),
    }
}

#[tokio::main]
async fn main() {
    let config: Config = {
        let contents = fs::read_to_string("config.toml").unwrap();
        toml::from_str(&contents).expect("could not parse config toml")
    };

    let listen: SocketAddr = config.listen.parse().unwrap();

    let app = Router::new()
        .route("/", get(index))
        .route("/teleport/:destination", get(teleport))
        .with_state(config);

    axum::Server::bind(&listen)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct ExitsTemplateContext {
    exits: Vec<String>,
}

async fn index(State(config): State<Config>) -> Html<String> {
    // let mut exits: Vec<String> = vec![];
    // for exit in &config.exits {
    //     exits.push(format!(
    //         "<li><a href=\"/teleport/{}\">{}</a></li>",
    //         exit.pf_id, exit.display_name
    //     ));
    // }
    let tpl = mustache::compile_str(include_str!("../templates/hub.html"))
        .expect("compiling html template in index()");
    let html = tpl
        .render_to_string(&config)
        .expect("rendering html template in index()");
    html.into()
}

async fn teleport(
    State(config): State<Config>,
    Path(destination): Path<String>,
    ConnectInfo(client_ip): ConnectInfo<SocketAddr>,
) -> Result<impl IntoResponse, StatusCode> {
    let client_ip = client_ip.ip();
    match set_host_exit(&config, &destination, &client_ip.to_string()) {
        Ok(_) => Ok(Redirect::permanent("/")),
        Err(e) => match e {
            Error::NotFound => Err(StatusCode::NOT_FOUND),
        },
    }
}
