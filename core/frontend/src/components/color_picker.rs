use std::rc::Rc;

use gloo::storage::errors::StorageError;
use gloo::storage::{LocalStorage, Storage};
use palette::rgb::channels::Rgba;
use palette::Srgb;
use thiserror::Error;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::hooks::notifications::{
    use_notifications,
    NotificationHandle,
    ResultReport,
};
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
    // DEPENDENCY FOR COLOR TYPE
    #[derive(Clone, Copy)]
    enum ChangeSliderType {
        Brightness,
        Hue,
    }

    // NOTIFICATION HANDLE
    let notification_hub = use_notifications();

    // COMPONENT STATE
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
    let picker_expanded = use_state(|| false);
    let picker_pinned = use_state(|| false);

    // DECOMPOSE COLOR INTO HUE AND LIGHTNESS
    let (hue, lightness) = decompose(&*current_color);

    // WHEN DRAW IS CLICKED FORWARD TO PROP CALLBACK
    let on_draw_event = {
        let current_color = current_color.clone();
        let on_draw = props
            .on_draw
            .clone();

        Callback::from(move |_| {
            on_draw.emit(*current_color);
        })
    };

    // COLOR IS PICKED FROM MEMORY EVENT
    let pick_color_memory_event = |color: Srgb<u8>| {
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

    // SLIDER IS PICKED UP EVENT
    fn start_slider_event<T: JsCast>(
        color_memory: UseStateHandle<ColorMemory>,
        current_color: UseStateHandle<Srgb<u8>>,
        notification_hub: Rc<NotificationHandle>,
    ) -> Callback<T> {
        Callback::from(move |_: T| {
            let mut new_memory = (*color_memory).clone();
            new_memory
                .push(*current_color)
                .or_notify(&notification_hub);
            color_memory.set(new_memory);
        })
    }

    // SLIDER IS MOVED EVENT
    let update_slider_event = |ty: ChangeSliderType| {
        let current_color = current_color.clone();
        let notification_hub = notification_hub.clone();

        Callback::from(move |event: InputEvent| {
            let input = event
                .target_dyn_into::<HtmlInputElement>()
                .ok_or(ColorPickerError::DynCastError)
                .or_notify(&notification_hub);

            let value = input.value_as_number();
            let new_color = match ty {
                ChangeSliderType::Brightness => compose(hue, value as u8),
                ChangeSliderType::Hue => compose(value as u16, lightness),
            };

            current_color.set(new_color);
        })
    };

    // SLIDER IS DROPPED DOWN EVENT
    fn commit_slider_event<T: JsCast>(
        current_color: UseStateHandle<Srgb<u8>>,
        notification_hub: Rc<NotificationHandle>,
        picker_pinned: UseStateHandle<bool>,
        picker_expanded: UseStateHandle<bool>,
    ) -> Callback<T> {
        Callback::from(move |_| {
            LocalStorage::set(LAST_COLOR_KEY, *current_color).or_notify(&notification_hub);

            if !*picker_pinned {
                picker_expanded.set(false);
            }
        })
    }

    // COLOR PICKER IS EXPANDED
    let expand_picker_event = {
        let picker_expanded = picker_expanded.clone();

        Callback::from(move |_| {
            picker_expanded.set(!*picker_expanded);
        })
    };

    let pin_picker_event = {
        let picker_pinned = picker_pinned.clone();

        Callback::from(move |_| {
            picker_pinned.set(!*picker_pinned);
        })
    };

    // UPDATE COLOR FROM WRITTEN VALUES LOOP
    // MAYBE COMMING SOON
    // {
    //     let current_color = current_color.clone();
    //
    //     use_effect_with((), move |_| {
    //         let value_selector_loop = Interval::new(1000, || {
    //
    //         });
    //
    //         || { value_selector_loop.cancel(); }
    //     })
    // }

    html! {
            <div class={classes!(&props.class, "color-picker-container")}>
                if *picker_expanded {
                    <div class="color-picker-selector">
                        <div
                            class="color-picker-selector-extra"
                            data-pinned={(*picker_pinned).to_string()}
                        >
                            <span class="cps-extra-format">
                                {format!(
                                    "rgb({}, {}, {}) - #{:06X}",
                                    (*current_color).red,
                                    (*current_color).green,
                                    (*current_color).blue,
                                    (*current_color).into_u32::<Rgba>() >> 8
                                )}
                            </span>
                            <div class="cps-extra-utils">
                                <Icon
                                    icon_id={IconId::FontAwesomeSolidEyeDropper}
                                    class="cps-extra-copy"
                                />
                                <Icon
                                    icon_id={IconId::FontAwesomeSolidThumbtack}
                                    onclick={pin_picker_event}
                                    class="cps-extra-pin"
                                />
                            </div>
                        </div>
    //                    <div class="color-picker-selector-values">
    //                        <div>
    //                            <label>
    //                                <span>{"#"}</span>
    //                                <input
    //                                    type="text"
    //                                    value={format!("{:06X}", (*current_color).into_u32::<Rgba>() >> 8)}
    //                                />
    //                            </label>
    //                        </div>
    //                        <div>
    //                            <label>
    //                                <span>{"R"}</span>
    //                                <input
    //                                    type="number"
    //                                    value={(*current_color).red.to_string()}
    //                                />
    //                            </label>
    //                            <label>
    //                                <span>{"G"}</span>
    //                                <input
    //                                    type="number"
    //                                    value={(*current_color).green.to_string()}
    //                                />
    //                            </label>
    //                            <label>
    //                                <span>{"B"}</span>
    //                                <input
    //                                    type="number"
    //                                    value={(*current_color).blue.to_string()}
    //                                />
    //                            </label>
    //                        </div>
    //                    </div>
                        <div class="color-picker-selector-sliders">
                            <input
                                type="range"
                                value={hue.to_string()}
                                min="0"
                                max="350"
                                style={format!("--brightness: {}%", lightness)}

                                onmousedown={start_slider_event(
                                    color_memory.clone(),
                                    current_color.clone(),
                                    notification_hub.clone()
                                )}
                                ontouchstart={start_slider_event(
                                    color_memory.clone(),
                                    current_color.clone(),
                                    notification_hub.clone()
                                )}
                                oninput={update_slider_event(ChangeSliderType::Hue)}
                                onmouseup={commit_slider_event(
                                    current_color.clone(),
                                    notification_hub.clone(),
                                    picker_pinned.clone(),
                                    picker_expanded.clone()
                                )}
                                ontouchend={commit_slider_event(
                                    current_color.clone(),
                                    notification_hub.clone(),
                                    picker_pinned.clone(),
                                    picker_expanded.clone()
                                )}
                            />
                            <input
                                type="range"
                                value={lightness.to_string()}
                                min="1"
                                max="99"
                                onmousedown={start_slider_event(
                                    color_memory.clone(),
                                    current_color.clone(),
                                    notification_hub.clone()
                                )}
                                ontouchstart={start_slider_event(
                                    color_memory.clone(),
                                    current_color.clone(),
                                    notification_hub.clone()
                                )}
                                oninput={update_slider_event(ChangeSliderType::Brightness)}
                                onmouseup={commit_slider_event(
                                    current_color.clone(),
                                    notification_hub.clone(),
                                    picker_pinned.clone(),
                                    picker_expanded.clone()
                                )}
                                ontouchend={commit_slider_event(
                                    current_color.clone(),
                                    notification_hub.clone(),
                                    picker_pinned.clone(),
                                    picker_expanded.clone()
                                )}
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
                                onclick={pick_color_memory_event(color.clone())}
                            >
                            </button>
                        }
                    }) }
                </div>
            </div>
        }
}
