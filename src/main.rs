mod cdcl;
use std::env;
use cdcl::cdcl;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args[1].ends_with(".cnf") {
        cdcl(&args[1]);
    }
}