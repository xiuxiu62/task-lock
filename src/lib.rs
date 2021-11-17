use std::{
    any::Any,
    sync::{Arc, RwLock},
    thread,
};

type TaskError = Box<dyn Any + Send>;
type TaskResult<T> = Result<T, TaskError>;
type Task<T> = &'static (dyn Fn(Arc<TaskLock<T>>) -> TaskResult<()> + Sync);

#[derive(Clone, Debug)]
pub struct TaskLock<T>
where
    T: Clone,
{
    inner: Arc<RwLock<T>>,
    task_alive: u32,
    task_complete: u32,
}

impl<T> TaskLock<T>
where
    T: Clone,
{
    pub fn new(i: T) -> Self {
        Self {
            inner: Arc::new(RwLock::new(i)),
            task_alive: 0,
            task_complete: 0,
        }
    }

    pub fn dispatch(&mut self, task: Task<T>) -> TaskResult<()>
    where
        T: Send + Sync,
    {
        self.task_alive += 1;

        // let lock = Arc::clone(&self.inner);
        // let count = self.count();
        let handle = thread::spawn(move || task(lock));

        self.task_complete += 1;
        self.task_alive -= 1;

        match handle.join() {
            Err(e) => Err(e),
            Ok(_) => Ok(()),
        }
    }

    pub fn count(&self) -> u32 {
        self.task_alive
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() -> TaskResult<()> {
        let mut lock = TaskLock::new(0_u32);

        for i in 1..=5 {
            lock.dispatch(&|lock| {
                let temp = &i.clone();
                println!("thread {} {{ inner: {:?} }}", count, lock);
                std::thread::sleep(std::time::Duration::from_millis(500));
                Ok(())
            })?
        }

        Ok(())
    }
}
