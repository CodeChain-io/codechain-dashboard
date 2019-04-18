mod sendgrid;
mod slack;

use std::sync::Arc;

use chrono::Utc;

use self::sendgrid::Sendgrid;
use self::slack::Slack;

#[derive(Default)]
pub struct NotiBuilder {
    slack: Option<String>,
    sendgrid: Option<(String, String)>,
}

impl NotiBuilder {
    pub fn slack(&mut self, url: String) -> &Self {
        self.slack = Some(url);
        self
    }
    pub fn sendgrid(&mut self, api_key: String, to: String) -> &Self {
        self.sendgrid = Some((api_key, to));
        self
    }

    pub fn build(self) -> Arc<Noti> {
        let slack = self.slack.map(|url| Slack::try_new(url).unwrap());
        let sendgrid = self.sendgrid.map(|(api_key, to)| Sendgrid::new(api_key, to));
        Arc::new(Noti {
            slack,
            sendgrid,
        })
    }
}

pub struct Noti {
    slack: Option<Slack>,
    sendgrid: Option<Sendgrid>,
}

impl Noti {
    pub fn warn(&self, network_id: &str, message: &str) {
        let mut targets = Vec::with_capacity(2);
        if self.slack.is_some() {
            targets.push("slack");
        }
        if self.sendgrid.is_some() {
            targets.push("sendgrid");
        }
        if targets.is_empty() {
            cinfo!("No targets to send warning: {}", message);
            return
        }
        cinfo!("Send a warning to {}: {}", targets.join(", "), message);

        if let Some(slack) = self.slack.as_ref() {
            if let Err(err) = slack.send(format!("{}: {}", network_id, message)) {
                cwarn!("Cannot send a slack message({}): {}", message, err);
            }
        }
        if let Some(sendgrid) = self.sendgrid.as_ref() {
            if let Err(err) = sendgrid.send(
                format!("[warn][{}][dashboard-server] Warning at {}", network_id, Utc::now().to_rfc3339()),
                message,
            ) {
                cwarn!("Cannot send an email({}): {}", message, err);
            }
        }
    }
}
