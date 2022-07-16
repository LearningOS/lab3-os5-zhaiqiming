//! Implementation of [`TaskManager`]
//!
//! It is only used to manage processes and schedule process based on ready queue.
//! Other CPU process monitoring functions are in Processor.


use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use crate::config::BIG_STRIDE;
use lazy_static::*;

pub struct TaskManager {
    ready_queue: VecDeque<Arc<TaskControlBlock>>,
    // ready_queue: BinaryHeap<Arc<TaskControlBlock>>,
}

// YOUR JOB: FIFO->Stride
/// A simple FIFO scheduler.
impl TaskManager {
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push_back(task);
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        // return self.ready_queue.pop_front();
        if self.ready_queue.is_empty() {
            return None;
        }
        let mut min_stride = self.ready_queue.get(0 as usize).unwrap().inner_exclusive_access().stride;
        for task in self.ready_queue.iter() {
            let inner = task.inner_exclusive_access();
            // min_stride = min_stride.min(inner.stride);
            if ((inner.stride - min_stride) as i8) < 0 {
                min_stride = inner.stride;
            }
        }
        let mut index = 0;
        for (i, task) in self.ready_queue.iter().enumerate() {
            let inner = task.inner_exclusive_access();
            if min_stride == inner.stride {
                index = i;
                break;
            }
        }
        {
            let mut queue = &mut self.ready_queue;
            let mut inner = queue.get(index).unwrap().inner_exclusive_access();
            // println!("{} - {} - {}", queue.get(index).unwrap().pid.0, inner.stride, inner.priority);
        }
        return self.ready_queue.swap_remove_back(index);

    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

pub fn add_task(task: Arc<TaskControlBlock>) {
    TASK_MANAGER.exclusive_access().add(task);
}

pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    TASK_MANAGER.exclusive_access().fetch()
}
