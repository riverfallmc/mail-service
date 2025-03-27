use anyhow::anyhow;
use axum::Json;
use adjust::{load_env, response::{HttpMessage, HttpResult}};
use lettre::{message::header::ContentType, Message as SmtpMessage, SmtpTransport, Transport};
use serde::{Deserialize, Serialize};

load_env!(EMAIL_ADDRESS);

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
  ) -> HttpResult<HttpMessage> {
    tokio::spawn(async move {
      let msg = SmtpMessage::builder()
        // по всей видимости, EMAIL_ADDRESS должен быть x@domain.com
        .from(EMAIL_ADDRESS.parse().map_err(|e| anyhow!("Не получилось запарсить EMAIL_ADDRESS: {e}"))?)
        .to(message.to.parse().map_err(|e| anyhow!("Не получилось запарсить поле \"to\": {e}"))?)
        .subject(message.subject)
        .header(ContentType::TEXT_HTML)
        .body(message.body)
        .map_err(|e| anyhow!("Не получилось собрать SmtpMessage: {e}"));

      if let Err(err) = msg {
        log::error!("{err}");

        return Err(err)
      }

      // safe unwrap
      let sended = transport.send(&msg.unwrap())
        .map_err(|e| anyhow!("Не получилось отправить сообщение через SMTP: {e}"));

      if let Err(err) = sended {
        log::error!("{err}");
      }

      // короче это нужно для указания return type = anyhow::Result
      let result: anyhow::Result<()> = Ok(());
      result
    });

    Ok(Json(HttpMessage {
      message: String::from("Сообщение было успешно отправлено.")
    }))
  }
}