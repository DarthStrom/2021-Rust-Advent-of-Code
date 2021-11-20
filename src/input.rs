use std::fs;

pub fn get_contents(file_name: &str) -> String {
    fs::read_to_string(format!("input/{}.txt", file_name)).unwrap()
}

pub fn get_lines(file_name: &str) -> Vec<String> {
    let contents = get_contents(file_name);
    contents.lines().map(str::to_owned).collect()
}
