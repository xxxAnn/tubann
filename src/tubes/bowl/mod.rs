mod base;

pub use base::BaseBowl;


pub enum Bowl<T> {
    Base(BaseBowl<T>)
}