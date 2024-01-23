use std::path::PathBuf;

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


pub fn get_output_path_from_input(input: String) -> PathBuf {
    let p_name = get_program_name(input);
    std::path::Path::new(&format!("./build/{p_name}")).to_owned()
}
