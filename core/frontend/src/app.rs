use std::rc::Rc;

use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::components::color_picker::ColorPicker;
use crate::components::common::{AppButton, AppDropdown, ButtonTarget};
use crate::components::notification_hub::NotificationHub;
use crate::utils::notifications::store::NotificationStore;

pub type SharedAppContext<'c> = UseStateHandle<AppContext<'c>>;

#[derive(Default, Clone, PartialEq)]
pub struct AppContext<'c> {
    pub notifications: Rc<NotificationStore<'c>>,
}

#[function_component(App)]
pub fn app() -> Html {
    let app_context = use_state(|| AppContext::default());

    let on_draw = {
        Callback::from(|color: Srgb<u8>| {
            info!("The color is: {}, {}, {}", color.red, color.green, color.blue);
        })
    };

    html! {
        <ContextProvider<SharedAppContext> context={app_context.clone()}>
            <AppButton text="test" target={ButtonTarget::Link("/".into())} enabled=true />
            <AppDropdown
                items={vec![
                    ("Literally", "lit"),
                    ("Not Really", "no")
                ]}
                on_change={|val| log::info!("{val}")}
                default=3
                enabled=true
            />

            <NotificationHub class="global-notification-hub" app_context={app_context.clone()} />
            <ColorPicker class="global-color-picker" on_draw={on_draw} />
        </ContextProvider<SharedAppContext>>
    }
}
