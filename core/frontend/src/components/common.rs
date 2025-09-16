use std::cmp::min;

use gloo::utils::window;
use yew::prelude::*;
use palette::{rgb::channels::Rgba, Srgb};
use yew_icons::{Icon, IconId};

use crate::utils::colors::{contrasting_bw, INFO_BLUE};


#[derive(PartialEq, Clone)]
pub enum ButtonTarget {
    Callback(Callback<MouseEvent>),
    Link(String)
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub target: ButtonTarget,
    pub text: String,
    #[prop_or(INFO_BLUE)]
    pub color: Srgb<u8>,
    #[prop_or(true)]
    pub enabled: bool,
    

    #[prop_or_default]
    class: String,
    #[prop_or_default]
    id: String,
    #[prop_or_default]
    style: String
}

#[function_component(AppButton)]
pub fn app_button(props: &ButtonProps) -> Html {
    let on_click = {
        let target = props.target.clone();
        Callback::from(move |event| {
            match &target {
                ButtonTarget::Callback(callback) => {
                    callback.emit(event);
                }

                ButtonTarget::Link(link) => {
                    window()
                        .location()
                        .set_href(&link)
                        .expect("To redirect.");
                }
            }
        })
    };

    let text_color = contrasting_bw(&props.color);

    html! {
        <button
            class={classes!("common-button", &props.class)}
            id={Some(props.id.clone()).take_if(|id| !id.is_empty())}
            disabled={!props.enabled}
            onclick={props.enabled.then_some(on_click)}
            style={format!(
                stringify!(
                    background-color: #{:08X};
                    color: #{:08X};
                    {}
                ),
                props.color.into_u32::<Rgba>(),
                text_color.into_u32::<Rgba>(),
                props.style
            )}
        >
            {&props.text}
        </button>
    }
}


#[derive(Properties, PartialEq)]
pub struct DropdownProps<'i> {
    #[prop_or(0)]
    pub default: usize,
    pub items: Vec<(&'i str, &'i str)>,
    #[prop_or(true)]
    pub enabled: bool,
    #[prop_or_default]
    pub on_change: Callback<String>,

    #[prop_or_default]
    class: String,
    #[prop_or_default]
    id: String,
    #[prop_or_default]
    style: String
}

#[function_component(AppDropdown)]
pub fn app_dropdown(props: &DropdownProps<'static>) -> Html {
    let selected_key = use_state(||
        props.items
            .get(min(props.items.len() - 1, props.default))
            .map(|(key, _)| key.to_string())
            .unwrap_or(String::new())
    );
    let expanded = use_state(|| false);

    let on_click = |key: String, value: String| {
        let callback = props.on_change.clone();
        let selected_key = selected_key.clone();

        Callback::from(move |_| {
            callback.emit(value.clone());
            selected_key.set(key.clone());
        })
    };

    let expand = {
        let expanded = expanded.clone();

        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    let options = props.items
        .iter()
        .map(|(key, value)| html! {
            <div
                onclick={on_click(
                    key.to_string(),
                    value.to_string()
                )}
            >
                <span>{key}</span>
            </div>
        });

    html! {
        <div
            class={classes!(
                "common-select",
                (!props.enabled).then_some("common-select-disabled"),
                &props.class
            )}
            onclick={expand}
        >
            <span>{&*selected_key}</span>
            <Icon icon_id={IconId::FontAwesomeSolidChevronDown} />
            if *expanded && props.enabled {
                <div>{for options}</div>
            }
        </div>
    }
}
