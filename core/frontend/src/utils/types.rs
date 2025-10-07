use std::cell::RefCell;
use std::fmt::{Debug, Result as FmtResult, Formatter};
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};

pub type InRef<T> = Rc<RefCell<T>>;
pub struct AtomicInRef<T>(Arc<Mutex<T>>);

// Used for functions whose only error is going to be locking a mutex.
pub type LMResult<'e, T, E> = Result<T, PoisonError<MutexGuard<'e, E>>>;

// Used for callbacks not dependent on the yew runtime.
pub struct AtomicCallback<T>(Arc<dyn Fn(T) + Send + Sync + 'static>);

impl<T> Deref for AtomicInRef<T> {
    type Target = Arc<Mutex<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: PartialEq> PartialEq for AtomicInRef<T> {
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(&self.0, &other.0) {
            return true;
        }

        let Ok(left) = self.0.lock() else { return false; };
        let Ok(right) = other.0.lock() else { return false; };

        *left == *right
    }
}

impl<T, F: Fn(T) + Send + Sync + 'static> From<F> for AtomicCallback<T> {
    fn from(value: F) -> Self {
        Self(Arc::new(value))
    }
}

impl<T> PartialEq for AtomicCallback<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}

impl<T> Deref for AtomicCallback<T> {
    type Target = Arc<dyn Fn(T) + Send + Sync + 'static>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for AtomicCallback<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Debug for AtomicCallback<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "<AtomicCallback: {:?}>", Arc::into_raw(self.into()))
    }
}

impl<T> AtomicCallback<T> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(T) + Send + Sync + 'static,
    {
        Self(Arc::new(f))
    }

    pub fn emit(&self, value: T) {
        (self.0)(value)
    }
}

unsafe impl<T: Send> Send for AtomicCallback<T> {}
unsafe impl<T: Sync> Sync for AtomicCallback<T> {}
