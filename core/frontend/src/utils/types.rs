use std::cell::RefCell;
use std::rc::Rc;

use num_traits::Num;

pub type InRef<T> = Rc<RefCell<T>>;

pub trait TupleCords<T: Num> {
    fn x(&self) -> T;

    fn y(&self) -> T;
}

impl<T: Num + Copy> TupleCords<T> for (T, T) {
    fn x(&self) -> T {
        self.0
    }

    fn y(&self) -> T {
        self.1
    }
}
