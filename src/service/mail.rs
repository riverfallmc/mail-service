use std::env;
use anyhow::anyhow;
use axum::Json;
use dixxxie::response::{HttpMessage, HttpResult};
use lazy_static::lazy_static;
use lettre::{message::header::ContentType, Message as SmtpMessage, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

lazy_static! {
  static ref EMAIL_ADDRESS: String = env::var("SMTP_USER")
    .expect("The SMTP_USER environment variable was not found!");
}

pub struct MailService;

#[derive(Serialize, Deserialize)]
pub struct Message {
  to: String,
  subject: String,
  body: String,
}

impl MailService {
  pub fn send(
    message: Message,
    transport: SmtpTransport
  ) -> HttpResult<Json<HttpMessage>> {
    let msg = SmtpMessage::builder()
      // по всей видимости, EMAIL_ADDRESS должен быть x@domain.com
      .from(EMAIL_ADDRESS.parse().map_err(|e| anyhow!("Не получилось запарсить EMAIL_ADDRESS: {e}"))?)
      .to(message.to.parse().map_err(|e| anyhow!("Не получилось запарсить поле \"to\": {e}"))?)
      .subject(message.subject)
      .header(ContentType::TEXT_HTML)
      .body(message.body)
      .map_err(|e| anyhow!("Не получилось собрать SmtpMessage: {e}"))?;

    transport.send(&msg)
      .map_err(|e| anyhow!("Не получилось отправить сообщение через SMTP: {e}"))?;

    Ok(Json(HttpMessage {
      message: String::from("Сообщение было успешно отправлено.")
    }))
  }
}