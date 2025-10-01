use bon::Builder;
use instant::{Duration, Instant};
use palette::Srgb;
use uuid::Uuid;
use yew::{Callback, Properties};

use crate::utils::colors::{ERROR_RED, INFO_BLUE, SUCCESS_GREEN};
use crate::utils::notifications::component::{
    ActionButton,
    Dropdown,
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

    #[builder(skip = Instant::now())]
    created_at: Instant,
    #[builder(default = Duration::from_secs(5))]
    duration: Duration,

    #[builder(skip = Uuid::new_v4())]
    id: Uuid,
    #[builder(into, default = |_| {})]
    on_close: Callback<()>,
}

impl Notification {
    pub fn id(&self) -> Uuid {
        self.id
    }

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

    pub fn hook_close(&mut self, callback: Callback<()>) {
        let orig_on_close = self
            .on_close
            .clone();

        self.on_close = Callback::from(move |_| {
            callback.emit(());
            orig_on_close.emit(());
        });
    }

    pub fn is_expired(&self) -> bool {
        self.created_at()
            .elapsed()
            > self.duration()
    }

    pub fn get_component(&self, id: &str) -> Option<&NotificationComponent> {
        self.components
            .iter()
            .find(|component| component.id() == id)
    }

    pub fn get_component_mut(&mut self, id: &str) -> Option<&mut NotificationComponent> {
        self.components
            .iter_mut()
            .find(|component| component.id() == id)
    }

    pub fn close(&self) {
        self.on_close
            .emit(());
    }

    #[inline]
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    #[inline]
    pub fn set_message(&mut self, message: String) {
        self.message = message;
    }

    #[inline]
    pub fn set_level(&mut self, level: NotificationLevel) {
        self.level = level;
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

    pub fn add_dropdown(mut self, dropdown: Dropdown) -> Self {
        self.components
            .push(dropdown.into());
        self
    }
}
