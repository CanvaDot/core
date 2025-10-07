use std::sync::{Condvar, Mutex, OnceLock};
use std::sync::mpsc::{channel, Receiver, Sender};


use crate::utils::notifications::notification::Notification;

struct NotificationReadyState {
    ready: Mutex<bool>,
    condvar: Condvar
}

static READY: NotificationReadyState = NotificationReadyState {
    ready: Mutex::new(false),
    condvar: Condvar::new()
};

static SENDER: OnceLock<Sender<Notification>> = OnceLock::new();

pub async fn start_listening_notifications() -> Option<Receiver<Notification>> {
    let (tx, rx) = channel();

    if SENDER.set(tx).is_ok() {
        let mut ready = READY.ready.lock()
            .expect("Notification listener to have an unpoisoned mutex available.");
        *ready = true;
        READY.condvar.notify_all();
        Some(rx)
    } else {
        None
    }
}

pub fn send_notification(notification: Notification) {
    let mut ready = READY.ready.lock()
        .expect("Notification listener to have an unpoisoned mutex available.");
    while !*ready {
        ready = READY.condvar.wait(ready)
            .expect("Notification listener to have an unpoisoned mutex available.");
    }

    if let Some(sender) = SENDER.get() {
        let _ = sender.send(notification);
    }
}
