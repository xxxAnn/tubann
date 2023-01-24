use crate::tubes::ball::Ball;

pub struct BaseBowl<T> {
    f: Box<dyn Fn(&T)>
}

impl<T> BaseBowl<T>
where T: Clone {
    pub fn hit(&mut self, obj: Ball<T>) {
        (self.f)(&*obj.open().lock().unwrap());
    }
}

impl<T> BaseBowl<T>
where T: Clone {
    pub fn new(f: Box<dyn Fn(&T)>) -> Self {
        BaseBowl {
            f
        }
    }
    pub fn type_name() -> String {
        "Base".to_string()
    }
}