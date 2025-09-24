use std::cell::RefCell;
use std::error::Error;
use std::rc::Rc;
use std::slice::Iter;

use instant::{Duration, Instant};
use palette::Srgb;
use uuid::Uuid;
use yew::{Callback, Properties};

use crate::utils::colors::{ERROR_RED, INFO_BLUE, SUCCESS_GREEN};
use crate::utils::notifications::notification::Notification;
use crate::utils::types::InRef;

#[derive(Default, PartialEq)]
pub struct NotificationStore {
    notifications: InRef<Vec<InRef<Notification>>>,
}

impl NotificationStore {
    pub fn add(&self, notification: Notification) {
        self.notifications
            .borrow_mut()
            .push(Rc::new(RefCell::new(notification)))
    }

    pub fn remove_expired(&self) {
        self.notifications
            .borrow_mut()
            .retain(|notification| {
                !notification
                    .borrow()
                    .is_expired()
            });
    }

    pub fn all(&self) -> InRef<Vec<InRef<Notification>>> {
        self.notifications
            .clone()
    }

    pub fn remove_by_id(&self, id: Uuid) {
        self.notifications
            .borrow_mut()
            .retain(|notification| {
                notification
                    .borrow()
                    .id()
                    != id
            });
    }
}
