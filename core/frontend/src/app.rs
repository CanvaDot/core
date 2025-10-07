use gloo::storage::{LocalStorage, Storage};
use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::components::color_picker::ColorPicker;
use crate::components::notifications::hub::NotificationHub;
use crate::utils::notifications::component::{ActionButton, NotificationComponentKind};
use crate::utils::notifications::notification::{Notification, NotificationLevel};
use crate::utils::notifications::store::send_notification;
use crate::utils::types::AtomicInRef;

pub type SharedAppContext = UseStateHandle<AppContext>;

#[derive(Default, Clone, PartialEq)]
pub struct AppContext {
    // let this be empty for future implementations (I guess.)
}

#[function_component(App)]
pub fn app() -> Html {
    let app_context = use_state(AppContext::default);

    let on_draw = {
        Callback::from(|color: Srgb<u8>| {
            info!("The color is: {}, {}, {}", color.red, color.green, color.blue);
        })
    };

    if env!("CANVADOT_PROFILE") == "DEBUG" && LocalStorage::get("remind_build").unwrap_or(1) == 1 {
        send_notification(
            Notification::builder()
                .title("Test Build")
                .level(NotificationLevel::Info)
                .message(format!(
                    "You are using a test build, generated at {}",
                    env!("CANVADOT_BUILD_AGE")
                ))
                .add_action_button(
                    ActionButton::builder()
                        .text("Dismiss")
                        .action(|notification: AtomicInRef<Notification>| {
                            notification
                                .lock()
                                .unwrap()
                                .close();
                        })
                        .build(),
                )
                .add_action_button(
                    ActionButton::builder()
                        .text("Stop reminding")
                        .kind(NotificationComponentKind::Secondary)
                        .action(move |notification: AtomicInRef<Notification>| {
                            LocalStorage::set("remind_build", 0);

                            notification
                                .lock()
                                .unwrap()
                                .close();
                        })
                        .build(),
                )
                .build()
        );
    }

    html! {
        <ContextProvider<SharedAppContext> context={app_context.clone()}>
            <NotificationHub class="global-notification-hub" app_context={app_context.clone()} />
            <ColorPicker class="global-color-picker" on_draw={on_draw} />
        </ContextProvider<SharedAppContext>>
    }
}
