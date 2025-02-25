use std::env;

use anyhow::{Context, Result};
use lettre::{transport::smtp::authentication::Credentials, SmtpTransport};

pub fn get_connection() -> Result<SmtpTransport> {
  let binding = env::var("SMTP_USER")?;
  let binding = binding
    .split("@")
    .collect::<Vec<&str>>();

  let smtp_user = binding
    .first()
    .context(anyhow::anyhow!("Unable to get SMTP Username (before @)"))?;

  let transport = SmtpTransport::relay(&env::var("SMTP_HOST")?)?
    .credentials(Credentials::new(smtp_user.to_string(), env::var("SMTP_PASS")?))
    .build();

  log::debug!("testing SMTP connection");

  transport.test_connection()?;

  log::debug!("smtp connection is stable");

  Ok(transport)
}