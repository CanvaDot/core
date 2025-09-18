use gloo::timers::callback::Interval;
use palette::rgb::channels::Rgba;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::app::SharedAppContext;
use crate::utils::notification_store::{Notification, NotificationComponent};

// TODO: make notification components, expiry and all

#[derive(Properties, PartialEq)]
pub struct NotificationHubProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(5)]
    pub max_notifications: u8,

    pub app_context: SharedAppContext,
}

#[function_component(NotificationElement)]
fn notification_element(props: &Notification) -> Html {
    let (buttons, dropdowns): (Vec<_>, Vec<_>) = props
        .components()
        .iter()
        .partition(|component| match component {
            | NotificationComponent::RedirectButton { .. }
            | NotificationComponent::ActionButton { .. } => true,

            NotificationComponent::DropDown { .. } => false,
        });

    return html! {
        <div
            class="notification-element"
            style={format!(
                "--accent-color: #{:08X}",
                props
                    .level()
                    .to_color()
                    .into_u32::<Rgba>()
            )}
        >
            <Icon icon_id={IconId::FontAwesomeSolidX} />
            <h1>{&props.title()}</h1>
            <p>{&props.message()}</p>
        </div>
    };
}

#[function_component(NotificationHub)]
pub fn notification_hub(props: &NotificationHubProps) -> Html {
    let current_notifications = use_state(|| Vec::<&Notification>::new());

    {
        let current_notifications = current_notifications.clone();
        let incoming_notifications = props
            .app_context
            .notifications
            .clone();

        use_effect(|| {
            // this is an interval, if it doesn't update state it should run again.
            let interval = Interval::new(1000, move || {
                incoming_notifications.remove_expired();

                current_notifications.set(
                    incoming_notifications
                        .all()
                        .borrow()
                        .iter()
                        .collect(),
                )
            });

            || {
                interval.cancel();
            }
        });
    }

    let mut current_notifications = (*current_notifications).clone();
    current_notifications.sort_by(|a, b| {
        b.created_at()
            .cmp(&a.created_at())
    });

    let notification = Notification::new("Test", "This is a test.").set_components(vec![
        NotificationComponent::RedirectButton {
            text: "".to_string(),
            redirect: "".to_string(),
            enabled: true,
            kind:
        }
    ]);

    html! {
        <aside class={classes!(&props.class, "notification-hub")} aria-live="polite">
            {for current_notifications.iter().map(|props| html! {
                <NotificationElement ..{props.clone()} />
            })}
            <NotificationElement ..notification />
        </aside>
    }
}
