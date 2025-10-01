use std::cell::RefCell;
use std::rc::Rc;

use instant::Duration;
use tokio::test as async_test;
use tokio::time::sleep;
use uuid::Version as UuidVersion;
use yew::Callback;

use crate::utils::colors::{ERROR_RED, INFO_BLUE, SUCCESS_GREEN};
use crate::utils::notifications::component::{
    group_components,
    ActionButton,
    Dropdown,
    NotificationComponent,
    NotificationComponentKind,
    RedirectButton,
};
use crate::utils::notifications::notification::{Notification, NotificationLevel};
use crate::utils::notifications::store::NotificationStore;
use crate::utils::types::InRef;

#[async_test]
async fn test_notification_store() {
    let store = NotificationStore::default();

    store.add(
        Notification::builder()
            .title("test")
            .message("test")
            .duration(Duration::from_secs(1))
            .build(),
    );

    assert_eq!(
        store
            .all()
            .borrow()
            .len(),
        1
    );

    sleep(Duration::from_secs(2)).await;

    store.remove_expired();

    assert_eq!(
        store
            .all()
            .borrow()
            .len(),
        0
    );

    let notification = Notification::builder()
        .title("test")
        .message("test")
        .build();

    let notification_id = notification
        .id()
        .clone();

    store.add(notification);

    assert_eq!(
        store
            .all()
            .borrow()
            .len(),
        1
    );

    store.remove_by_id(notification_id);

    assert_eq!(
        store
            .all()
            .borrow()
            .len(),
        0
    );
}

#[test]
fn test_notification_builders() {
    let notification = Notification::builder()
        .title("test")
        .message("test")
        .level(NotificationLevel::Info)
        .duration(Duration::from_secs(5))
        .add_redirect_button(
            RedirectButton::builder()
                .text("test")
                .id("test_redirect_button")
                .target("/")
                .kind(NotificationComponentKind::Primary)
                .enabled(false)
                .build(),
        )
        .add_action_button(
            ActionButton::builder()
                .text("test")
                .id("test_action_button")
                .action(|notification: InRef<Notification>| {
                    notification
                        .borrow_mut()
                        .set_title("test 2".into());
                })
                .kind(NotificationComponentKind::Secondary)
                .enabled(true)
                .build(),
        )
        .add_dropdown(
            Dropdown::builder()
                .id("test_dropdown")
                .default(0)
                .add_value("test", "test")
                .on_change(|notification: InRef<Notification>| {
                    let notification = notification.borrow();

                    if let Some(NotificationComponent::Dropdown(dropdown)) =
                        notification.get_component("test_dropdown")
                    {
                        assert_eq!(
                            *dropdown
                                .current_value()
                                .borrow(),
                            "test"
                        );
                    } else {
                        unreachable!("The dropdown does not exist or is not a Dropdown.")
                    }
                })
                .enabled(true)
                .build(),
        )
        .build();

    let notification = Rc::new(RefCell::new(notification));

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        assert_eq!(
            notification_borrow
                .id()
                .get_version(),
            Some(UuidVersion::Random)
        );
        assert_eq!(notification_borrow.level(), NotificationLevel::Info);
        assert_eq!(notification_borrow.title(), "test");
        assert_eq!(notification_borrow.message(), "test");
        assert_eq!(notification_borrow.duration(), Duration::from_secs(5));
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        let redirect_button = notification_borrow
            .get_component("test_redirect_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::RedirectButton(button) = redirect_button {
            assert_eq!(button.id(), "test_redirect_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.target(), "/");
            assert_eq!(button.kind(), NotificationComponentKind::Primary);
            assert!(!button.enabled())
        } else {
            unreachable!("Redirect Button is not a Redirect Button.");
        }
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow_mut();

        let action_button = notification_borrow
            .get_component("test_action_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::ActionButton(button) = action_button {
            assert_eq!(button.id(), "test_action_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.kind(), NotificationComponentKind::Secondary);
            assert!(button.enabled());
            let action = button
                .action()
                .clone();

            drop(notification_borrow);

            action.emit(notification.clone());

            let notification_borrow = notification.borrow();

            assert_eq!(notification_borrow.title(), "test 2");
        } else {
            unreachable!("Action Button is not an Action Button.");
        }
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        let dropdown = notification_borrow
            .get_component("test_dropdown")
            .expect("Test Dropdown To Exist.");

        if let NotificationComponent::Dropdown(dropdown) = dropdown {
            assert_eq!(dropdown.id(), "test_dropdown");
            assert_eq!(
                dropdown
                    .values()
                    .len(),
                1
            );
            assert_eq!(dropdown.default(), 0);
            assert!(dropdown.enabled());

            dropdown.set_current_value("test".into());
            dropdown
                .on_change()
                .emit(notification.clone());
        } else {
            unreachable!("Dropdown is not a Dropdown");
        }
    }
}

#[test]
fn test_default_builder_options() {
    let notification = Notification::builder()
        .title("test")
        .message("test")
        .add_redirect_button(
            RedirectButton::builder()
                .text("test")
                .id("test_redirect_button")
                .target("/")
                .build(),
        )
        .add_action_button(
            ActionButton::builder()
                .text("test")
                .id("test_action_button")
                .build(),
        )
        .add_dropdown(
            Dropdown::builder()
                .id("test_dropdown")
                .build(),
        )
        .build();

    let notification = Rc::new(RefCell::new(notification));

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        assert_eq!(
            notification_borrow
                .id()
                .get_version(),
            Some(UuidVersion::Random)
        );
        assert_eq!(notification_borrow.level(), NotificationLevel::Info);
        assert_eq!(notification_borrow.title(), "test");
        assert_eq!(notification_borrow.message(), "test");
        assert_eq!(notification_borrow.duration(), Duration::from_secs(5));
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        let redirect_button = notification_borrow
            .get_component("test_redirect_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::RedirectButton(button) = redirect_button {
            assert_eq!(button.id(), "test_redirect_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.target(), "/");
            assert_eq!(button.kind(), NotificationComponentKind::Primary);
            assert!(button.enabled())
        } else {
            unreachable!("Redirect Button is not a Redirect Button.");
        }
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow_mut();

        let action_button = notification_borrow
            .get_component("test_action_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::ActionButton(button) = action_button {
            let action = button
                .action()
                .clone();
            action.emit(notification.clone()); // nothing should change in this case.

            assert_eq!(button.id(), "test_action_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.kind(), NotificationComponentKind::Primary);
            assert!(button.enabled());
        } else {
            unreachable!("Action Button is not an Action Button.");
        }
    }

    {
        let notification = notification.clone();
        let notification_borrow = notification.borrow();

        let dropdown = notification_borrow
            .get_component("test_dropdown")
            .expect("Test Dropdown To Exist.");

        if let NotificationComponent::Dropdown(dropdown) = dropdown {
            assert_eq!(dropdown.id(), "test_dropdown");
            assert_eq!(
                dropdown
                    .values()
                    .len(),
                0
            );
            assert_eq!(dropdown.default(), 0);
            assert!(dropdown.enabled());
        } else {
            unreachable!("Dropdown is not a Dropdown");
        }
    }
}

#[test]
fn test_setters() {
    let notification = Notification::builder()
        .title("")
        .message("")
        .duration(Duration::from_secs(5)) // immutable
        .add_redirect_button(
            RedirectButton::builder()
                .text("")
                .id("test_redirect_button") // immutable
                .target("")
                .build()
        )
        .add_action_button(
            ActionButton::builder()
                .text("")
                .id("test_action_button") // immutable
                .build()
        )
        .add_dropdown(
            Dropdown::builder()
                .id("test_dropdown") // immutable
                .build()
        )
        .build();

    let notification = Rc::new(RefCell::new(notification));

    {
        let notification = notification.clone();
        let mut notification_borrow = notification.borrow_mut();

        notification_borrow.set_title("test".into());
        notification_borrow.set_message("test".into());
        notification_borrow.set_level(NotificationLevel::Info);

        assert_eq!(
            notification_borrow
                .id()
                .get_version(),
            Some(UuidVersion::Random)
        );
        assert_eq!(notification_borrow.level(), NotificationLevel::Info);
        assert_eq!(notification_borrow.title(), "test");
        assert_eq!(notification_borrow.message(), "test");
        assert_eq!(notification_borrow.duration(), Duration::from_secs(5));
    }

    {
        let notification = notification.clone();
        let mut notification_borrow = notification.borrow_mut();

        let redirect_button = notification_borrow
            .get_component_mut("test_redirect_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::RedirectButton(button) = redirect_button {
            button.set_text("test".into());
            button.set_target("/".into());
            button.set_kind(NotificationComponentKind::Secondary);
            button.set_enabled(false);

            assert_eq!(button.id(), "test_redirect_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.target(), "/");
            assert_eq!(button.kind(), NotificationComponentKind::Secondary);
            assert!(!button.enabled())
        } else {
            unreachable!("Redirect Button is not a Redirect Button.");
        }
    }

    {
        let notification = notification.clone();
        let mut notification_borrow = notification.borrow_mut();

        let action_button = notification_borrow
            .get_component_mut("test_action_button")
            .expect("Test Button to Exist.");

        if let NotificationComponent::ActionButton(button) = action_button {
            button.set_text("test".into());
            button.set_kind(NotificationComponentKind::Secondary);
            button.set_enabled(true);

            assert_eq!(button.id(), "test_action_button");
            assert_eq!(button.text(), "test");
            assert_eq!(button.kind(), NotificationComponentKind::Secondary);
            assert!(button.enabled());
        } else {
            unreachable!("Action Button is not an Action Button.");
        }
    }

    {
        let notification = notification.clone();
        let mut notification_borrow = notification.borrow_mut();

        let dropdown = notification_borrow
            .get_component_mut("test_dropdown")
            .expect("Test Dropdown To Exist.");

        if let NotificationComponent::Dropdown(dropdown) = dropdown {
            dropdown.set_enabled(true);
            dropdown.set_values(vec![("test".into(), "test".into())]);
            dropdown.set_default(0);
            dropdown.set_enabled(true);

            assert_eq!(dropdown.id(), "test_dropdown");
            assert_eq!(
                dropdown
                    .values()
                    .len(),
                1
            );
            assert_eq!(dropdown.default(), 0);
            assert!(dropdown.enabled());
        } else {
            unreachable!("Dropdown is not a Dropdown");
        }
    }
}

#[test]
fn test_group_components() {
    let notification = Notification::builder()
        .title("test")
        .message("test")
        .add_redirect_button(
            RedirectButton::builder()
                .text("test")
                .target("/")
                .build(),
        )
        .add_redirect_button(
            RedirectButton::builder()
                .text("test2")
                .target("/")
                .build(),
        )
        .add_dropdown(Dropdown::builder().build())
        .add_action_button(
            ActionButton::builder()
                .text("test3")
                .build(),
        )
        .build();

    let grouped_components = group_components(notification.components());

    let first_row = grouped_components
        .get(0)
        .expect("For a first two button row to exist.");

    assert!(matches!(first_row.get(0), Some(NotificationComponent::RedirectButton(_))));
    assert!(matches!(first_row.get(1), Some(NotificationComponent::RedirectButton(_))));

    let second_row = grouped_components
        .get(1)
        .expect("For the dropdown row to exist.");

    assert!(matches!(second_row.get(0), Some(NotificationComponent::Dropdown(_))));

    let third_row = grouped_components
        .get(2)
        .expect("For the third single action button row to exist.");

    assert!(matches!(third_row.get(0), Some(NotificationComponent::ActionButton(_))));
}

#[test]
fn notification_level_colors() {
    assert_eq!(NotificationLevel::Info.to_color(), INFO_BLUE);
    assert_eq!(NotificationLevel::Success.to_color(), SUCCESS_GREEN);
    assert_eq!(NotificationLevel::Error.to_color(), ERROR_RED);
}

#[test]
fn notification_close_hooks() {
    let flag = Rc::new(RefCell::new(false));

    let mut notification = Notification::builder()
        .title("test")
        .message("test")
        .build();

    {
        let flag = flag.clone();
        notification.hook_close(Callback::from(move |_| {
            *flag.borrow_mut() = true;
        }));
    }

    notification.close();
    assert!(*flag.borrow());
}
