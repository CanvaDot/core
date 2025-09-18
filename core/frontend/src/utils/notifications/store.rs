use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::slice::Iter;

use instant::{Duration, Instant};
use palette::Srgb;
use yew::{Callback, Properties};

use crate::utils::colors::{ERROR_RED, INFO_BLUE, SUCCESS_GREEN};
use crate::utils::notifications::notification::Notification;

#[derive(Default, PartialEq)]
pub struct NotificationStore<'c> {
    notifications: Rc<RefCell<Vec<Notification<'c>>>>,
}

impl<'c> NotificationStore<'c> {
    pub fn add(&self, notification: Notification<'c>) {
        self.notifications
            .borrow_mut()
            .push(notification)
    }

    pub fn remove_expired(&self) {
        self.notifications
            .borrow_mut()
            .retain(|notification| !notification.is_expired());
    }

    pub fn all(&self) -> Rc<RefCell<Vec<Notification<'c>>>> {
        self.notifications
            .clone()
    }
}
