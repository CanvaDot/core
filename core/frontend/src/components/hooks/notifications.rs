use std::fmt::Display;
use std::rc::Rc;

use log::error;
use yew::prelude::*;

use crate::app::SharedAppContext;
use crate::utils::notifications::component::{ActionButton, NotificationComponent};
use crate::utils::notifications::notification::{Notification, NotificationLevel};
use crate::utils::types::InRef;

#[allow(unused)]
pub trait ResultReport<T, E: Display> {
    fn or_notify(self, handle: &NotificationHandle) -> T;
}

#[allow(unused)]
pub trait OptionReport<T, E: Display> {
    fn ok_or_notify(self, error: E, handle: &NotificationHandle) -> T;
}

pub struct NotificationHandle {
    state_handle: Option<SharedAppContext>,
}

impl NotificationHandle {
    pub fn notify(&self, notification: Notification) {
        let Some(ref state_handle) = self.state_handle else {
            // XXX: Report this.
            error!("Couldn't notify, SharedAppContext was not found.");
            return;
        };

        state_handle
            .notifications
            .borrow_mut()
            .add(notification);
    }
}

impl<T, E: Display> ResultReport<T, E> for Result<T, E> {
    fn or_notify(self, handle: &NotificationHandle) -> T {
        match self {
            Ok(value) => value,

            Err(error) => {
                handle.notify(error_notification(&error));

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

impl<T, E: Display> OptionReport<T, E> for Option<T> {
    fn ok_or_notify(self, error: E, handle: &NotificationHandle) -> T {
        match self {
            Some(value) => value,

            None => {
                handle.notify(error_notification(&error));

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

pub fn error_notification<E>(_error: &E) -> Notification {
    Notification::builder()
        .title("Unexpected Error")
        .message("The application was unable to process your request.")
        .level(NotificationLevel::Error)
        .add_action_button(
            ActionButton::builder()
                .text("Report")
                .id("report_button")
                .action(|notification: InRef<Notification>| {
                    let mut notif_mut = notification.borrow_mut();
                    let button = notif_mut.get_component_mut("report_button");

                    if let Some(NotificationComponent::ActionButton(button)) = button {
                        // TODO: implement reporting here.

                        button.set_enabled(false);
                    }
                })
                .build(),
        )
        .build()
}
