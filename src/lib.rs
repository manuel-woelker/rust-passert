use std::cmp;


pub struct PassertInfo {
    column_offset: usize,
    value: String
}

pub struct PassertHelper {
    column_offset: usize,
    string: String,
    infos: Vec<PassertInfo>
}

impl PassertHelper {


    pub fn new(column_offset: usize, string: &str) -> PassertHelper {
        PassertHelper {column_offset: column_offset, string: String::from(string), infos: Vec::new()}
    }

    pub fn add_expression(&mut self, column_offset: usize, value: String) {
        self.infos.push(PassertInfo {column_offset: column_offset - self.column_offset, value: value})
    }

    fn place_string(line: &mut String, column: usize, what: &str) {
        let neededLength = column+what.len();
        if neededLength > line.len() {
            let diff = neededLength - line.len();
            for _ in 0..diff {
                line.push(' ');
            }
        }
        *line = String::new() + &line[..column] + what + &line[column+what.len()..];
//        line.replace(anchor, anchor + str.length(), str)
    }

    fn fits(line: & String, column: usize, width: usize) -> bool {
        let min = cmp::min(column, line.len());
        let max = cmp::min(column+width, line.len());
        line[min..max].chars().all(char::is_whitespace)
    }

    pub fn print_result(&mut self) {
        println!("Assertion failed:");
        println!("{}", self.string);
        self.infos.sort_by(|a, b| a.column_offset.cmp(&b.column_offset).reverse());
        let mut lines = Vec::new();
        lines.push(String::new());
        'outer: for info in &self.infos {
            PassertHelper::place_string(&mut lines.get_mut(0).unwrap(), info.column_offset, "|");
            for i in 1..lines.len() {
                let mut line = lines.get_mut(i).unwrap();
                if(PassertHelper::fits(&mut line, info.column_offset, info.value.len()+1)) {
                    // found a place
                    PassertHelper::place_string(&mut line, info.column_offset, &info.value);
                    continue 'outer;
                }
                PassertHelper::place_string(&mut line, info.column_offset, "|");
            }
            let mut new_line = String::new();
            PassertHelper::place_string(&mut new_line, info.column_offset, &info.value);
            lines.push(new_line);
        }
        for line in lines {
            println!("{}", line);
        }
    }

}

#[test]
fn it_works() {
}
