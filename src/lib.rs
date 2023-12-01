pub fn get_input(file_name: &str) -> (String, &str) {
    let path_name = format!("input/{}", file_name);
    let raw_input = std::fs::read_to_string(path_name).expect("Wrong file name!");
    let input = raw_input.trim_end().to_string();
    let Some(lf_pos) = input.find('\n') else { return (input, "\n") };
    let Some(lf_pre) = input.as_bytes().get(lf_pos-1) else { return (input, "\n") };
    if *lf_pre == b'\r' { (input, "\r\n")}
    else { (input, "\n")}
}
