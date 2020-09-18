# rs-notifier
rust social software notification  (eg: slack)

## Example
```rust

use rs-notifier::notification::Notifier;
use rs-notifier::slack_notify::SlackNotify;

fn main() {
    let slack = SlackNotify::new("YOUR SLACK INCOMING WEBHOOK URL");
    let _ = slack.notify("test test");
}
```

## Development
You can customize the notifier by implementing `rs-notifier::notification::Notifier`
### eg
```rust
pub struct customNotify {
    pub url: String
}


impl customNotify {
    pub fn new(url: &str)-> customNotify{
        customNotify{
            url: url.to_owned()
        }
    }
}

impl Notifier for customNotify {
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
```
