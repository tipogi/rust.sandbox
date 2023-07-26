use std::cell::RefCell;

use todo_notifier::{file::File, status::Status};
use zbus;


fn main() {
    let daily_tasks:RefCell<Vec<Status>> = RefCell::new(Vec::new());
    let task_collection = File::read_task_file();
    for task in task_collection.iter() {
        zbus::block_on(async {
            task.chech_task_done(&daily_tasks).await;
        });
    }
    File::save_daily_tasks(&daily_tasks);
}
