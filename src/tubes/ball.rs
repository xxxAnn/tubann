use std::{sync::Mutex, rc::Rc};

#[derive(Clone)]

pub struct Ball<T>
where T: Clone {
    content: Rc<Mutex<T>>,
}

impl<T> std::fmt::Display for Ball<T>
where T: Clone + std::fmt::Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\x1b[94m(INFO)\x1b[0m Passing Ball object containing: {} to Logging Bowl.", &*self.content.lock().unwrap())
    }
}

impl<T> From<T> for Ball<T> 
where T: Sized + Send + Clone + std::fmt::Display {
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