use std::rc::Rc;
use std::sync::Mutex;

use crate::tubes::ball::Ball;
use crate::tubes::bowl::Bowl;
// Ideally the Tubes should contain a Rc<RefCell<Bowl>>
pub trait Tube<'a, T: 'a> {
    type ReceivingSideHandlerType;
    
    fn insert<U>(&self, obj: U)
    where U: Into<Ball<'a, T>>;

    fn receiving_side(&self) -> Self::ReceivingSideHandlerType;
}

pub trait ReceivingSideHandler {}

/// The base tube is simply an interface for methods that interact with a bowl. 
/// It only contains a Vec of references to Bowls.
pub struct BaseTube<'a, T> {
    t: Vec<Rc<Mutex<dyn Bowl<'a, T>>>>,
}

pub struct BaseTubeReceivingSideHandler<'a, T> {
    tube: &'a BaseTube<'a, T>,
}

impl<'a> BaseTubeReceivingSideHandler<'a, u32> {
    fn test(&mut self) {
        let tube = self.tube;

        let a = &tube.t[0];

        let mut data = a.lock().unwrap();

        data.hit()

    }
}

impl<'a, T: 'a> ReceivingSideHandler for BaseTubeReceivingSideHandler<'a, T> {}

impl<'a, T: 'a> Tube<'a, T> for BaseTube<'a, T>
where T: Sized + Send {
    type ReceivingSideHandlerType = BaseTubeReceivingSideHandler<'a, T>;

    fn insert<U>(&self, obj: U)
    where U: Into<Ball<'a, T>> {
        todo!()
    }
    fn receiving_side(&self) -> BaseTubeReceivingSideHandler<'a, T> {
        todo!()
    }

    
}