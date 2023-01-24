use super::ball::Ball;
use super::bowl::{Bowl, BaseBowl, LoggingBowl};
use super::tube::{Tube, BaseTube};


pub struct BowlBuilder;

impl BowlBuilder {
    pub fn base<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone {
        Bowl::Base(BaseBowl::new(f))
    }
    pub fn logging<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone + std::fmt::Display {
        Bowl::Logging(LoggingBowl::new(f))
    }
}

pub struct TubeBuilder;

impl TubeBuilder {
    pub fn base<T>() -> Tube<T>
    where T: Into<Ball<T>> + Clone + std::fmt::Display {
        Tube::Base(BaseTube::new())
    }
}