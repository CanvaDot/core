use std::rc::Rc;

use gloo::timers::callback::Interval;
use yew::{classes, function_component, html, use_effect_with, use_state, Html, Properties};

use crate::app::SharedAppContext;
use crate::components::notifications::notification::NotificationElement;


#[derive(Properties, PartialEq)]
pub struct NotificationHubProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or(5)]
    pub max_notifications: u8,
    pub app_context: SharedAppContext,
}


#[function_component(NotificationHub)]
pub fn notification_hub(props: &NotificationHubProps) -> Html {
    let re_render = use_state(|| true);
    let notifications = props.app_context.notifications.clone();

    {
        let notifs_borrow = notifications.clone();

        use_effect_with((), |_| {
            let interval = Interval::new(1000, move || {
                notifs_borrow.borrow()
                    .remove_expired();
                re_render.set(!*re_render);
            });

            || {
                interval.cancel();
            }
        })
    }

    let notifs_borrow = notifications.borrow().all();
    let all_notifs = notifs_borrow.borrow();


    html! {
        <aside class={classes!(&props.class, "notification-hub")} aria-live="polite">
            {for all_notifs.iter().map(|notification| html! {
                <NotificationElement notification={Rc::clone(&notification)} />
            })}
        </aside>
    }
}
