use std::env;
use line_checker::calculator;

fn main() {
    println!("Line Checker V 0.1.0");
    let env_args: Vec<String> = env::args().collect();

    let file_path: &str = &env_args[1];

    let line_count: usize = calculator::check_file(file_path, "//");

    println!("{} lines in file {}", line_count, file_path);
}