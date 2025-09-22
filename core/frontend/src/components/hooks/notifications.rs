use std::fmt::Display;
use std::rc::Rc;

use log::error;
use yew::prelude::*;

use crate::app::SharedAppContext;
use crate::utils::notifications::component::{ActionButton, NotificationComponent};
use crate::utils::notifications::notification::{Notification, NotificationLevel};
use crate::utils::types::InRef;


pub trait ResultReport<T, E: Display> {
    fn or_notify(self, handle: &NotificationHandle) -> T;
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
                handle.notify(
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
                                        button.set_enabled(false);
                                    }
                                })
                                .build()
                        )
                        .build()
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
