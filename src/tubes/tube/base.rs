use std::{sync::Mutex, rc::Rc};

use crate::tubes::{bowl::BaseBowl, ball::Ball};

pub struct BaseTube<T>
where T: Into<Ball<T>> + Clone {
    bowls: Vec<Rc<Mutex<BaseBowl<T>>>>,
}

impl<T> BaseTube<T>
where T: Into<Ball<T>> + Clone {
    pub fn roll(&self, obj: T) {
        let ball = obj.into();
        for bowl in self.bowls.iter() {
            bowl.lock().expect("Could not unlock bowl").hit(ball.clone());
        }
    }
    pub fn connect(&mut self, bowl: BaseBowl<T>) {
        self.bowls.push(Rc::new(Mutex::new(bowl)));
    }
}