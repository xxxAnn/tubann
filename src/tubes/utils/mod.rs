use super::ball::Ball;
use super::bowl::{Bowl, BaseBowl, LoggingBowl, MultiBowl};
use super::tube::{Tube, BaseTube};


pub struct BowlBuilder;

impl BowlBuilder {
    pub fn base<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone + Into<Ball<T>> {
        Bowl::Base(BaseBowl::new(f))
    }
    pub fn logging<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone + std::fmt::Display + Into<Ball<T>> {
        Bowl::Logging(LoggingBowl::new(f))
    }
    pub fn multi<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone + std::fmt::Display + Into<Ball<T>> {
        Bowl::Multi(MultiBowl::new(f))
    }
}

pub struct TubeBuilder;

impl TubeBuilder {
    pub fn base<T>() -> Tube<T>
    where T: Into<Ball<T>> + Clone + std::fmt::Display {
        Tube::Base(BaseTube::new())
    }
}