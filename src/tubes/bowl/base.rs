//!

use crate::tubes::ball::Ball;

use super::Bowl;

/// The `BaseBowl` type. Simplest implementation of a `Bowl`. 
/// It simply calls its inner function upon receiving a `Ball`.
pub struct BaseBowl<T> {
    f: Box<dyn Fn(&T)>
}

impl<T> Bowl<T> for BaseBowl<T>
where T: Clone {
    fn hit(&mut self, obj: Ball<T>) {
        (self.f)(&*obj.open().lock().unwrap());
    }
    fn type_name() -> String {
        "Base".to_string()
    }
}

impl<T> BaseBowl<T>
where T: Clone {
    /// Creates a new `BaseBowl`.
    pub fn new(f: Box<dyn Fn(&T)>) -> Box<Self> {
        Box::new(BaseBowl {
            f
        })
    }
}