use std::rc::Rc;

use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::utils::notification_store::NotificationStore;
use crate::components::color_picker::ColorPicker;
use crate::components::notification_hub::NotificationHub;
use crate::components::common::{AppButton, ButtonTarget, AppDropdown};

pub type SharedAppContext = UseStateHandle<AppContext>;

#[derive(Default, Clone, PartialEq)]
pub struct AppContext {
    pub notifications: Rc<NotificationStore>
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
                enabled=false
            />

            <NotificationHub app_context={app_context.clone()} />
            <ColorPicker class="global-color-picker" on_draw={on_draw} />
        </ContextProvider<SharedAppContext>>
    }
}
