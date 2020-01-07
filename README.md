# rs-notifier
When your program panic, notify a message to your server (eg: slack)

## Example
```rust
use rs-notifier::notification::{Notification, MessageBuilder, PanicCatcher};
use rs-notifier::slack_notify::SlackNotify;
struct CustomMessageBuilder;

impl MessageBuilder for CustomMessageBuilder {
    fn build(&self, name: &str)->String {
        format!("test, test, test, {}", name)
    }
}

fn main() {
    let slack = SlackNotify::new("YOUR SLACK INCOMING WEBHOOK URL");
    let messagebuilder = CustomMessageBuilder{};
    let notif = Notification::new(slack, messagebuilder);
    let panicCatcher = PanicCatcher::new("123", notif);
    panic!("panic!!");
}
// or when use in thread

fn main() {
    let slack = SlackNotify::new("YOUR SLACK INCOMING WEBHOOK URL");
    let messagebuilder = CustomMessageBuilder{};
    let notif = Notification::new(slack, messagebuilder);
    let panicCatcher = PanicCatcher::new("123", notif);

    let handler = std::thread::spawn(move || panicCatcher.notification.catch("thread name", ||{
        // your thread code
    }))

    handler.join().unrawp();
}
```

## Development
You can customize the notifier by implementing `rs-notifier::notification::Notifier`
### eg
```rust
pub struct SlackNotify {
    pub url: String
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
```
