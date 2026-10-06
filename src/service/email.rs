use std::{env, time::Duration};

use lettre::{
    message::header::{ContentType},
    message::{Mailbox, MultiPart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport,
    AsyncTransport,
    Message,
    Tokio1Executor,
};

use crate::error::error::AppError;

#[derive(Clone)]
pub struct Mailer{
    pub transport: AsyncSmtpTransport<Tokio1Executor>,
    pub from: Mailbox,
}
impl Mailer {
    pub fn from_env() -> Result<Self, AppError> {
        let host = env::var("SMTP_HOST")?;
        let username = env::var("SMTP_USERNAME")?;
        let password = env::var("SMTP_PASSWORD")?;
        let from = env::var("EMAIL_FROM")?.parse()?;

        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(&host)?
            .credentials(Credentials::new(username, password))
            .timeout(Some(Duration::from_secs(10)))
            .build();

        Ok(Self { transport, from })
    }

    pub async fn send_otp_to_email(&self, recipient: &str, otp: &str) -> Result<(), AppError> {
        let plain = format!(
            "Your verification code is: {otp}\n\nIt expires shortly. If you didn't request this, ignore this email."
        );

        let html = format!(
            r#"<div style="max-width:480px;margin:0 auto;font-family:Arial,Helvetica,sans-serif;border:1px solid #F0F0F0;">
        <div style="background:#C1121F;padding:20px 24px;">
            <h2 style="margin:0;font-family:Georgia,'Times New Roman',serif;font-size:22px;color:#FFFFFF;">The Frame</h2>
        </div>
        <div style="background:#FFFFFF;padding:28px 24px;color:#0D0D0D;">
            <p style="margin:0 0 16px;font-size:15px;">Your verification code is:</p>
            <p style="margin:0 0 24px;font-size:30px;font-weight:bold;letter-spacing:6px;">{otp}</p>
            <p style="margin:0;font-size:13px;color:#6B6B6B;">This code expires shortly. If you didn't request it, you can ignore this email.</p>
        </div>
        </div>"#
        );

        let email = Message::builder()
            .from(self.from.clone())
            .to(recipient.parse::<Mailbox>().map_err(|_| AppError::BadRequest)?)
            .subject("Your The Frame verification code")
            .multipart(MultiPart::alternative_plain_html(plain, html))?;

        self.transport.send(email).await?;
        Ok(())
    }
}