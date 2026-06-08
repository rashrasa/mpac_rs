use log::warn;

pub struct CompletionGuard {
    descriptor: String,
    completed: bool,
}

impl CompletionGuard {
    pub fn new(descriptor: String) -> Self {
        Self {
            descriptor,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        if self.completed {
            warn!("{} has already been completed", self.descriptor);
        }
        self.completed = true;
    }
}

impl Drop for CompletionGuard {
    fn drop(&mut self) {
        if !self.completed {
            warn!("{} was dropped without completion", self.descriptor);
        }
    }
}

impl std::fmt::Debug for CompletionGuard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.completed {
            write!(f, "{}: completed", self.descriptor)
        } else {
            write!(f, "{}: not completed", self.descriptor)
        }
    }
}
