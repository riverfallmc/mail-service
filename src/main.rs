use std::env;
use dixxxie::{
  axum::{self, Router}, controller::ApplyControllerOnRouter, setup
};
use anyhow::{Context, Result};
use controller::mail::MailController;
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

mod controller;
mod service;

#[tokio::main]
async fn main() -> Result<()> {
  setup()?;

  let binding = env::var("SMTP_USER")?;
  let binding = binding
    .split("@")
    .collect::<Vec<&str>>();

  let smtp_user = binding
    .get(0)
    .context(anyhow::anyhow!("Unable to get SMTP Username (before @)"))?;

  let connection = SmtpTransport::relay(&env::var("SMTP_HOST")?)?
    .credentials(Credentials::new(smtp_user.to_string(), env::var("SMTP_PASS")?))
    .build();

  log::debug!("Testing SMTP connection");

  connection.test_connection()?;

  log::debug!("Test... OK");

  let router = Router::new()
    .apply_controller(MailController)
    .with_state(connection);

  let listener = tokio::net::TcpListener::bind("0.0.0.0:80")
    .await?;

  Ok(axum::serve(listener, router).await?)
}