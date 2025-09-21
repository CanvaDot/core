use std::rc::Rc;

use palette::rgb::channels::Rgba;
use yew::{function_component, html, use_state, Callback, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::components::common::{AppButton, AppDropdown, ButtonTarget};
use crate::utils::notifications::component::{group_components, NotificationComponent, NotificationComponentKind};
use crate::utils::types::InRef;
use crate::utils::notifications::notification::Notification;
use crate::utils::colors::SECONDARY_GREY;


#[derive(Properties, PartialEq)]
pub struct NotificationProps {
    pub notification: InRef<Notification>
}

#[function_component(NotificationElement)]
pub fn notification_element(props: &NotificationProps) -> Html {
    let notif_borrow = props.notification.borrow();
    // NOTE: Maybe change this in a future.
    let re_render = use_state(|| true);

    let notif_component_groups = group_components(notif_borrow.components());
    let notif_components = notif_component_groups
        .iter()
        .map(|group| {
            let group_map = group
                .iter()
                .map(|component| match component {
                    NotificationComponent::RedirectButton(button) => html! {
                        <AppButton
                            target={ButtonTarget::Link(button.target().to_string())}
                            text={button.text().to_string()}
                            color={
                                match button.kind() {
                                    NotificationComponentKind::Primary
                                        => notif_borrow.level().to_color(),

                                    NotificationComponentKind::Secondary
                                        => SECONDARY_GREY,
                                }
                            }
                            enabled={button.enabled()}
                        />
                    },

                    NotificationComponent::ActionButton(button) => html! {
                        <AppButton
                            target={ButtonTarget::Callback(Callback::from({
                                let action = button.action().clone();
                                let notif_rc = Rc::clone(&props.notification);
                                let re_render = re_render.clone();
                                move |_| {
                                    action.emit(Rc::clone(&notif_rc));
                                    re_render.set(!*re_render);
                                }
                            }))}
                            text={button.text().to_string()}
                            color={
                                match button.kind() {
                                    NotificationComponentKind::Primary
                                        => notif_borrow.level().to_color(),

                                    NotificationComponentKind::Secondary
                                        => SECONDARY_GREY,
                                }
                            }
                            enabled={button.enabled()}
                        />
                    },

                    NotificationComponent::Dropdown(dropdown) => html! {
                        <AppDropdown
                            values={dropdown.values()}
                            default={dropdown.default()}
                            enabled={dropdown.enabled()}
                            onchange={Callback::from({
                                let dropdown = dropdown.clone();
                                let notif_rc = Rc::clone(&props.notification);
                                let re_render = re_render.clone();
                                move |value: String| {
                                    dropdown.set_current_value(value.clone());
                                    dropdown.on_change().emit(Rc::clone(&notif_rc));
                                    re_render.set(!*re_render);
                                }
                            })}
                        />
                    },
                });

            html! {
                <div class="notification-components-group">
                    {for group_map}
                </div>
            }
        });

    html! {
        <div
            class="notification"
            style={format!(
                "--accent-color: #{:08X}",
                notif_borrow
                    .level()
                    .to_color()
                    .into_u32::<Rgba>()
            )}
        >
            <Icon icon_id={IconId::FontAwesomeSolidX} />

            <div class="notification-content">
                <h1>{notif_borrow.title()}</h1>
                <p>{notif_borrow.message()}</p>
            </div>

            <div class="notification-components">
                {for notif_components}
            </div>
        </div>
    }
}
