//! The [`Bot`] struct and server listener setup.

use crate::core::Core;
use crate::request::CallbackAPIRequest;
use log::{debug, error, info, trace, warn};
use rocket::config::{Config, Environment};
use rocket::http::Status;
use rocket::State;
use rocket_contrib::json::Json;

/// The string `ok` which needs to be sent in response to every Callback API request.
const VK_OK: &'static str = "ok";

/// [`Bot`] represents a chat bot, and hands received requests to [`Core`]
#[derive(Debug, Clone)]
pub struct Bot {
    vk_token: String,
    confirmation_token: String,
    group_id: i32,
    secret: String,
    port: u16,
    core: Core,
}

impl Bot {
    /// Creates a new [`Bot`].
    #[must_use = "the bot does nothing unless started via `.start()`"]
    pub fn new(
        vk_token: &str,
        confirmation_token: &str,
        group_id: i32,
        secret: &str,
        port: u16,
        core: Core,
    ) -> Self {
        Self {
            vk_token: vk_token.into(),
            confirmation_token: confirmation_token.into(),
            group_id,
            secret: secret.into(),
            port,
            core,
        }
    }

    /// Alias for `self.core.handle(req, self.vk_token())`.
    pub fn handle(&self, req: &CallbackAPIRequest) {
        self.core.handle(req, self.vk_token());
    }

    /// Starts this [`Bot`], consuming `self`.
    pub fn start(self) {
        simple_logger::init().unwrap();

        info!("starting bot...");

        rocket::custom(
            Config::build(Environment::Production)
                .address("127.0.0.1")
                .port(self.port)
                .unwrap(),
        )
        .mount("/", routes![post, get])
        .manage(self)
        .launch();
    }

    /// Returns the VK token stored in this [`Bot`].
    pub fn vk_token(&self) -> &String {
        &self.vk_token
    }

    /// Returns the confirmation token stored in this [`Bot`].
    pub fn confirmation_token(&self) -> &String {
        &self.confirmation_token
    }

    /// Returns the group ID stored in this [`Bot`].
    pub fn group_id(&self) -> i32 {
        self.group_id
    }

    /// Returns the secret stored in this [`Bot`].
    pub fn secret(&self) -> &String {
        &self.secret
    }
}

/// Handles `GET` requests by returning [`rocket::http::Status::MethodNotAllowed`]
#[get("/")]
fn get() -> Status {
    debug!("received a GET request");
    Status::MethodNotAllowed
}

/// Handles `POST` requests by first checking that secret and group ID are
/// correct, and then responds with either confirmation token (if that is
/// what was requested) or [`VK_OK`] in the other case.
#[post("/", format = "json", data = "<data>")]
fn post(data: Json<CallbackAPIRequest>, state: State<Bot>) -> Result<String, Status> {
    let bot = &*state;

    if data.secret() != bot.secret() {
        debug!("received a POST request with invalid `secret`");
        Err(Status::BadRequest)
    } else if data.group_id() != bot.group_id() {
        debug!("received a POST request with invalid `group_id`");
        Err(Status::BadRequest)
    } else if data.r#type() == "confirmation" {
        debug!("responded with confirmation token");
        Ok(bot.confirmation_token().clone())
    } else {
        bot.handle(&data);
        Ok(VK_OK.into())
    }
}