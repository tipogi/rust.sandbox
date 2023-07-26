use std::{fmt::Display, cell::RefCell};
use notify_rust::{Hint, Urgency, Notification};
use serde::{Serialize, Deserialize};
use zbus::export::futures_util::FutureExt;

use crate::status::Status;

pub enum Work {
    Done,
    Fail,
    UnKnown
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: u16,
    pub title: String,
    pub body: String,
    pub icon: String
}

impl Task {
    pub fn new(id: u16, title: String, body: String, icon: String) -> Self {
        Self {
            id,
            title,
            body,
            icon
        }
    }

    pub async fn chech_task_done(&self, daily_task: &RefCell<Vec<Status>>) {
        let title = format!("{} {}", self.icon, self.title);
        Notification::new()
            .summary(&title)
            .hint(Hint::Urgency(Urgency::Critical))
            .body(&self.body)
            .action("done", "✔")
            .action("fail", "✗")
            .show_async()
            .then(|handle| async move {
                match handle {
                    Ok(handle) => handle.wait_for_action(| action_identifier | {
                        let completed = match action_identifier {
                            "done"  => true,
                            _       => false
                        };
                        let task_status = Status::new(
                            completed,
                            self.id
                        );
                        daily_task.borrow_mut().push(task_status);
                    }),
                    Err(error) => println!("failed to send notification {error}"),
                };
            }).await;
    }

}

// Implement the trait for a better output of the object
impl Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {}: {}", self.icon,self.title, self.body)
    }
}
