use log::info;
use palette::Srgb;
use yew::prelude::*;

use crate::components::color_picker::ColorPicker;

#[function_component(App)]
pub fn app() -> Html {
    let on_draw = {
        Callback::from(|color: Srgb<u8>| {
            info!("The color is: {}, {}, {}", color.red, color.green, color.blue);
        })
    };

    html! {
        <ColorPicker classname="global-color-picker" on_draw={on_draw} />
    }
}
