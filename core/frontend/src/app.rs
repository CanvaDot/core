use gloo::storage::{LocalStorage, Storage};
use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::components::canvas::Canvas;
use crate::components::color_picker::ColorPicker;
use crate::components::hooks::notifications::{use_notifications, ResultReport};
use crate::components::notifications::hub::NotificationHub;
use crate::utils::notifications::component::{ActionButton, NotificationComponentKind};
use crate::utils::notifications::notification::{Notification, NotificationLevel};
use crate::utils::notifications::store::NotificationStore;
use crate::utils::types::InRef;

pub type SharedAppContext = UseStateHandle<AppContext>;

const CANVAS_HEIGHT: u32 = 1000;
const CANVAS_WIDTH: u32 = 1000;

#[derive(Default, Clone, PartialEq)]
pub struct AppContext {
    pub notifications: InRef<NotificationStore>,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_context = use_state(AppContext::default);
    let notification_hub = use_notifications();

    let on_draw = {
        Callback::from(|color: Srgb<u8>| {
            info!("The color is: {}, {}, {}", color.red, color.green, color.blue);
        })
    };

    if env!("CANVADOT_PROFILE") == "DEBUG" && LocalStorage::get("remind_build").unwrap_or(1) == 1 {
        let notification_hub = notification_hub.clone();

        app_context
            .notifications
            .borrow()
            .add(
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
                            .action(|notification: InRef<Notification>| {
                                notification
                                    .borrow()
                                    .close();
                            })
                            .build(),
                    )
                    .add_action_button(
                        ActionButton::builder()
                            .text("Stop reminding")
                            .kind(NotificationComponentKind::Secondary)
                            .action(move |notification: InRef<Notification>| {
                                LocalStorage::set("remind_build", 0).or_notify(&notification_hub);

                                notification
                                    .borrow()
                                    .close();
                            })
                            .build(),
                    )
                    .build(),
            );
    }

    html! {
        <ContextProvider<SharedAppContext> context={app_context.clone()}>
            <NotificationHub
                class="global-notification-hub"
                app_context={app_context.clone()}
                id="notification-hub"
            />
            <ColorPicker class="global-color-picker" on_draw={on_draw} />
            <Canvas
                pixel_height={CANVAS_HEIGHT}
                pixel_width={CANVAS_WIDTH}
                not_affecting_movement={vec![
                    "app".to_string(),
                    "notification-hub".to_string()
                ]}
            />
        </ContextProvider<SharedAppContext>>
    }
}
