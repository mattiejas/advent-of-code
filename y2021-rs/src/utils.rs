pub fn input(day: i8, test: bool) -> String {
    let file_name = format!(
        "src/input/d{:02}{}.txt",
        day,
        if test { ".test" } else { "" }
    );
    return std::fs::read_to_string(file_name).expect("Unable to read file");
}
