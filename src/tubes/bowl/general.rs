use crate::tubes::ball::Ball;

pub trait Bowl<'a, T: 'a> {
    fn hit(&mut self, );//obj: Ball<T>);
}