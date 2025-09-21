use std::{cell::RefCell, rc::Rc};


pub type InRef<T> = Rc<RefCell<T>>;
