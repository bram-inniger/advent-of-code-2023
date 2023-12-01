use std::fs::read_to_string;

pub fn read_inputs(day: i8) -> Vec<String> {
    let filename = format!("./inputs/day_{:0>2}.txt", day);
    read_input_lines_naive(&filename)
}

// Maybe switch over to a better way to read a file as per here?
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
fn read_input_lines_naive(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Ok(s) => s.lines().map(String::from).collect(),
        Err(e) => panic!(
            "Could not read input file: {}, throwing error {}",
            filename, e
        ),
    }
}
