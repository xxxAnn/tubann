mod base;
mod logger;

pub use base::BaseBowl;
pub use logger::LoggingBowl;


pub enum Bowl<T> {
    Base(BaseBowl<T>),
    Logging(LoggingBowl<T>)
}

impl<T> Bowl<T> {
    pub fn get_bowl_type(&self) -> String {
        match self {
            Self::Base(_) => "Base".to_owned(),
            Self::Logging(_) => "Logging".to_owned()
        } 
    }
}