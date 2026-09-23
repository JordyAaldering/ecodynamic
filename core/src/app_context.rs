use std::collections::HashMap;

use ecodynamic_api::{Demand, Request, Sample};

use crate::{Controller, Letterbox};

/// Each application context is a mapping from a unique identifier of the 'task' that is being controller
/// to a `Letterbox` that tracks the recent samples for that task, and the controller that is managing that task.
/// The letterbox is updated every sample, whereas the controller is typically evolved when the letterbox is full.
pub struct ApplicationContext<C, F>
where
    C: Controller,
    F: Fn() -> C,
{
    letterbox_size: usize,
    build_controller: F,
    context: HashMap<i32, (Letterbox, C)>
}

impl<C, F> ApplicationContext<C, F>
where
    C: Controller,
    F: Fn() -> C,
{
    pub fn new(letterbox_size: usize, build_controller: F) -> Self {
        Self {
            letterbox_size,
            build_controller,
            context: HashMap::new(),
        }
    }

    pub fn current(&mut self, request: &Request) -> (usize, &mut C) {
        let (letterbox, controller) = self.context.get_mut(&request.task_id).unwrap();
        (letterbox.len(), controller)
    }

    pub fn request(&mut self, request: &Request) -> Demand {
        let (letterbox, controller) = self.context.entry(request.task_id)
            .or_insert_with(|| {
                log::debug!("Instantiating task {}", request.task_id);
                let letterbox = Letterbox::new(self.letterbox_size);
                let controller = (self.build_controller)();
                (letterbox, controller)
            });
        let index = letterbox.len();
        controller.request(index)
    }

    pub fn push(&mut self, sample: Sample) {
        let (letterbox, controller) = self.context.get_mut(&sample.task_id)
            .expect("Received sample for a task that has not yet been instantiated");
        if let Some(samples) = letterbox.push(sample) {
            controller.evolve(samples);
        }
    }
}
