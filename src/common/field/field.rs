use std::collections::LinkedList;
use rand::Rng;
use crate::common::consts::consts::{ALIVE, RIP};
use crate::common::printer::field_printer::{DefaultFieldPrinter, FieldPrinter};

pub struct Field {
    width: usize,
    height: usize,
    content: LinkedList<LinkedList<i8>>,
    printer: Box<dyn FieldPrinter>
}

impl Field {
    pub fn new(width: usize, height: usize) -> Self {
        let printer = DefaultFieldPrinter::new();
        return Field::new_with_printer(width, height, Box::new(printer));
    }

    pub fn new_with_printer(width: usize, height: usize, field_printer: Box<dyn FieldPrinter>) -> Self {
        let mut content = LinkedList::new();

        for _ in 0..width {
            let mut row= LinkedList::new();

            for _ in 0..height {
                let num = rand::thread_rng().gen_range(RIP..=ALIVE);
                row.push_back(num);
            }

            content.push_back(row);
        }

        return Field {width, height, content, printer: field_printer }
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn get_value(&self, row: usize, col: usize) -> i8 {
        return self.get_value_from_content(&self.content, row, col);
    }

    pub fn print_field(&self) {
        self.printer.clear_display();
        self.printer.print_field(self);
    }

    fn get_value_from_content(&self, content: &LinkedList<LinkedList<i8>>, row: usize, col: usize) -> i8 {
        let mut row_idx = 0;

        for content_row in content.iter() {
            if row_idx == row {
                let mut col_idx = 0;

                for content_col in content_row.iter() {
                    if col_idx == col {
                        return *content_col;
                    } else {
                        col_idx += 1;
                    }
                }
            } else {
                row_idx += 1;
            }
        }

        return -1;
    }

    pub fn next_gen(&mut self) {
        let mut content = LinkedList::new();

        for i in 0..self.width {
            let mut row= LinkedList::new();

            for j in 0..self.height {
                row.push_back(self.define_new_state(i, j));
            }

            content.push_back(row);
        }

        for row in 0..self.width {
            for col in 0..self.height {
                self.set_value(row, col, self.get_value_from_content(&content, row, col));
            }
        }
    }

    fn set_value(&mut self, row: usize, col: usize, value: i8) {
        let mut row_idx = 0;

        for content_row in self.content.iter_mut() {
            if row_idx == row {
                let mut col_idx = 0;

                for content_col in content_row.iter_mut() {
                    if col_idx == col {
                        *content_col = value;
                        break;
                    } else {
                        col_idx += 1;
                    }
                }
            } else {
                row_idx += 1;
            }
        }
    }

    fn define_new_state(&self, row: usize, col: usize) -> i8 {
        let cur_state = self.get_value(row, col);
        let mut alive_count = 0;

        for dx in -1i32..=1 {
            for dy in -1i32..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let cur_col = col as i32 + dx;
                let cur_row = row as i32 + dy;

                if cur_row < 0 || cur_row >= self.width as i32 || cur_col < 0 || cur_col >= self.height as i32 {
                    continue;
                }

                if self.get_value(cur_row as usize, cur_col as usize) == ALIVE {
                    alive_count += 1;
                }
            }
        }

        if cur_state == RIP && alive_count == 3 {
            return ALIVE;
        }

        if cur_state == ALIVE && (alive_count == 2 || alive_count == 3) {
            return ALIVE;
        }

        return RIP;
    }
}