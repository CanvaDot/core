use log::info;
use yew::prelude::*;

use crate::components::color_picker::ColorPicker;

#[function_component(App)]
pub fn app() -> Html {
    let on_draw = {
        Callback::from(|color| {
            info!("The color is: {}", color);
        })
    };

    html! {
        <ColorPicker classname="global-color-picker" on_draw={on_draw} />
    }
}
