use std::{cell::RefCell, rc::Rc};

use bon::Builder;
use itertools::Itertools;
use yew::Callback;

use crate::utils::{notifications::notification::Notification, types::InRef};

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
    id: String
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct ActionButton {
    #[builder(default)]
    kind: NotificationComponentKind,
    #[builder(into)]
    text: String,
    #[builder(into, default = |_| {})]
    action: Callback<InRef<Notification>>,
    #[builder(default = true)]
    enabled: bool,

    #[builder(into, default = "")]
    id: String
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct Dropdown {
    #[builder(field)]
    values: Vec<(String, String)>,
    #[builder(field)]
    current_value: InRef<String>,

    #[builder(default = 0)]
    default: usize,
    #[builder(default = true)]
    enabled: bool,

    #[builder(into, default = |_| {})]
    on_change: Callback<InRef<Notification>>,

    #[builder(into, default = "")]
    id: String
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
    pub fn action(&self) -> &Callback<InRef<Notification>> {
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
        self.values.clone()
    }

    #[inline]
    pub fn default(&self) -> usize {
        self.default
    }

    #[inline]
    pub fn on_change(&self) -> &Callback<InRef<Notification>> {
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
    pub fn current_value(&self) -> InRef<String> {
        self.current_value.clone()
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

    pub fn set_current_value(&self, value: String) {
        *self.current_value.borrow_mut() = value;
    }
}

impl<S: dropdown_builder::State> DropdownBuilder<S> {
    pub fn add_value(mut self, key: impl ToString, value: impl ToString) -> Self {
        self.values
            .push((key.to_string(), value.to_string()));
        self
    }
}

impl Into<NotificationComponent> for RedirectButton {
    fn into(self) -> NotificationComponent {
        NotificationComponent::RedirectButton(self)
    }
}

impl Into<NotificationComponent> for ActionButton {
    fn into(self) -> NotificationComponent {
        NotificationComponent::ActionButton(self)
    }
}

impl Into<NotificationComponent> for Dropdown {
    fn into(self) -> NotificationComponent {
        NotificationComponent::Dropdown(self)
    }
}

pub fn group_components(
    components: &[NotificationComponent]
) -> Vec<&[NotificationComponent]> {
    components
        .iter()
        .enumerate()
        .chunk_by(|(i, components)| match components {
            NotificationComponent::Dropdown(_) => format!("drop{i}"),
            | NotificationComponent::RedirectButton(_)
            | NotificationComponent::ActionButton(_) => "button".to_string()
        })
        .into_iter()
        .map(|(_, group)| {
            let indices: Vec<_> = group.map(|(i, _)| i).collect();
            let start = *indices.first().unwrap();
            let end = *indices.last().unwrap() + 1;
            &components[start..end]
        })
        .collect()
}
