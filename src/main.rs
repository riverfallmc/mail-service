use adjust::{controllers, controller::Controller, main, service::Service};
use controller::mail::MailController;
use lettre::SmtpTransport;

mod controller;
mod service;
mod transport;

#[derive(Clone)]
pub struct AppState {
  transport: SmtpTransport
}

#[main]
async fn main() -> Service<'_, AppState> {
  adjust::server::WebServer::enviroment();

  let transport = transport::get_connection()
    .expect("failed to establish connection with smtp server");

  Service {
    name: "Mail",
    state: AppState { transport },
    controllers: controllers![MailController],
    port: Some(1400)
  }
}