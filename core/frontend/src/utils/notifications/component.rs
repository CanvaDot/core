use std::{cell::RefCell, rc::Rc};

use bon::Builder;
use itertools::Itertools;
use yew::Callback;

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
    DropDown(DropDown),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct RedirectButton {
    #[builder(field)]
    kind: NotificationComponentKind,
    #[builder(into)]
    text: String,
    #[builder(into)]
    redirect: String,
    #[builder(default = true)]
    enabled: bool,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct ActionButton {
    #[builder(field)]
    kind: NotificationComponentKind,
    #[builder(into)]
    text: String,
    #[builder(into)]
    action: Callback<Rc<RefCell<Self>>>,
    #[builder(default = true)]
    enabled: bool,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct DropDown {
    #[builder(field)]
    values: Vec<String>,
    #[builder(default = 0)]
    default: usize,
    #[builder(default = true)]
    enabled: bool,

    onchange: Callback<Rc<RefCell<Self>>>,
}

impl RedirectButton {
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn redirect(&self) -> &str {
        &self.redirect
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
    pub fn set_kind(&mut self, kind: NotificationComponentKind) {
        self.kind = kind;
    }

    #[inline]
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }

    #[inline]
    pub fn set_redirect(&mut self, redirect: String) {
        self.redirect = redirect;
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
    pub fn action(&self) -> &Callback<Rc<RefCell<Self>>> {
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

impl DropDown {
    #[inline]
    pub fn values(&self) -> &[String] {
        &self.values
    }

    #[inline]
    pub fn default(&self) -> usize {
        self.default
    }

    #[inline]
    pub fn onchange(&self) -> &Callback<Rc<RefCell<Self>>> {
        &self.onchange
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    #[inline]
    pub fn set_values(&mut self, values: Vec<String>) {
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
}

impl<S: drop_down_builder::State> DropDownBuilder<S> {
    pub fn add_value(mut self, value: impl ToString) -> Self {
        self.values
            .push(value.to_string());
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

impl Into<NotificationComponent> for DropDown {
    fn into(self) -> NotificationComponent {
        NotificationComponent::DropDown(self)
    }
}

pub fn group_components(
    components: &[NotificationComponent]
) -> Vec<&[NotificationComponent]> {
    components
        .iter()
        .enumerate()
        .chunk_by(|(_, components)| match components {
            NotificationComponent::DropDown(_) => "drop",
            | NotificationComponent::RedirectButton(_)
            | NotificationComponent::ActionButton(_) => "button"
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
