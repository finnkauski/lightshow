pub mod lshow;
pub mod parser;
use parser::root;

/// Main script runtime
pub fn run_script(filename: &str) {
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    let bridge = lighthouse::HueBridge::connect();
    let (_, parsed) = root(&contents[..]).expect("Could not parse the file");
    lshow::structure(parsed, bridge);
}
