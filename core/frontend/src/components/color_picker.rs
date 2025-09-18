use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use palette::rgb::channels::Rgba;
use palette::Srgb;
use thiserror::Error;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::hooks::notifications::{use_notifications, ResultReport};
use crate::utils::color_memory::ColorMemory;
use crate::utils::colors::{compose, decompose};

const MAX_LAST_COLORS: usize = 5;
const COLOR_MEMORY_KEY: &str = "color_memory";
const LAST_COLOR_KEY: &str = "last_color";

#[derive(Error, Debug)]
enum ColorPickerError {
    #[error("Couldn't cast the event target.")]
    DynCastError,
}

#[derive(Properties, PartialEq)]
pub struct ColorPickerProps {
    #[prop_or_default]
    pub class: String,

    #[prop_or_default]
    pub on_draw: Callback<Srgb<u8>>,
}

#[function_component(ColorPicker)]
pub fn color_picker(props: &ColorPickerProps) -> Html {
    let notification_hub = use_notifications();

    let color_memory = use_state(|| {
        ColorMemory::from_ls(COLOR_MEMORY_KEY.into(), MAX_LAST_COLORS).or_notify(&notification_hub)
    });
    let current_color = use_state(|| {
        LocalStorage::get(LAST_COLOR_KEY)
            .or_else(|error| match error {
                StorageError::KeyNotFound(_) => Ok(None),
                error => Err(error),
            })
            .or_notify(&notification_hub)
            .unwrap_or(Srgb::new(0, 105, 255))
    });
    let picker_expanded = use_state(|| !false);

    let on_draw_event = {
        let current_color = current_color.clone();
        let on_draw = props
            .on_draw
            .clone();

        Callback::from(move |_| {
            on_draw.emit(*current_color);
        })
    };

    let change_color_event = |color: Srgb<u8>| {
        let current_color = current_color.clone();
        let color_memory = color_memory.clone();
        let notification_hub = notification_hub.clone();

        Callback::from(move |_| {
            let mut new_memory = (*color_memory).clone();

            new_memory
                .push(*current_color)
                .or_notify(&notification_hub);

            color_memory.set(new_memory);
            current_color.set(color);
        })
    };

    let (hue, lightness) = decompose(&*current_color);

    #[derive(Clone, Copy)]
    enum ChangeSliderType {
        Brightness,
        Hue,
    }

    let change_slider_event = |ty: ChangeSliderType| {
        let current_color = current_color.clone();
        let color_memory = color_memory.clone();
        let notification_hub = notification_hub.clone();

        Callback::from(move |event: Event| {
            let value = event
                .target_dyn_into::<HtmlInputElement>()
                .ok_or(ColorPickerError::DynCastError)
                .or_notify(&notification_hub)
                .value_as_number();

            let new_color = match ty {
                ChangeSliderType::Brightness => compose(hue, value as u8),
                ChangeSliderType::Hue => compose(value as u16, lightness),
            };

            LocalStorage::set(LAST_COLOR_KEY, new_color).or_notify(&notification_hub);

            let mut new_memory = (*color_memory).clone();
            new_memory
                .push(*current_color)
                .or_notify(&notification_hub);
            color_memory.set(new_memory);

            current_color.set(new_color);
        })
    };

    let expand_picker_event = {
        let picker_expanded = picker_expanded.clone();

        Callback::from(move |_| {
            picker_expanded.set(!*picker_expanded);
        })
    };

    html! {
        <div class={classes!(&props.class, "color-picker-container")}>
            if *picker_expanded {
                <div class="color-picker-selector">
                    <div class="color-picker-selector-values">
                    </div>
                    <div class="color-picker-selector-sliders">
                        <input
                            type="range"
                            value={hue.to_string()}
                            min="0"
                            max="350"
                            onchange={change_slider_event(ChangeSliderType::Hue)}
                            style={format!("--brightness: {}%", (lightness as f32 / 50.0) * 100.0)}
                        />
                        <input
                            type="range"
                            value={lightness.to_string()}
                            min="1"
                            max="100"
                            onchange={change_slider_event(ChangeSliderType::Brightness)}
                        />
                    </div>
                </div>
            }

            <button
                class="color-picker-paint color-picker-after"
                onclick={on_draw_event}
                style={format!("background-color: #{:08X}", current_color.into_u32::<Rgba>())}
            >
                <Icon icon_id={IconId::FontAwesomeSolidPaintbrush} />
            </button>
            <div class="color-picker-colors">
                <button
                    class="color-picker-toggle-selector"
                    onclick={expand_picker_event}
                >
                </button>
                { for (*color_memory).iter().map(|color| {
                    html! {
                        <button
                            style={ format!("background-color: #{:08X}", color.into_u32::<Rgba>()) }
                            class="color-picker-color"
                            onclick={change_color_event(color.clone())}
                        >
                        </button>
                    }
                }) }
            </div>
        </div>
    }
}
