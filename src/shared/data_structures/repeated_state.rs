#![allow(dead_code)]

pub struct RepeatedState<T: Clone> {
    collection: Vec<T>,
    state: T,
    index: usize,
}

impl<T: Clone> RepeatedState<T> {
    pub fn new(collection: Vec<T>) -> Self {
        match collection.first().clone() {
            Some(first_state) => Self {
                collection: collection.clone(),
                state: first_state.clone(),
                index: 0,
            },
            None => panic!("State collection cannot be empty!"),
        }
    }

    #[inline]
    pub fn state(&self) -> T {
        self.state.clone()
    }

    pub fn next(&mut self) -> T {
        let next_index = self.index + 1;

        match self.collection.get(next_index) {
            Some(next_state) => {
                self.index = next_index;

                next_state.clone()
            }
            None => {
                self.index = 0;

                self.collection[0].clone()
            }
        }
    }
}
