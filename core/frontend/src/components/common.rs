use std::cmp::min;

use gloo::events::EventListener;
use gloo::utils::{document, window};
use palette::rgb::channels::Rgba;
use palette::Srgb;
use thiserror::Error;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, Node};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

use crate::components::hooks::notifications::{use_notifications, ResultReport};
use crate::utils::colors::{contrasting_bw, INFO_BLUE};

#[derive(Error, Debug)]
pub enum CommonElementError {
    #[error(
        "Couldn't find element cordinate: {}",
        .0.as_string().unwrap_or(format!("{:?}", .0))
    )]
    MissingCordinate(JsValue),

    #[error("Could not find an HTML element with the '{0}' selector.")]
    MissingElement(String),
}

#[derive(PartialEq, Clone)]
pub enum ButtonTarget {
    Callback(Callback<MouseEvent>),
    Link(String),
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
    style: String,
}

#[function_component(AppButton)]
pub fn app_button(props: &ButtonProps) -> Html {
    let on_click = {
        let target = props
            .target
            .clone();

        Callback::from(move |event| match &target {
            ButtonTarget::Callback(callback) => {
                callback.emit(event);
            },

            ButtonTarget::Link(link) => {
                window()
                    .location()
                    .set_href(&link)
                    .expect("To redirect.");
            },
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
pub struct DropdownProps {
    #[prop_or(0)]
    pub default: usize,
    pub values: Vec<(String, String)>,
    #[prop_or(true)]
    pub enabled: bool,
    #[prop_or_default]
    pub onchange: Callback<String>,

    #[prop_or_default]
    class: String,
    #[prop_or_default]
    id: String,
    #[prop_or_default]
    style: String,
}

#[function_component(AppSelect)]
pub fn app_select(props: &DropdownProps) -> Html {
    let expanded = use_state(|| false);
    let portal_style = use_state(|| None);
    let trigger_ref = use_node_ref();
    let notification_hub = use_notifications();
    let selected_key = use_state(|| {
        props
            .values
            .get(min(
                props
                    .values
                    .len()
                    - 1,
                props.default,
            ))
            .map(|(key, _)| key.to_string())
            .unwrap_or(String::new())
    });

    let on_click = |key: String, value: String| {
        let callback = props
            .onchange
            .clone();
        let selected_key = selected_key.clone();

        Callback::from(move |_| {
            callback.emit(value.clone());
            selected_key.set(key.clone());
        })
    };

    {
        let expanded = expanded.clone();
        let trigger_ref = trigger_ref.clone();

        use_effect_with((*expanded).clone(), move |is_expanded| {
            if *is_expanded {
                let listener = EventListener::new(&document(), "click", move |event| {
                    if let (Some(target), Some(trigger)) = (
                        event
                            .target()
                            .and_then(|target| {
                                target
                                    .dyn_into::<Node>()
                                    .ok()
                            }),
                        trigger_ref.cast::<Node>(),
                    ) {
                        if !trigger.contains(Some(&target)) {
                            expanded.set(false);
                        }
                    }
                });

                Box::new(move || {
                    drop(listener);
                }) as Box<dyn FnOnce()>
            } else {
                Box::new(|| ()) as Box<dyn FnOnce()>
            }
        })
    }

    let expand = {
        let expanded = expanded.clone();

        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    let options = props
        .values
        .iter()
        .map(|(key, value)| {
            html! {
                <div
                    onclick={on_click(
                        key.to_string(),
                        value.to_string()
                    )}
                >
                    <span>{key}</span>
                </div>
            }
        });

    {
        let portal_style = portal_style.clone();
        let expanded = expanded.clone();
        let notification_hub = notification_hub.clone();
        let trigger_ref = trigger_ref.clone();

        use_effect_with((*expanded).clone(), move |expanded| {
            if *expanded {
                if let Some(element) = trigger_ref.cast::<Element>() {
                    let window = window();
                    let rect = element.get_bounding_client_rect();

                    let style = format!(
                        r#"
                            position: absolute;

                            top: {}px;
                            left: {}px;

                            width: {}px;

                            z-index: 100;
                        "#,
                        2.0 + rect.bottom()
                            + window
                                .scroll_y()
                                .map_err(|error| CommonElementError::MissingCordinate(error))
                                .or_notify(&notification_hub),
                        rect.left()
                            + window
                                .scroll_x()
                                .map_err(|error| CommonElementError::MissingCordinate(error))
                                .or_notify(&notification_hub),
                        rect.width()
                    );

                    portal_style.set(Some(style));
                }
            }
        });
    }

    let portal = if *expanded && props.enabled {
        let host = document()
            .get_element_by_id("tooltip-portal")
            .ok_or(CommonElementError::MissingElement("#tooltip-portal".into()))
            .or_notify(&notification_hub);

        create_portal(
            html! {
                <div class="common-select-dropdown" style={(*portal_style).clone()}>
                    {for options}
                </div>
            },
            host,
        )
    } else {
        Html::default()
    };

    html! {
        <div
            ref={trigger_ref.clone()}
            class={classes!(
                "common-select",
                (!props.enabled).then_some("common-select-disabled"),
                &props.class
            )}
            onclick={expand}
        >
            <span>{&*selected_key}</span>
            <Icon icon_id={IconId::FontAwesomeSolidChevronDown} />
            {portal}
        </div>
    }
}
