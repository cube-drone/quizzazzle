use std::env;
use aws_sdk_sesv2::client::Client;
use aws_sdk_sesv2::types::{Destination, Content, Body, Message, EmailContent};
use tera::Tera;
use anyhow::Result;

pub struct EmailProvider{
    templates: Tera,
    provider: Option<Client>
}

fn send_dummy(to: String, subject: String, message: String, _message_html: String) -> () {
    println!("to: {}, from: noreply@mail.groovelet.com, subject: {}, message: {}", to, subject, message);
}

async fn _send_real(email_client: &Client, to: String, subject: String, message: String, message_html: String) -> Result<()> {
    println!("to: {}, from: noreply@mail.groovelet.com, subject: {}, message: {}", to, subject, message);

    let mut dest: Destination = Destination::builder().build();
    dest.to_addresses = Some(vec![to]);

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

async fn send_real(email_client: &Client, to: String, subject: String, message: String, message_html: String) -> Result<()> {
    // we always send a second copy to ourselves for debugging
    _send_real(email_client, to, subject.clone(), message.clone(), message_html.clone()).await?;
    _send_real(email_client, "safe@gooble.email".to_string(), subject, message, message_html).await?;
    Ok(())
}

impl EmailProvider{
    pub async fn setup() -> EmailProvider{
        let tera = Tera::new("templates_email/*").unwrap();
        let aws_config_exists: bool = env::var("AWS_ACCESS_KEY_ID").is_ok();
        if !aws_config_exists{
            EmailProvider{
                templates: tera,
                provider: None,
            }
        }
        else{
            let config = aws_config::load_from_env().await;
            let client = aws_sdk_sesv2::Client::new(&config);
            EmailProvider{
                templates: tera,
                provider: Some(client),
            }
        }
    }

    pub async fn send(&self, to: String, subject: String, message: String, message_html: String) -> Result<()> {
        match &self.provider{
            None => send_dummy(to,  subject, message, message_html),
            Some(client) => send_real(&client, to, subject, message, message_html).await?,
        }

        Ok(())
    }

    pub async fn send_hello(&self, to: String) -> Result<()> {
        let templates = &self.templates;
        let mut context = tera::Context::new();
        context.insert("name", "Curtis");
        let message_html = templates.render("email_helloworld.html.tera", &context)?;
        self.send(to, "Hello!".to_string(), "Hello!".to_string(), message_html).await?;

        Ok(())
    }

}