use super::notification::Notifier;

#[derive(Clone)]
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
    type body = String;

    fn notify(&self, msg: &str) -> Result<(), String> {
        if &self.url == "" {
            return Err("not set url".to_owned());
        }
        let resp = ureq::post(&self.url)
                .set("Content-Type", "application/json")
                .set("Accept", "application/json")
                .send_json(json!(&self.bodybuild(msg)));
        if resp.error() {
            return Err("send error".to_owned());
        }
        Ok(())
    }

    fn bodybuild(&self, message: &str) -> Self::body {
        json!({
            "text": "server crash",
            "attachments":[
                {
                    "mrkdwn_in": ["text"],
                    "color": "#DD2248",
                    "text": message,
                }
            ]}).as_str().unwrap().to_owned()
    
    }
}

