use super::ball::Ball;
use super::bowl::{Bowl, BaseBowl};
use super::tube::{Tube, BaseTube};


pub struct BowlBuilder;

impl BowlBuilder {
    pub fn base<T>(f: Box<dyn Fn(&T)>) -> Bowl<T>
    where T: Clone {
        return Bowl::Base(BaseBowl::new(f));
    }
}

pub struct TubeBuilder;

impl TubeBuilder {
    pub fn base<T>() -> Tube<T>
    where T: Into<Ball<T>> + Clone {
        return Tube::Base(BaseTube::new())
    }
}