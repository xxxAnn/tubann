use crate::tubes::ball::Ball;

pub trait Bowl<'a, T: 'a> {
    fn hit(obj: Ball<T>);
}