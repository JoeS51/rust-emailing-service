use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    SmtpTransport, Transport,
    message::{Message, Mailbox}
};
use dotenvy::dotenv;

use azure_messaging_servicebus::prelude::*;
use azure_core::Result;
use std::sync::Arc;

fn create_mailer() -> SmtpTransport {
    let username = std::env::var("EMAIL_USERNAME").expect("EMAIL_USERNAME not set");
    let password = std::env::var("EMAIL_PASSWORD").expect("EMAIL_PASSWORD not set");
    println!("user name is {username}");

    let creds = Credentials::new(username, password);

    SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build()
}

fn send_email(body: String) {
    let email = Message::builder()
        .from("joesluis51@gmail.com".parse::<Mailbox>().unwrap())
        .to("joesluis51@gmail.com".parse::<Mailbox>().unwrap())
        .subject("email with rust")
        .body(body)
        .unwrap();

    let mailer = create_mailer();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent!"),
        Err(error) => {
            println!("Email didn't send. {:?}", error);
        }
    }
}

async fn get_message_and_send_email() -> Result<()> {
     let http_client: Arc<dyn azure_core::HttpClient> = Arc::new(reqwest::Client::new());
     let service_bus_namespace = std::env::var("SERVICE_BUS_NAMESPACE").expect("SERVICE_BUS_NAMESPACE not set");
     let queue_name = std::env::var("QUEUE_NAME").expect("QUEUE_NAME not set");
     let policy_name = std::env::var("POLICY_NAME").expect("POLICY_NAME not set");
     let policy_key = std::env::var("POLICY_KEY").expect("POLICY_KEY not set");

     let client = QueueClient::new(
         http_client,
         service_bus_namespace,
         queue_name,
         policy_name,
         policy_key,
     )?;

     client.send_message("JOE MESSAGE FROM QUEUEU", None).await?;

    let msg = client.receive_and_delete_message().await?;

    println!("received message {msg}");

    create_mailer();

    println!("Sending email...");

    send_email(msg);
    
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    get_message_and_send_email().await.unwrap_or_else(|e| {
        eprintln!("Error initializing Azure credentials: {}", e);
    });

}
