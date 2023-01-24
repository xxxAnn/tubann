use std::{sync::Mutex, rc::Rc};

use crate::{tubes::{bowl::{BaseBowl, Bowl}, ball::Ball}, internals::*};

pub struct BaseTube<T>
where T: Into<Ball<T>> + Clone {
    bowls: Vec<Rc<Mutex<Bowl<T>>>>,
}

impl<T> BaseTube<T>
where T: Into<Ball<T>> + Clone + std::fmt::Display {
    pub fn roll(&self, obj: T) {
        let ball = obj.into();
        for bowl in self.bowls.iter() {
            match &mut *bowl.lock().unwrap() {
                Bowl::Base(b) => {
                    b.hit(ball.clone())
                }
                Bowl::Logging(b) => {
                    b.hit(ball.clone())
                }
                Bowl::Multi(b) => {
                    b.hit(ball.clone())
                }
             }
        }
    }
    pub fn connect(&mut self) -> BaseTubeConnector<T> {
        BaseTubeConnector {
            r: self
        }
    }

    pub fn new() -> Self {
        BaseTube {
            bowls: Vec::new()
        }
    }
}

pub struct BaseTubeConnector<'a, T>
where T: Into<Ball<T>> + Clone {
    r: &'a mut BaseTube<T>
}

impl<'a, T> BaseTubeConnector<'a, T> 
where T: Into<Ball<T>> + Clone {
    pub fn base(&mut self, bowl: Bowl<T>) -> TubeResult<()> {
        if let Bowl::Base(basic_bowl) = bowl {
            self.r.bowls.push(Rc::new(Mutex::new(Bowl::Base(basic_bowl))));
            Ok(())
        } else {
            Err(TubeError::InvalidBowlType(InvalidBowlTypeError::new("Base".to_owned(), bowl.get_bowl_type())))
        }
    }
    pub fn logging(&mut self, bowl: Bowl<T>) -> TubeResult<()>{
        if let Bowl::Logging(logging_bowl) = bowl {
            self.r.bowls.push(Rc::new(Mutex::new(Bowl::Logging(logging_bowl))));
            Ok(())
        } else {
            Err(TubeError::InvalidBowlType(InvalidBowlTypeError::new("Logging".to_owned(), bowl.get_bowl_type())))
        }
    }
}