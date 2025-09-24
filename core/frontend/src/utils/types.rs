use std::cell::RefCell;
use std::rc::Rc;

pub type InRef<T> = Rc<RefCell<T>>;
