use slack_hook::{SlackText, Slack as Hook, PayloadBuilder, Result};

pub struct Slack(Hook);

impl Slack {
    pub fn try_new(url: impl AsRef<str>) -> Result<Self> {
        Ok(Self(Hook::new(url.as_ref())?))
    }

    pub fn send(&self, message: impl Into<SlackText>) -> Result<()> {
        let p = PayloadBuilder::new()
            .text(message)
            .build()?;

        self.0.send(&p)
    }
}