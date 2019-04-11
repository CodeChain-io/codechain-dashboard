use sendgrid::{SGClient, Mail, Destination};
use sendgrid::errors::SendgridResult;

pub struct Sendgrid {
    client: SGClient,
    to: String,
}

impl Sendgrid {
    pub fn new(api_key: String, to: String) -> Self {
        Self {
            client: SGClient::new(api_key),
            to,
        }
    }

    pub fn send(&self, subject: impl AsRef<str>, text: impl AsRef<str>) -> SendgridResult<()> {
        let mail = Mail::new()
            .add_to(Destination {
                address: self.to.as_str(),
                name: self.to.as_str(),
            })
            .add_from("no-reply@dashboard.codechan.io")
            .add_subject(subject.as_ref())
            .add_text(text.as_ref());
        let result = self.client.send(mail)?;
        cinfo!("Send email to {}: {}", self.to, result);
        Ok(())
    }
}
