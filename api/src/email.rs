use std::env;
use aws_sdk_sesv2::client::Client;
use aws_sdk_sesv2::types::{Destination, Content, Body, Message, EmailContent};
use tera::Tera;
use anyhow::Result;

pub struct EmailProvider{
    templates: Tera,
    is_production: bool,
    provider: Option<Client>
}

pub struct EmailAddress(String);

impl EmailAddress{
    pub fn new(email: String) -> Result<EmailAddress>{
        Ok(EmailAddress(email))
    }
    pub fn to_string(&self) -> String{
        self.0.clone()
    }
    pub fn domain(&self) -> String{
        let parts: Vec<&str> = self.0.split("@").collect();
        parts[1].to_string()
    }
}

fn send_dummy(to: &EmailAddress, subject: &str, message: &str, _message_html: &str) -> () {
    println!("to: {}, from: noreply@mail.groovelet.com, subject: {}, message: {}", to.to_string(), subject, message);
}

async fn _send_real(email_client: &Client, to: &EmailAddress, subject: &str, message: &str, message_html: &str) -> Result<()> {
    println!("to: {}, from: noreply@mail.groovelet.com, subject: {}, message: {}", to.to_string(), subject, message);

    let mut dest: Destination = Destination::builder().build();
    dest.to_addresses = Some(vec![to.to_string()]);

    let subject_content = Content::builder()
        .data(subject)
        .charset("UTF-8")
        .build()
        .expect("building Content");

    let body_content = Content::builder()
        .data(message)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body_html = Content::builder()
        .data(message_html)
        .charset("UTF-8")
        .build()
        .expect("building Content");
    let body = Body::builder().text(body_content).html(body_html).build();

    let msg = Message::builder()
        .subject(subject_content)
        .body(body)
        .build();

    let email_content = EmailContent::builder().simple(msg).build();

    email_client
        .send_email()
        .from_email_address("noreply@mail.groovelet.com")
        .destination(dest)
        .content(email_content)
        .send()
        .await?;

    Ok(())
}

async fn send_real(email_client: &Client, to: &EmailAddress, subject: &str, message: &str, message_html: &str, is_production:bool) -> Result<()> {
    // we always send a second copy to ourselves for debugging
    if is_production {
        _send_real(email_client, &to, &subject, &message, &message_html).await?;
        _send_real(email_client, &EmailAddress::new("safe@gooble.email".to_string()).expect("gooble is always a valid email address"), &subject, &message, &message_html).await?;
    }
    else{
        _send_real(email_client, &EmailAddress::new("email@gooble.email".to_string()).expect("gooble is always a valid email address"), &subject, &message, &message_html).await?;
    }
    Ok(())
}

impl EmailProvider{
    pub async fn setup(is_production: bool) -> EmailProvider{
        let tera = Tera::new("templates_email/*").unwrap();
        let aws_config_exists: bool = env::var("AWS_ACCESS_KEY_ID").is_ok();
        if !aws_config_exists{
            EmailProvider{
                templates: tera,
                provider: None,
                is_production: is_production,
            }
        }
        else{
            let config = aws_config::load_from_env().await;
            let client = aws_sdk_sesv2::Client::new(&config);
            EmailProvider{
                templates: tera,
                provider: Some(client),
                is_production: is_production,
            }
        }
    }

    pub async fn send(&self, to: &EmailAddress, subject: &str, message: &str, message_html: &str) -> Result<()> {
        match &self.provider{
            None => send_dummy(&to, &subject, &message, &message_html),
            Some(client) => send_real(&client, &to, &subject, &message, &message_html, self.is_production).await?,
        }

        Ok(())
    }

    pub async fn send_hello(&self, to: &EmailAddress) -> Result<()> {
        let templates = &self.templates;
        let mut context = tera::Context::new();
        context.insert("name", "Curtis");
        let message_html = templates.render("email_helloworld.html.tera", &context)?;
        self.send(to, "Hello!", "Hello!", &message_html).await?;

        Ok(())
    }

    pub async fn send_verification_email(&self, to: &EmailAddress, verification_link: &str) -> Result<()> {
        let templates = &self.templates;
        let mut context = tera::Context::new();
        context.insert("verification_link", verification_link);
        let message_html = templates.render("email_verification.html.tera", &context)?;
        let message_text: String = format!("Please follow the email verification link: {}", verification_link);
        self.send(to, "Verify your email", &message_text, &message_html).await?;

        Ok(())
    }

}