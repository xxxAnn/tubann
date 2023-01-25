mod base;
mod logger;

use super::ball::Ball;

pub use logger::LoggingBowl;
pub use base::BaseBowl;

/// The Bowl trait. Functions with super powers!
pub trait Bowl<T>
where T: Clone {
    fn hit(&mut self, obj: Ball<T>);
    fn type_name() -> String
    where Self: Sized; 
}