//! Signal Implement

use super:: {
    task::Task,
};

/// Signal Type
pub enum Signal {
    NewTask(Task),
    Terminate,
}