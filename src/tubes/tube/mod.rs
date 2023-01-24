#![allow(unused_imports)]

mod base;

use self::base::BaseTubeConnector;

use super::{ball::Ball, bowl::BaseBowl};
pub use base::BaseTube;


pub enum Tube<T>
where T: Into<Ball<T>> + Clone {
    Base(BaseTube<T>)
}

impl<T> Tube<T>
where T: Into<Ball<T>> + Clone + std::fmt::Display {
    pub fn roll(&self, obj: T) {
        match self {
            Tube::Base(tube) => {
                tube.roll(obj);
            },
        }
    }
    pub fn connect(&mut self) -> BaseTubeConnector<T> {
        return match self {
            Tube::Base(tube) => {
                tube.connect()
            },
        }
    }
}