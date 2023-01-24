use crate::tubes::ball::Ball;

pub struct LoggingBowl<T> {
    f: Box<dyn Fn(&T)>
}

impl<T> LoggingBowl<T>
where T: Clone + std::fmt::Display {
    pub fn hit(&mut self, obj: Ball<T>) {
        println!("{}", obj);
        (self.f)(&*obj.open().lock().unwrap());
    }
}

impl<T> LoggingBowl<T>
where T: Clone {
    pub fn new(f: Box<dyn Fn(&T)>) -> Self {
        LoggingBowl {
            f
        }
    }
    pub fn type_name() -> String {
        "Base".to_string()
    }
}