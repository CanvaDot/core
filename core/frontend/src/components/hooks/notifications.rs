use std::error::Error;
use std::rc::Rc;

use log::error;
use yew::prelude::*;

use crate::app::SharedAppContext;
use crate::utils::notification_store::{
    Notification,
    NotificationComponent,
    NotificationComponentType,
    NotificationLevel,
};

pub struct NotificationHandle {
    state_handle: Option<SharedAppContext>,
}

impl NotificationHandle {
    pub fn notify(&self, notification: Notification) {
        let Some(state_handle) = &self.state_handle else {
            // XXX: Report this.
            error!("Couldn't notify, SharedAppContext was not found.");
            return;
        };

        state_handle
            .notifications
            .add(notification);
    }
}

pub trait ResultReport<T, E: Error> {
    fn or_notify(self, handle: &NotificationHandle) -> T;
}

impl<T, E: Error> ResultReport<T, E> for Result<T, E> {
    fn or_notify(self, handle: &NotificationHandle) -> T {
        match self {
            Ok(value) => value,

            Err(error) => {
                handle.notify(
                    Notification::new("Error", format!("{error}"))
                        .set_level(NotificationLevel::Error)
                        .set_components(vec![NotificationComponent::RedirectButton {
                            text: "Notify".into(),
                            redirect: "/".into(),
                            enabled: false,
                            kind: NotificationComponentType::Primary,
                        }])
                        .clone(),
                );

                panic!(
                    concat!(
                        "An application error occurred, ",
                        "the error has been notified to the user ",
                        "\n{:#}"
                    ),
                    error
                );
            },
        }
    }
}

#[hook]
pub fn use_notifications() -> Rc<NotificationHandle> {
    let context = use_context::<SharedAppContext>();

    Rc::new(NotificationHandle { state_handle: context })
}
