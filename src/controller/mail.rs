use crate::service::mail::{MailService, Message};
use axum::{extract::State, routing::post, Json};
use dixxxie::{controller::Controller, response::{HttpMessage, HttpResult}};
use lettre::SmtpTransport;

pub struct MailController;

impl MailController {
  async fn send(
    State(transport): State<SmtpTransport>,
    Json(message): Json<Message>,
  ) -> HttpResult<Json<HttpMessage>> {
    MailService::send(message, transport)
  }
}

impl Controller<SmtpTransport> for MailController {
  fn register(&self, router: axum::Router<SmtpTransport>) -> axum::Router<SmtpTransport> {
    router
      .route("/send", post(Self::send))
  }
}