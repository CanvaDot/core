use std::error::Error;

use yew::prelude::*;
use log::error;

use crate::app::SharedAppContext;
use crate::utils::notification_store::{Notification, NotificationComponent, NotificationLevel, NotificationComponentType};

pub struct NotificationHandle {
    state_handle: Option<SharedAppContext>
}

impl NotificationHandle {
    pub fn notify(&self, notification: Notification) {
        let Some(state_handle) = &self.state_handle
        else {
            // XXX: Report this.
            error!("Couldn't notify, SharedAppContext was not found.");
            return;
        };

        state_handle.notifications.add(notification);
    }
}

trait ResultReport<T, E: Error> {
    fn or_notify(self, handle: &NotificationHandle) -> Self;
}

impl<T, E: Error> ResultReport<T, E> for Result<T, E> {
    fn or_notify(self, handle: &NotificationHandle) -> Self {
        if let Err(error) = &self {
            handle.notify(
                Notification::new("Error", format!("{error}"))
                    .set_level(NotificationLevel::Error)
                    .set_components(vec![
                        NotificationComponent::RedirectButton {
                            text: "Notify".into(),
                            redirect: "/".into(),
                            enabled: false,
                            kind: NotificationComponentType::Primary
                        }
                    ])
                    .clone()
            );
        }

        self
    }
}

#[hook]
pub fn use_notifications() -> NotificationHandle {
    let context = use_context::<SharedAppContext>();

    NotificationHandle { state_handle: context }
}
