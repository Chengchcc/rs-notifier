

pub struct Notification< N: Notifier, M: MessageBuilder>{
    /// notifier
    pub notifier: N,
    
    pub messagebuilder: M
}


impl<N: Notifier, M: MessageBuilder> Notification<N, M>{
    pub fn catch<F: Fn() -> R + std::panic::UnwindSafe, R>(&self, name:&str, f: F){
        let result = std::panic::catch_unwind(f);
        if result.is_err() {
            println!("thread:{} panic !!", name );
            self.notifier.notify(&self.messagebuilder.build(name));
        }
    }

    pub fn new(notifier: N, messagebuilder: M)-> Notification<N,M> {
        Notification{
            notifier,
            messagebuilder
        }
    }
}




pub struct PanicCatcher< N: Notifier, M: MessageBuilder>{
    pub thread_name: String,
    pub notification: Notification<N, M>
}

impl<N: Notifier, M: MessageBuilder> PanicCatcher<N,M> {
    pub fn new(name: &str, notification: Notification<N,M>)-> PanicCatcher<N,M> {
        PanicCatcher {
            thread_name: name.to_owned(),
            notification
        }
    }
}


impl<N: Notifier, M: MessageBuilder> Drop for PanicCatcher<N,M> {
    fn drop (&mut self){
        if std::thread::panicking() {
            println!("thread:{} panic !!", self.thread_name);
        }
        self.notification.notifier.notify(&self.notification.messagebuilder.build(&self.thread_name));
    }
}



pub trait Notifier {
    fn notify(&self, message: &str);
} 


pub trait MessageBuilder {
    fn build(&self, name: &str) -> String;
}





