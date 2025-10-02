use std::rc::Rc;

use gloo::timers::callback::Interval;
use uuid::Uuid;
use yew::{
    classes,
    function_component,
    html,
    use_effect_with,
    use_state,
    Callback,
    Html,
    Properties,
};

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
    let notifications = props
        .app_context
        .notifications
        .clone();

    {
        let notifications = notifications.clone();

        use_effect_with((), |()| {
            let interval = Interval::new(500, move || {
                notifications
                    .borrow()
                    .remove_expired();
                re_render.set(!*re_render);
            });

            || {
                interval.cancel();
            }
        });
    }

    let on_close_notification = {
        let notifications = notifications.clone();

        Callback::from(move |id: Uuid| {
            notifications
                .borrow()
                .remove_by_id(id);
        })
    };

    let notifs_borrow = notifications
        .borrow()
        .all();
    let all_notifs = notifs_borrow.borrow();

    html! {
        <aside class={classes!(&props.class, "notification-hub")} aria-live="polite">
            {for all_notifs.iter().map(|notification| html! {
                <NotificationElement
                    notification={Rc::clone(notification)}
                    on_close={&on_close_notification}
                />
            })}
        </aside>
    }
}
