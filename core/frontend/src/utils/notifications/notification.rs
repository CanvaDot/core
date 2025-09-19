use bon::Builder;
use instant::{Duration, Instant};
use palette::Srgb;
use yew::Properties;

use crate::utils::colors::{ERROR_RED, INFO_BLUE, SUCCESS_GREEN};
use crate::utils::notifications::component::{
    ActionButton,
    DropDown,
    NotificationComponent,
    RedirectButton,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum NotificationLevel {
    Info,
    Success,
    Error,
}

#[derive(Builder, Clone, PartialEq, Debug)]
pub struct Notification {
    #[builder(field)]
    components: Vec<NotificationComponent>,

    #[builder(into)]
    title: String,
    #[builder(into)]
    message: String,
    #[builder(default = NotificationLevel::Info)]
    level: NotificationLevel,

    #[builder(default = Instant::now())]
    created_at: Instant,
    #[builder(default = Duration::from_secs(5))]
    duration: Duration,
}

impl Notification {
    #[inline]
    pub fn components(&self) -> &[NotificationComponent] {
        &self.components
    }

    #[inline]
    pub fn title(&self) -> &str {
        &self.title
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }

    #[inline]
    pub fn level(&self) -> NotificationLevel {
        self.level
    }

    #[inline]
    pub fn created_at(&self) -> Instant {
        self.created_at
    }

    #[inline]
    pub fn duration(&self) -> Duration {
        self.duration
    }

    pub fn is_expired(&self) -> bool {
        self.created_at()
            .elapsed()
            > self.duration()
    }
}

impl NotificationLevel {
    #[inline]
    pub const fn to_color(&self) -> Srgb<u8> {
        match self {
            NotificationLevel::Info => INFO_BLUE,
            NotificationLevel::Success => SUCCESS_GREEN,
            NotificationLevel::Error => ERROR_RED,
        }
    }
}

impl<S: notification_builder::State> NotificationBuilder<S> {
    pub fn add_redirect_button(mut self, button: RedirectButton) -> Self {
        self.components
            .push(button.into());
        self
    }

    pub fn add_action_button(mut self, button: ActionButton) -> Self {
        self.components
            .push(button.into());
        self
    }

    pub fn add_dropdown(mut self, dropdown: DropDown) -> Self {
        self.components
            .push(dropdown.into());
        self
    }
}
