use palette::Srgb;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::utils::color_memory::ColorMemory;

const MAX_LAST_COLORS: usize = 5;

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    #[prop_or_default]
    pub classname: String,

    #[prop_or_default]
    pub on_draw: Callback<Srgb<u8>>,
}

#[function_component(ColorPicker)]
pub fn color_picker(props: &ColorPickerProps) -> Html {
    let color_memory = use_state(||
        ColorMemory::from_ls("color_memory".into(), MAX_LAST_COLORS)
        .expect("Color memory to not have an error, errors this way should be removed.")
    );
    let current_color = use_state(|| (*color_memory)
        .last_obtained()
        .expect("Color memory to not have an error, errors this way should be removed.")
        .unwrap_or(Srgb::new(0, 105, 255))
    );

    let on_draw_event = {
        let current_color = current_color.clone();
        let on_draw = props.on_draw.clone();

        Callback::from(move |_| {
            on_draw.emit(*current_color);
        })
    };

    html! {
        <div
            class={classes!(&props.classname, "color-picker-container")}
        >
            <button
                class="color-picker-paint"
                onclick={on_draw_event}
                style={format!(
                    "background-color: #{:02X}{:02X}{:02X}",
                    current_color.red,
                    current_color.green,
                    current_color.blue
                )}
            >
                <Icon
                    icon_id={IconId::FontAwesomeSolidPaintbrush}
                />
            </button>
            <div class="color-picker-colors">
                <button class="color-picker-selector"></button>
                { for (*color_memory).iter().map(|color| {
                    html! {
                        <button
                            style={ format!(
                                "background-color: #{:02X}{:02X}{:02X}",
                                color.red,
                                color.green,
                                color.blue
                            ) }
                            class="color-picker-color"
                        >
                        </button>
                    }
                }) }
            </div>
        </div>
    }
}
