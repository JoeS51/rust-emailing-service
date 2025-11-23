use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    SmtpTransport, Transport,
    message::{Message, Mailbox}
};
use dotenvy::dotenv;

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

fn send_email() {
    let email = Message::builder()
        .from("joesluis51@gmail.com".parse::<Mailbox>().unwrap())
        .to("joesluis51@gmail.com".parse::<Mailbox>().unwrap())
        .subject("email with rust")
        .body("sending this email using rust congrats broski".to_string())
        .unwrap();

    let mailer = create_mailer();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent!"),
        Err(error) => {
            println!("Email didn't send. {:?}", error);
        }
    }
}

fn main() {
    dotenv().ok();

    println!("Sending email...");

    send_email();
}
