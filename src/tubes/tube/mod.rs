#![allow(unused_imports)]

mod base;

use crate::error::TubeResult;

use self::base::BaseTubeConnector;

use super::{ball::Ball, bowl::{BaseBowl, Bowl}};
pub use base::BaseTube;


pub trait Tube<T>
where T: Into<Ball<T>> + Clone + std::fmt::Display {
    fn roll(&self, obj: T) -> TubeResult<()>;
}

pub trait TubeConnector<T, U>
where T: Into<Ball<T>> + Clone + std::fmt::Display, U: Tube<T> {
    fn send_to(&mut self, bowl: Box<dyn Bowl<T>>) -> TubeResult<()>;
}