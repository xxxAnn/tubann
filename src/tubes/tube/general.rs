use crate::tubes::ball::Ball;

// Ideally the Tubes should contain a Rc<RefCell<Bowl>>
pub trait Tube<'a, T: 'a> {
    type ReceivingSideHandlerType;
    
    fn insert<U>(obj: U)
    where U: Into<Ball<'a, T>>;

    fn receiving_side() -> Self::ReceivingSideHandlerType;
}

pub trait ReceivingSideHandler {}

pub struct BaseTube {
    
}

pub struct BaseTubeReceivingSideHandler<'a> {
    tube: &'a BaseTube,
}

impl ReceivingSideHandler for BaseTubeReceivingSideHandler<'_> {}

impl<'a, T: 'a> Tube<'a, T> for BaseTube {
    type ReceivingSideHandlerType = BaseTubeReceivingSideHandler<'a>;

    fn insert<U>(obj: U)
    where U: Into<Ball<'a, T>> {
        todo!()
    }
    fn receiving_side() -> BaseTubeReceivingSideHandler<'a> {
        todo!()
    }

    
}