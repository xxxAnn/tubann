use crate::tubes::ball::Ball;

use super::Bowl;

/// The `LoggingBowl` type. Identical to `BaseBowl` but logs the `Ball`s it receives.
pub struct LoggingBowl<T> {
    f: Box<dyn Fn(&T)>
}

impl<T> Bowl<T> for LoggingBowl<T>
where T: Clone + std::fmt::Display {
    fn hit(&mut self, obj: Ball<T>) {
        println!("{}", obj);
        (self.f)(&*obj.open().lock().unwrap());
    }
    fn type_name() -> String {
        "Logging".to_string()
    }
}

impl<T> LoggingBowl<T>
where T: Clone {
    /// Creates a new `LoggingBowl`.
    pub fn new(f: Box<dyn Fn(&T)>) -> Box<Self> {
        Box::new(LoggingBowl {
            f
        })
    }

}