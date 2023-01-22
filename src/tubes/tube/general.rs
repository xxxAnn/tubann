use crate::tubes::ball::Ball;

// Ideally the Tubes should contain a Rc<RefCell<Bowl>>
pub trait Tube<T> {
    fn insert<U>(obj: U)
    where U: Into<Ball<T>>;
}