pub fn get_program_name(path: impl ToString) -> String {
    let path = path.to_string();
    return path
        .split('/')
        .last()
        .unwrap()
        .split('.')
        .next()
        .unwrap()
        .to_string();
}
