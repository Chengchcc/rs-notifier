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
    use super::notification::Notifier;
    use super::slack_notify::SlackNotify;
  
    
    #[test]
    fn it_works() {
        let slack = SlackNotify::new("");
        let _ = slack.notify("test test");
    }
}



