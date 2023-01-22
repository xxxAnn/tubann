use crate::tubes::ball::Ball;

// Ideally the Tubes should contain a Rc<RefCell<Bowl>>
pub trait Tube<'a, T: 'a> {
    fn insert<U>(obj: U)
    where U: Into<Ball<'a, T>>;

    fn receiving_side<V>() -> V
    where V: ReceivingSideHandler;
}

pub trait ReceivingSideHandler {}