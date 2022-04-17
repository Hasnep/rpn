pub fn get_cli_args() -> Vec<String> {
    return std::env::args().skip(1).collect();
}
