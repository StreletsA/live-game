use crate::common::field::field::Field;

pub trait FieldPrinter : Send + Sync {
    fn print_field(&self, field: &Field);
    fn clear_display(&self);
}

pub struct DefaultFieldPrinter;

impl DefaultFieldPrinter {
    pub fn new() -> Self {
        return DefaultFieldPrinter {};
    }
}

impl FieldPrinter for DefaultFieldPrinter {
    fn print_field(&self, field: &Field) {
        for i in 0..field.get_width() {
            for j in 0..field.get_height() {
                print!("{} ", field.get_value(i, j));
            }

            println!()
        }
    }

    fn clear_display(&self) {
        println!();
    }
}