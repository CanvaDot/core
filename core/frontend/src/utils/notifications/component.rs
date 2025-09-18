use bon::Builder;
use yew::Callback;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NotificationComponentKind {
    Primary,
    Secondary,
}

#[derive(Clone, PartialEq, Debug)]
pub enum NotificationComponent<'a> {
    RedirectButton(RedirectButton),
    ActionButton(ActionButton<'a>),
    DropDown(DropDown<'a>),
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct RedirectButton {
    text: String,
    redirect: String,
    #[builder(default = true)]
    enabled: bool,
    #[builder(default = NotificationComponentKind::Primary)]
    kind: NotificationComponentKind,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct ActionButton<'a> {
    text: String,
    action: Callback<&'a mut Self>,
    #[builder(default = true)]
    enabled: bool,
    #[builder(default = NotificationComponentKind::Primary)]
    kind: NotificationComponentKind,
}

#[derive(Builder, Clone, Debug, PartialEq)]
pub struct DropDown<'a> {
    #[builder(field)]
    values: Vec<String>,
    #[builder(default = 0)]
    default: usize,

    onchange: Callback<&'a mut Self>,
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
}

impl<'a> ActionButton<'a> {
    #[inline]
    pub fn text(&self) -> &str {
        &self.text
    }

    #[inline]
    pub fn action(&self) -> &Callback<&'a mut Self> {
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
}

impl<'a> DropDown<'a> {
    #[inline]
    pub fn values(&self) -> &[String] {
        &self.values
    }

    #[inline]
    pub fn default(&self) -> usize {
        self.default
    }

    #[inline]
    pub fn onchange(&self) -> &Callback<&'a mut Self> {
        &self.onchange
    }
}

impl<'a, S: drop_down_builder::State> DropDownBuilder<'a, S> {
    pub fn add_value(mut self, value: impl ToString) -> Self {
        self.values
            .push(value.to_string());
        self
    }
}

impl<'c> Into<NotificationComponent<'c>> for RedirectButton {
    fn into(self) -> NotificationComponent<'c> {
        NotificationComponent::RedirectButton(self)
    }
}

impl<'c> Into<NotificationComponent<'c>> for ActionButton<'c> {
    fn into(self) -> NotificationComponent<'c> {
        NotificationComponent::ActionButton(self)
    }
}

impl<'c> Into<NotificationComponent<'c>> for DropDown<'c> {
    fn into(self) -> NotificationComponent<'c> {
        NotificationComponent::DropDown(self)
    }
}
