use awc::{cookie::Cookie, http::StatusCode, Client};
use clap::{ArgAction, Parser, Subcommand};

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultReturn<B> {
    pub success: bool,
    pub message: String,
    pub payload: B,
}

// cli
#[derive(Parser, Debug)]
#[command(version, about, long_about = Option::Some("Bundlrs Orbiter CLI"))]
#[command(propagate_version = true)]
struct Orbiter {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Manage authentication (SECONDARY TOKEN ONLY)
    Login { token: String },
    /// View paste
    View { paste: String },
}

// ...
pub mod config;

#[actix_rt::main]
async fn main() {
    // init
    let args = Orbiter::parse();
    let client = Client::default();

    // get current config
    let cnf = config::Configuration::get_config();

    // match commands
    match &args.command {
        // login
        Commands::Login { token } => {
            // check token validity by attempting to login
            let res = client
                .post(format!("{}/api/v1/auth/login-st", cnf.auth_server))
                .timeout(std::time::Duration::from_millis(10_000))
                .append_header(("Content-Type", "application/json"))
                // .cookie(Cookie::new())
                .send_body(
                    serde_json::to_string(&json!({
                        "uid": token
                    }))
                    .unwrap(),
                )
                .await;

            if res.is_err() | (res.as_ref().unwrap().status() != StatusCode::OK) {
                no("Failed to send request! Token may be invalid or the server may be unreachable.");
            }

            // update configuration
            let res = config::Configuration::update_config(config::Configuration {
                server: cnf.server,
                auth_server: cnf.auth_server,
                token: token.to_string(),
                name: cnf.name,
                ..Default::default()
            });

            if res.is_err() {
                no("Failed to write token!");
            } else {
                yes("Token written to configuration!");
            }
        }
        // view
        Commands::View { paste } => no(paste),
    }
}

fn no(msg: &str) -> () {
    println!("\x1b[91m{}\x1b[0m", format!("✘ ⎹ {msg}"));
    std::process::exit(1);
}

fn yes(msg: &str) -> () {
    println!("\x1b[92m{}\x1b[0m", format!("✔ ⎹ {msg}"));
    std::process::exit(0);
}

fn maybe(msg: &str) -> () {
    println!("🛈 ⎹ {}", msg);
}

fn almost(msg: &str) -> () {
    println!("\x1b[94m{}\x1b[0m", format!("🛈 ⎹ {msg}"));
}
