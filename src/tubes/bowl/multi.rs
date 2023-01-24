use std::{rc::Rc, sync::Mutex};

use crate::tubes::{tube::Tube, ball::Ball};

pub struct MultiBowl<T>
where T: Into<Ball<T>> + Clone {
    f: Box<dyn Fn(&T)>,
    outs: Vec<Rc<Mutex<Tube<T>>>>,
}

impl<T> MultiBowl<T>
where T: Clone + Into<Ball<T>> + std::fmt::Display {
    pub fn hit(&mut self, obj: Ball<T>) {
        (self.f)(&*obj.open().lock().unwrap());
        for out in &self.outs {
            match &*out.lock().unwrap() {
                Tube::Base(t) => t.roll((&*obj.open().lock().unwrap()).clone()),
            }
        }
    }
}

impl<T> MultiBowl<T>
where T: Clone + Into<Ball<T>> {
    pub fn new(f: Box<dyn Fn(&T)>) -> Self {
        MultiBowl {
            f,
            outs: Vec::new()
        }
    }
    pub fn type_name() -> String {
        "Multi".to_string()
    }
}