use std::time::{Duration, Instant};

use yew::prelude::*;

use crate::app::SharedAppContext;
use crate::utils::notification_store::{NotificationComponent, NotificationLevel};

// TODO: make notification components, expiry and all

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
    let (buttons, dropdowns): (Vec<_>, Vec<_>) = props.components
        .iter()
        .partition(|component| match component {
            | NotificationComponent::RedirectButton { .. }
            | NotificationComponent::ActionButton { .. } => true,

            NotificationComponent::DropDown { .. } => false
        });

    return html! {
        <div>
            <span>{&props.title}</span>
            <p>{&props.message}</p>
        </div>
    }
}

#[function_component(NotificationHub)]
pub fn notification_hub(props: &NotificationHubProps) -> Html {

    html! {
        <aside class={classes!(&props.class, "notification-hub")} aria-live="polite">
        </aside>
    }
}
