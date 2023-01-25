use std::{sync::Mutex, rc::Rc};

use crate::{tubes::{bowl::{BaseBowl, Bowl}, ball::Ball}, internals::*};

use super::{Tube, TubeConnector};

pub struct BaseTube<T>
where T: Into<Ball<T>> + Clone {
    bowls: Vec<Rc<Mutex<Box<dyn Bowl<T>>>>>,
}

impl<T> Tube<T> for BaseTube<T>
where T: Into<Ball<T>> + Clone + std::fmt::Display {
    fn roll(&self, obj: T) -> TubeResult<()> {
        let ball = obj.into();
        for bowl in self.bowls.iter() {
            match bowl.lock() {
                Ok(mut b) => {
                    b.hit(ball.clone());
                },
                Err(_) => {

                }
            }
        }
        Ok(())
    }
}

impl<T> BaseTube<T>
where T: Into<Ball<T>> + Clone{
    pub fn new() -> Self {
        BaseTube {
            bowls: Vec::new()
        }
    }
    pub fn connect(&mut self) -> BaseTubeConnector<T> {
        BaseTubeConnector {
            r: self
        }
    }
}

pub struct BaseTubeConnector<'a, T>
where T: Into<Ball<T>> + Clone {
    r: &'a mut BaseTube<T>
}

impl<'a, T> TubeConnector<T, BaseTube<T>> for BaseTubeConnector<'a, T> 
where T: Into<Ball<T>> + Clone + std::fmt::Display + 'a {
    fn send_to(&mut self, bowl: Box<dyn Bowl<T>>) -> TubeResult<()> {
        self.r.bowls.push(Rc::new(Mutex::<Box<dyn Bowl<T>>>::new(bowl.into())));
        Ok(())
    }
}