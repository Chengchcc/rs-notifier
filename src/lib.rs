#![allow(dead_code)]
#![allow(unused_imports)]
#[macro_use]
extern crate ureq;

mod notification;
mod slack_notify;


use notification::*;
use slack_notify::SlackNotify;



#[cfg(test)]
mod tests {
    use super::notification::{Notification, MessageBuilder, PanicCatcher};
    use super::slack_notify::SlackNotify;
    struct CustomMessageBuilder;

    impl MessageBuilder for CustomMessageBuilder {
        fn build(&self, name: &str)->String {
            format!("test, test, test, {}", name)
        }
    }
    
    #[test]
    fn it_works() {
        let slack = SlackNotify::new("");
        let messagebuilder = CustomMessageBuilder{};
        let notif = Notification::new(slack, messagebuilder);
        let panicCatcher = PanicCatcher::new("123", notif);
        panic!("panic!!");
    }
}



