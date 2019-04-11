use slack_hook::{PayloadBuilder, Result, Slack as Hook, SlackText};

pub struct Slack(Hook);

impl Slack {
    pub fn try_new(url: impl AsRef<str>) -> Result<Self> {
        Ok(Self(Hook::new(url.as_ref())?))
    }

    pub fn send(&self, message: impl Into<SlackText>) -> Result<()> {
        let p = PayloadBuilder::new().text(message).build()?;

        self.0.send(&p)
    }
}
