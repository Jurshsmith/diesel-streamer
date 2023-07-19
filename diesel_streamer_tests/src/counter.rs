pub struct Counter {
    pub value: Box<u16>,
}

impl Counter {
    pub fn new(initial_value: u16) -> Self {
        Self {
            value: Box::new(initial_value),
        }
    }

    pub fn increment(&self) -> Self {
        Self {
            value: Box::new(*self.value + 1),
        }
    }

    pub fn decrement(&self) -> Self {
        Self {
            value: Box::new(std::cmp::max(0, *self.value - 1)),
        }
    }
}
