
use res_project::components::analyser::read_trame;

use std::env;

fn main() {

    for arg in env::args().skip(1) {
        read_trame(&arg);
    }
}

