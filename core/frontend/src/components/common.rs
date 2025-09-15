use gloo::utils::window;
use yew::prelude::*;
use palette::Srgb;


#[derive(PartialEq, Clone)]
pub enum ButtonTarget {
    Callback(Callback<MouseEvent>),
    Link(String)
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub target: ButtonTarget,
    pub text: String,
    pub color: Srgb<u8>,

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

    html! {
        <button onclick={on_click}>
            {&props.text}
        </button>
    }
}
