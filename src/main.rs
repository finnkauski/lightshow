use lightshow::run_script;
use std::env::args;

fn main() {
    let arguments: Vec<String> = args().collect();
    run_script(&arguments[1])
}
