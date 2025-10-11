use gloo::events::EventListener;
use gloo::utils::{document, window};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

use crate::components::hooks::notifications::{use_notifications, ResultReport};
use crate::utils::error::GenericError;
use crate::utils::types::TupleCords;

const ZOOM_SENSITIVITY: f64 = 20000.0;
const ZOOM_MIN: f64 = 0.1;
const ZOOM_MAX: f64 = 5.0;
const DIVISION: u32 = 10;

#[derive(Properties, PartialEq)]
pub struct CanvasProps {
    // canvas base size.
    pub pixel_height: u32,
    pub pixel_width: u32,

    #[prop_or(ZOOM_MIN)]
    pub default_zoom_level: f64,
    #[prop_or_default]
    pub default_position: Option<(f64, f64)>,

    #[prop_or_default]
    pub not_affecting_movement: Vec<String>,

    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub id: String,
}

#[allow(clippy::cast_sign_loss, clippy::cast_lossless, clippy::cast_precision_loss)]
#[function_component(Canvas)]
pub fn canvas(props: &CanvasProps) -> Html {
    let notification_hub = use_notifications();

    let zoom_level = use_state(|| props.default_zoom_level);
    let is_dragging = use_state(|| false);
    let position = use_state(|| {
        props
            .default_position
            .unwrap_or((0.0, 0.0))
    });
    let last_position = use_mut_ref(|| *position);

    let canvas_ref = use_node_ref();

    {
        let canvas_ref = canvas_ref.clone();

        let zoom_level = zoom_level.clone();
        let is_dragging = is_dragging.clone();
        let position = position.clone();
        let last_position = last_position.clone();
        let notification_hub = notification_hub.clone();

        let canvas_pixel_width = props.pixel_width;
        let canvas_pixel_height = props.pixel_height;

        let not_affecting_movement = props
            .not_affecting_movement
            .clone();

        use_effect(move || {
            let window_inst = window();

            let zoom_level_listener = {
                let zoom_level = zoom_level.clone();
                let notification_hub = notification_hub.clone();

                EventListener::new(&window_inst, "wheel", move |event| {
                    let event = event
                        .dyn_ref::<WheelEvent>()
                        .ok_or(GenericError::DynCastError)
                        .or_notify(&notification_hub);

                    event.prevent_default();

                    let delta = -event.delta_y() / ZOOM_SENSITIVITY;
                    let new_zoom = (*zoom_level + delta).clamp(ZOOM_MIN, ZOOM_MAX);
                    zoom_level.set(new_zoom);
                })
            };

            let mouse_down_listener = {
                let is_dragging = is_dragging.clone();
                let last_position = last_position.clone();
                let notification_hub = notification_hub.clone();
                let canvas_ref = canvas_ref.clone();

                EventListener::new(&window_inst, "mousedown", move |event| {
                    let event = event
                        .dyn_ref::<MouseEvent>()
                        .ok_or(GenericError::DynCastError)
                        .or_notify(&notification_hub);

                    let hovered_element = document()
                        .element_from_point(event.client_x() as f32, event.client_y() as f32);

                    let document_inst = document();

                    let mut not_affecting_movement = not_affecting_movement
                        .iter()
                        .filter_map(|id| document_inst.get_element_by_id(id))
                        .collect::<Vec<_>>();

                    if let Some(canvas_element) = canvas_ref.cast::<Element>() {
                        not_affecting_movement.push(canvas_element);
                    }

                    if let Some(hovered_element) = hovered_element {
                        let should_not_affect_movement = not_affecting_movement
                            .iter()
                            .any(|element| element == &hovered_element);

                        if !should_not_affect_movement {
                            return;
                        }
                    }

                    is_dragging.set(true);
                    *last_position.borrow_mut() =
                        (f64::from(event.client_x()), f64::from(event.client_y()));
                })
            };

            let mouse_up_listener = {
                let is_dragging = is_dragging.clone();

                EventListener::new(&window_inst, "mouseup", move |_| {
                    is_dragging.set(false);
                })
            };

            let mouse_move_listener = {
                let is_dragging = is_dragging.clone();
                let last_position = last_position.clone();
                let position = position.clone();
                let notification_hub = notification_hub.clone();

                EventListener::new(&window_inst, "mousemove", move |event| {
                    if !*is_dragging {
                        return;
                    }

                    let event = event
                        .dyn_ref::<MouseEvent>()
                        .ok_or(GenericError::DynCastError)
                        .or_notify(&notification_hub);

                    let (lx, ly) = *last_position.borrow();
                    let dx = event.client_x() as f64 - lx;
                    let dy = event.client_y() as f64 - ly;
                    *last_position.borrow_mut() =
                        (event.client_x() as f64, event.client_y() as f64);

                    let canvas_width = canvas_pixel_width as f64 * (DIVISION as f64 * *zoom_level);
                    let canvas_height =
                        canvas_pixel_height as f64 * (DIVISION as f64 * *zoom_level);
                    let max_offset_x = canvas_width * 1.1;
                    let max_offset_y = canvas_height * 1.1;

                    position.set((
                        (position.x() + dx).clamp(-max_offset_x, max_offset_x),
                        (position.y() + dy).clamp(-max_offset_y, max_offset_y),
                    ));
                })
            };

            || {
                drop(mouse_down_listener);
                drop(mouse_up_listener);
                drop(mouse_move_listener);
                drop(zoom_level_listener);
            }
        });
    }

    let canvas_height = f64::from(props.pixel_height * DIVISION);
    let canvas_width = f64::from(props.pixel_width * DIVISION);

    let window_width = window()
        .inner_width()
        .or(Err(GenericError::Undefined))
        .or_notify(&notification_hub)
        .as_f64()
        .ok_or(GenericError::DynCastError)
        .or_notify(&notification_hub);

    let window_height = window()
        .inner_height()
        .or(Err(GenericError::Undefined))
        .or_notify(&notification_hub)
        .as_f64()
        .ok_or(GenericError::DynCastError)
        .or_notify(&notification_hub);

    html! {
        <canvas
            ref={canvas_ref}

            id={props.id.clone()}
            class={classes!("canvas-main", &props.class)}

            style={
                format!(
                    r"
                        --scale: {}%;
                        --left: {}px;
                        --top: {}px;
                        --height: {}px;
                        --width: {}px;
                    ",
                    *zoom_level * 100.0,
                    (position.x() - canvas_width / 2.0) + window_width / 2.0,
                    (position.y() - canvas_height / 2.0) + window_height / 2.0,
                    canvas_height,
                    canvas_width
                )
            }
        />
    }
}
