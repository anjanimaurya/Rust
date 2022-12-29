use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use lettre::message::{Mailboxes, MaybeString};
use lettre::message::header;
use lettre_email::EmailBuilder;
use lettre::message::SinglePart;
use lettre::message::MessageBuilder;

fn main() {
    mail_by_lettre(); //working
}

fn mail_by_lettre(){
    let mail_body = "<h2>This is a test email</h2><p>This is sent by a Rust Program:</p><p>Built on GitHub, Codespaces using VSCode.</p>";
    let email_body = MaybeString::String(mail_body.into());

    let to_address = "Anjani Maurya<maurya.anjani@gmail.com>";

    let mailboxes: Mailboxes = to_address.parse().unwrap();
    let to_header: header::To = mailboxes.into();

    let email = MessageBuilder::new()
        .mailbox(to_header)
        .from("Anjani Maurya<anjani.maurya@ocr-inc.com>".parse().unwrap())
        .subject("Happy new year")
        .singlepart(SinglePart::html(email_body))
        .unwrap();

    let creds = Credentials::new("anjani.maurya@ocr-inc.com".to_string(), "rkmzvfmwookrlznd".to_string());

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => panic!("Could not send email: {:?}", e),
    }
}
