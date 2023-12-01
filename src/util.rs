use std::fs::read_to_string;

pub fn read_inputs(day: i8) -> Vec<String> {
    let filename = format!("./inputs/day_{:0>2}.txt", day);
    read_lines_naive(&*filename)
}

// TODO switch over to a better way to read a file as per here:
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html#a-more-efficient-approach
fn read_lines_naive(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}
