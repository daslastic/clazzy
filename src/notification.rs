use crate::{Clazzy, ProgramError};
use notify_rust::Notification;

pub fn send_messege(clazzy: &mut Clazzy, title: String, msg: String) {
    let mut notification = Notification::new().summary(&title).body(&msg).clone();
    if let Some(name) = &clazzy.clazz.notify_sound {
        notification.sound_name(name);
    }
    clazzy.notifications.push(notification);
}

pub fn send_class_messege(clazzy: &mut Clazzy, msg: String) {
    if let Some(current_class) = &clazzy.current_class {
        send_messege(clazzy, current_class.class.name.clone(), msg);
    }
}

pub fn process_messege(clazzy: &mut Clazzy) {
    if !clazzy.notifications.is_empty() {
        let notification = &clazzy.notifications[0];
        if let Err(e) = notification.show() {
            crate::error::runtime_error(ProgramError::Notify(e));
        }
        clazzy.notifications.remove(0);
    }
}
