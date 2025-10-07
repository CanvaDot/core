use std::cell::RefCell;
use std::rc::Rc;

use bon::Builder;
use itertools::Itertools;
use yew::Callback;

use crate::utils::notifications::notification::Notification;
use crate::utils::types::{AtomicCallback, AtomicInRef, InRef};

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub enum NotificationComponentKind {
    #[default]
    Primary,
    Secondary,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NotificationComponent {
    RedirectButton(RedirectButton),
    ActionButton(ActionButton),
    Dropdown(Dropdown),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct RedirectButton {
    #[builder(default)]
    kind: NotificationComponentKind,
    #[builder(into)]
    text: String,
    #[builder(into)]
    target: String,
    #[builder(default = true)]
    enabled: bool,

    #[builder(into, default = "")]
    id: String,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct ActionButton {
    #[builder(default)]
    kind: NotificationComponentKind,
    #[builder(into)]
    text: String,
    #[builder(into, default = |_| {})]
    action: AtomicCallback<AtomicInRef<Notification>>,
    #[builder(default = true)]
    enabled: bool,

    #[builder(into, default = "")]
    id: String,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Dropdown {
    #[builder(field)]
    values: Vec<(String, String)>,
    #[builder(field)]
    current_value: String,

    #[builder(default = 0)]
    default: usize,
    #[builder(default = true)]
    enabled: bool,

    #[builder(into, default = |_| {})]
    on_change: AtomicCallback<AtomicInRef<Notification>>,

    #[builder(into, default = "")]
    id: String,
}

impl NotificationComponent {
    #[inline]
    pub fn id(&self) -> &str {
        match self {
            NotificationComponent::RedirectButton(button) => button.id(),
            NotificationComponent::ActionButton(button) => button.id(),
            NotificationComponent::Dropdown(dropdown) => dropdown.id(),
        }
    }
}

impl RedirectButton {
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn target(&self) -> &str {
        &self.target
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn kind(&self) -> NotificationComponentKind {
        self.kind
    }

    #[inline]
    pub fn set_kind(&mut self, kind: NotificationComponentKind) {
        self.kind = kind;
    }

    #[inline]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    #[inline]
    pub fn set_target(&mut self, redirect: String) {
        self.target = redirect;
    }

    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }
}

impl ActionButton {
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn action(&self) -> &AtomicCallback<AtomicInRef<Notification>> {
        &self.action
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn kind(&self) -> NotificationComponentKind {
        self.kind
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    #[inline]
    pub fn set_kind(&mut self, kind: NotificationComponentKind) {
        self.kind = kind;
    }
}

impl Dropdown {
    #[inline]
    pub fn values(&self) -> Vec<(String, String)> {
        self.values
            .clone()
    }

    #[inline]
    pub fn default(&self) -> usize {
        self.default
    }

    #[inline]
    pub fn on_change(&self) -> &AtomicCallback<AtomicInRef<Notification>> {
        &self.on_change
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn id(&self) -> &str {
        &self.id
    }

    #[inline]
    pub fn current_value(&self) -> &str {
        &self.current_value
    }

    #[inline]
    pub fn set_values(&mut self, values: Vec<(String, String)>) {
        self.values = values;
    }

    #[inline]
    pub fn set_default(&mut self, default: usize) {
        self.default = default;
    }

    #[inline]
    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn set_current_value(&mut self, value: String) {
        self.current_value = value;
    }
}

impl<S: dropdown_builder::State> DropdownBuilder<S> {
    pub fn add_value<K: ToString + ?Sized, V: ToString + ?Sized>(
        mut self,
        key: &K,
        value: &V,
    ) -> Self {
        self.values
            .push((key.to_string(), value.to_string()));
        self
    }
}

impl From<RedirectButton> for NotificationComponent {
    fn from(val: RedirectButton) -> Self {
        NotificationComponent::RedirectButton(val)
    }
}

impl From<ActionButton> for NotificationComponent {
    fn from(val: ActionButton) -> Self {
        NotificationComponent::ActionButton(val)
    }
}

impl From<Dropdown> for NotificationComponent {
    fn from(val: Dropdown) -> Self {
        NotificationComponent::Dropdown(val)
    }
}

pub fn group_components(components: &[NotificationComponent]) -> Vec<&[NotificationComponent]> {
    components
        .iter()
        .enumerate()
        .chunk_by(|(i, components)| match components {
            NotificationComponent::Dropdown(_) => format!("drop{i}"),
            | NotificationComponent::RedirectButton(_) | NotificationComponent::ActionButton(_) => {
                "button".to_string()
            },
        })
        .into_iter()
        .map(|(_, group)| {
            let indices: Vec<_> = group
                .map(|(i, _)| i)
                .collect();
            let start = *indices
                .first()
                .unwrap();
            let end = *indices
                .last()
                .unwrap()
                + 1;
            &components[start..end]
        })
        .collect()
}
