


pub struct Expression;

pub struct PassertHelper {
    expressions: Vec<Expression>
}

impl PassertHelper {
    pub fn new(column_offset: usize, string: &str) -> PassertHelper {
        println!("NEWWWWWW {:?} {}", column_offset, string);
        PassertHelper {expressions: Vec::new()}
    }

    pub fn add_expression(&mut self, offset: usize, value: String) {
        println!("ADD EXPR {} {}", offset, value);
    }

    pub fn print_result(&mut self) {
        println!("PRINTING RESULT");
    }
}

#[test]
fn it_works() {
}
