use crate::{service::mail::{MailService, Message}, AppState};
use axum::{extract::State, routing::post, Json};
use adjust::{controller::Controller, response::{HttpMessage, HttpResult}};

pub struct MailController;

impl MailController {
  async fn send(
    State(state): State<AppState>,
    Json(message): Json<Message>,
  ) -> HttpResult<HttpMessage> {
    MailService::send(message, state.transport)
  }
}

impl Controller<AppState> for MailController {
  fn new() -> anyhow::Result<Box<Self>>{
    Ok(Box::new(Self))
  }

  fn register(&self, router: axum::Router<AppState>) -> axum::Router<AppState> {
    router
      .route("/send", post(Self::send))
  }
}