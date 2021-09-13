pub fn get_first_cli_arg() -> String {
    return std::env::args().nth(1).expect("No stack given.");
}
