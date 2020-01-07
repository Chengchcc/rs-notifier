use super::notification::Notifier;

pub struct SlackNotify {
    pub url: String
}
impl SlackNotify {
    pub fn new(url: &str)-> SlackNotify{
        SlackNotify{
            url: url.to_owned()
        }
    }
}

impl Notifier for SlackNotify {
    fn notify(&self, message: &str) {
        if &self.url == "" {
            return;
        }
        let _ = ureq::post(&self.url)
                .set("Content-Type", "application/json")
                .set("Accept", "application/json")
                .send_json(json!({
                    "text": "server crash",
                    "attachments":[
                        {
                            "mrkdwn_in": ["text"],
                            "color": "#DD2248",
                            "text": message,
                        }
                    ]
        }));
    }
}

