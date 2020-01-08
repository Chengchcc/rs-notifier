
pub trait Notifier {
    type body;
    /// notify
    fn notify(&self, msg:&str) -> Result<(), String>;
    /// notify body build
    fn bodybuild(&self, message: &str)-> Self::body;
} 








