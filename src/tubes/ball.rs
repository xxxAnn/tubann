use std::{sync::Mutex, rc::Rc};

#[derive(Clone)]

pub struct Ball<T>
where T: Clone {
    content: Rc<Mutex<T>>,
}

impl<T> From<T> for Ball<T> 
where T: Sized + Send + Clone {
    fn from(value: T) -> Self {
        Ball {
            content: Rc::new(Mutex::new(value)),
        }
    }
}

impl<T> Ball<T>
where T: Clone {
    pub fn open(&self) -> Rc<Mutex<T>> {
        return self.content.clone()
    }
}