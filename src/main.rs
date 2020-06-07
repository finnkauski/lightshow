use lightshow::parser::root;
use lightshow::runtime;

fn main() {
    let filename = "/home/art/projects/rust/lightshow/test.lshow";
    let contents =
        std::fs::read_to_string(filename).expect("Something went wrong reading the file");

    if let Ok((_, entities)) = root(&contents[..]) {
        runtime(entities)
    }
}
