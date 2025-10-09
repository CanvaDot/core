use gloo::events::EventListener;
use gloo::utils::window;
use thiserror::Error;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::hooks::notifications::{use_notifications, ResultReport};

const ZOOM_SENSITIVITY: f64 = 750.0;
const ZOOM_MIN: f64 = 2.0;
const ZOOM_MAX: f64 = 5.0;

#[derive(Error, Debug)]
enum CanvasError {
    #[error("Couldn't cast the event target.")]
    DynCastError,
}

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    // canvas base size.
    pub height: u32,
    pub width: u32,

    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: String,
}

#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let notification_hub = use_notifications();

    let zoom_level = use_state(|| ZOOM_MIN);
    let is_dragging = use_state(|| false);
    let position = use_state(|| (0.0, 0.0));
    let last_position = use_mut_ref(|| (0.0, 0.0));

    let on_wheel_event = {
        let zoom_level = zoom_level.clone();

        Callback::from(move |event: WheelEvent| {
            event.prevent_default();

            let delta = -event.delta_y() / ZOOM_SENSITIVITY;
            let new_zoom = (*zoom_level + delta).clamp(ZOOM_MIN, ZOOM_MAX);
            zoom_level.set(new_zoom);
        })
    };

    {
        let is_dragging = is_dragging.clone();
        let position = position.clone();
        let last_position = last_position.clone();
        let notification_hub = notification_hub.clone();

        use_effect(move || {
            let window = window();

            let mouse_down_listener = {
                let is_dragging = is_dragging.clone();
                let last_position = last_position.clone();
                let notification_hub = notification_hub.clone();

                EventListener::new(&window, "mousedown", move |event| {
                    let event = event
                        .dyn_ref::<MouseEvent>()
                        .ok_or(CanvasError::DynCastError)
                        .or_notify(&notification_hub);

                    is_dragging.set(true);
                    *last_position.borrow_mut() =
                        (event.client_x() as f64, event.client_y() as f64);
                })
            };

            let mouse_up_listener = {
                let is_dragging = is_dragging.clone();

                EventListener::new(&window, "mouseup", move |_| {
                    is_dragging.set(false);
                })
            };

            let mouse_move_listener = {
                let is_dragging = is_dragging.clone();
                let last_position = last_position.clone();
                let position = position.clone();
                let notification_hub = notification_hub.clone();

                EventListener::new(&window, "mousemove", move |event| {
                    if !*is_dragging {
                        return;
                    }

                    let event = event
                        .dyn_ref::<MouseEvent>()
                        .ok_or(CanvasError::DynCastError)
                        .or_notify(&notification_hub);

                    let (lx, ly) = *last_position.borrow();
                    let dx = event.client_x() as f64 - lx;
                    let dy = event.client_y() as f64 - ly;
                    *last_position.borrow_mut() =
                        (event.client_x() as f64, event.client_y() as f64);
                    position.set((position.0 + dx, position.1 + dy));
                })
            };

            || {
                drop(mouse_down_listener);
                drop(mouse_up_listener);
                drop(mouse_move_listener);
            }
        });
    }

    html! {
        <canvas
            id={props.id.clone()}
            class={classes!("canvas-main", &props.class)}
            height={props.height.to_string()}
            width={props.width.to_string()}

            onwheel={on_wheel_event}

            style={
                format!(
                    r"
                        --scale: {};
                        --left: {}px;
                        --top: {}px;
                    ",
                    *zoom_level,
                    position.0,
                    position.1
                )
            }
        />
    }
}
