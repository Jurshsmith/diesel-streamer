pub struct Counter {
    pub value: Box<u16>,
}

impl Counter {
    #[must_use]
    pub fn new(initial_value: u16) -> Self {
        Self {
            value: Box::new(initial_value),
        }
    }

    pub fn increment(&mut self) {
        *self.value += 1;
    }

    pub fn decrement(&mut self) {
        *self.value = std::cmp::max(0, *self.value - 1);
    }
}
