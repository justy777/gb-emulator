#[derive(Debug, Copy, Clone)]
pub struct Delay<T> {
    value: T,
    delayed_value: Option<T>,
    counter: Option<u8>,
}

impl<T> Delay<T> {
    pub const fn new(value: T) -> Self {
        Self {
            value,
            delayed_value: None,
            counter: None,
        }
    }

    pub const fn get(&self) -> &T {
        &self.value
    }

    pub fn get_and_advance(&mut self) -> &T {
        if self.counter == Some(0) {
            self.value = self.delayed_value.take().unwrap();
            self.counter = None;
        }
        self.counter = self.counter.map(|count| count - 1);
        &self.value
    }

    pub fn set_delay(&mut self, delayed_value: T, counter: u8) {
        self.delayed_value = Some(delayed_value);
        self.counter = Some(counter);
    }

    pub fn clear_delay(&mut self) {
        self.delayed_value = None;
        self.counter = None;
    }

    pub fn into_inner(self) -> T {
        self.value
    }
}

impl<T> Default for Delay<T>
where
    T: Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::util::Delay;

    #[test]
    fn test_delay() {
        let mut delay = Delay::new(false);
        delay.set_delay(true, 2);
        assert_eq!(*delay.get_and_advance(), false);
        assert_eq!(*delay.get_and_advance(), false);
        assert_eq!(*delay.get_and_advance(), true);
    }
}
