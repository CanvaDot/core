use std::time::{Duration, Instant};

use yew::prelude::*;

use crate::app::SharedAppContext;
use crate::utils::notification_store::{Notification, NotificationComponent, NotificationLevel};

// TODO: make notification components, expiry and all
// TODO: make common components such as buttons, inputs...

#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub title: String,
    pub message: String,
    #[prop_or(NotificationLevel::Info)]
    pub level: NotificationLevel,
    #[prop_or_default]
    pub components: Vec<NotificationComponent>,

    pub created_at: Instant,
    pub duration: Duration
}

#[derive(Properties, PartialEq)]
pub struct NotificationHubProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(5)]
    pub max_notifications: u8,

    pub app_context: SharedAppContext
}

#[function_component(NotificationElement)]
fn notification_element(props: &NotificationProps) -> Html {
    return html! {

    }
}

#[function_component(NotificationHub)]
pub fn notification_hub(props: &NotificationHubProps) -> Html {

    html! {
        <aside class={classes!(&props.class, "notification-hub")} aria-live="polite">
        </aside>
    }
}
