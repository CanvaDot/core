use std::cell::RefCell;
use std::rc::Rc;

use uuid::Uuid;

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
            .push(Rc::new(RefCell::new(notification)));
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
