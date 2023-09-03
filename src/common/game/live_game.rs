use std::thread::sleep;
use std::time::Duration;

use crate::common::field::field::Field;

pub struct LiveGame {
    field: Field,
    generation: usize
}

impl LiveGame {
    pub fn new(width: usize, height: usize) -> Self {
        let field = Field::new(width, height);

        return LiveGame {field, generation: 0 };
    }

    pub fn get_generation(&self) -> usize {
        return self.generation;
    }

    pub async fn start(&mut self) {
        loop {
            self.field.print_field();
            self.field.next_gen();

            self.generation += 1;
        }
    }

    pub fn next_gen(&mut self) {
        self.field.next_gen();
        self.generation += 1;
    }

    pub(crate) fn start_with_pause(&mut self, pause_in_millis: u64) {
        loop {
            self.field.print_field();
            self.field.next_gen();

            self.generation += 1;

            sleep(Duration::from_millis(pause_in_millis));
        }
    }
}