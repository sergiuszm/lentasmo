use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use log::info;
use rexpect::error::Error;
use rexpect::spawn_bash;
use serde::{Deserialize, Serialize};
use std::env;
use std::process::Command;
use std::str;

const SCREEN_ON: &str = "ON";
const SCREEN_OFF: &str = "OFF";
const USER_ENV: &str = "TASMOTA_USER";
const USER_PASSWD_ENV: &str = "TASMOTA_PASSWORD";
const DISPLAY_ENV: &str = "DISPLAY";

#[derive(Deserialize)]
struct Info {
    user: String,
    password: String,
    cmnd: String,
}

#[derive(Serialize)]
enum Stat {
    #[serde(rename = "POWER1")]
    Power(String),
    #[serde(rename = "Command")]
    Cmd(String),
}

async fn cm(info: web::Query<Info>) -> impl Responder {
    let tasmota_user = std::env::var(USER_ENV).expect("TASMOTA_USER must be set.");
    let tasmota_password = std::env::var(USER_PASSWD_ENV).expect("TASMOTA_PASSWORD must be set.");

    if info.user.as_str() == tasmota_user && info.password.as_str() == tasmota_password {
        match info.cmnd.as_str() {
            "Power1 on" => {
                turn_display("on");
                return web::Json(Stat::Power("On".to_string()));
            }
            "Power1 off" => {
                turn_display("off");
                return web::Json(Stat::Power("Off".to_string()));
            }
            "Power1" => {
                let status = get_display_status().unwrap().to_string();
                return web::Json(Stat::Power(status));
            }
            _ => return web::Json(Stat::Cmd("Error".to_string())),
        }
    }

    web::Json(Stat::Cmd("Error".to_string()))
}

fn set_display() {
    let display_nr = std::env::var(DISPLAY_ENV).expect("DISPLAY must be set.");
    env::set_var(DISPLAY_ENV, &display_nr);
    assert_eq!(env::var(DISPLAY_ENV), Ok(display_nr));
}

fn turn_display(state: &'static str) {
    Command::new("xset")
        .arg("dpms")
        .arg("force")
        .arg(state)
        .output()
        .expect("Error while executing: xset dpms force [on|off]");
}

fn get_display_status() -> Result<&'static str, Error> {
    let mut bash = spawn_bash(Some(2000))?;
    bash.send_line("xset -q|grep \"Monitor is On\"")?;
    let output = bash.read_line()?.find("Monitor is On");
    bash.wait_for_prompt()?;

    if output.is_some() {
        return Ok(SCREEN_ON);
    }

    Ok(SCREEN_OFF)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    dotenv().ok();
    set_display();

    HttpServer::new(|| App::new().route("/cm", web::get().to(cm)))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
