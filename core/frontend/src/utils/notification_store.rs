use std::{cell::RefCell, error::Error, rc::Rc, time::{Duration, Instant}};

use yew::Callback;


#[derive(Clone, Copy, PartialEq)]
pub enum NotificationLevel {
    Info,
    Success,
    Error
}

#[derive(Clone, Copy, PartialEq)]
pub enum NotificationComponentType {
    Primary,
    Secondary
}

#[derive(Clone, PartialEq)]
pub enum NotificationComponent {
    RedirectButton {
        text: String,
        redirect: String,
        enabled: bool,
        kind: NotificationComponentType
    },

    ActionButton {
        text: String,
        action: Callback<()>,
        enabled: bool,
        kind: NotificationComponentType
    },

    DropDown {
        default: Option<usize>,
        values: Vec<String>,
        on_change: Callback<String>
    }
}

#[derive(Clone, PartialEq)]
pub struct Notification {
    title: String,
    message: String,
    level: Option<NotificationLevel>,
    components: Vec<NotificationComponent>,

    created_at: Instant,
    duration: Option<Duration>
}

#[derive(Default, PartialEq)]
pub struct NotificationStore {
    notifications: Rc<RefCell<Vec<Notification>>>
}

impl Notification {
    pub fn new(title: impl ToString, message: impl ToString) -> Self {
        Self {
            title: title.to_string(),
            message: message.to_string(),
            level: None,
            components: Vec::new(),

            created_at: Instant::now(),
            duration: None
        }
    }

    pub fn is_expired(&self) -> bool {
        self.created_at.elapsed() >= self.duration()
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn level(&self) -> NotificationLevel {
        self.level
            .unwrap_or(NotificationLevel::Info)
    }

    pub fn components(&self) -> &[NotificationComponent] {
        &self.components
    }

    pub fn created_at(&self) -> &Instant {
        &self.created_at
    }

    pub fn duration(&self) -> Duration {
        self.duration
            .unwrap_or(Duration::from_secs(5))
    }

    pub fn set_level(&mut self, level: NotificationLevel) -> &mut Self {
        self.level = Some(level);
        self
    }

    pub fn set_components(&mut self, components: Vec<NotificationComponent>) -> &mut Self {
        self.components = components;
        self
    }

    pub fn set_created_at(&mut self, created_at: Instant) -> &mut Self {
        self.created_at = created_at;
        self
    }

    pub fn set_duration(&mut self, duration: Duration) -> &mut Self {
        self.duration = Some(duration);
        self
    }
}

impl NotificationStore {
    pub fn add(&self, notification: Notification) {
        self.notifications
            .borrow_mut()
            .push(notification)
    }

    pub fn remove_expired(&self) {
        self.notifications
            .borrow_mut()
            .retain(|notification| !notification.is_expired());
    }
}
