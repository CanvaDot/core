use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::components::color_picker::ColorPicker;
use crate::components::common::{AppButton, AppSelect, ButtonTarget};
use crate::components::notifications::hub::NotificationHub;
use crate::utils::notifications::store::NotificationStore;
use crate::utils::types::InRef;

pub type SharedAppContext = UseStateHandle<AppContext>;

#[derive(Default, Clone, PartialEq)]
pub struct AppContext {
    pub notifications: InRef<NotificationStore>,
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

            <NotificationHub class="global-notification-hub" app_context={app_context.clone()} />
            <ColorPicker class="global-color-picker" on_draw={on_draw} />
        </ContextProvider<SharedAppContext>>
    }
}
