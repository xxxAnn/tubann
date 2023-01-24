mod base;
mod logger;
mod multi;

pub use base::BaseBowl;
pub use logger::LoggingBowl;
pub use multi::MultiBowl;

use super::ball::Ball;

pub enum Bowl<T>
where T: Clone + Into<Ball<T>> {
    Base(BaseBowl<T>),
    Logging(LoggingBowl<T>),
    Multi(MultiBowl<T>)
}

impl<T> Bowl<T> 
where T: Clone + Into<Ball<T>> {
    pub fn get_bowl_type(&self) -> String {
        match self {
            Self::Base(_) => BaseBowl::<T>::type_name(),
            Self::Logging(_) => LoggingBowl::<T>::type_name(),
            Self::Multi(_) => MultiBowl::<T>::type_name()
        } 
    }
}