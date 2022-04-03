// to create file in windows -> "cd . > <filename>"
// to start a project -> "cargo init"
// to build and compile the code -> "cargo run"
// for build in dev -> "cargo build" 
// for build in production -> "cargo build --release"
mod print;
mod vars;

fn main() {
    // print::run();
    vars::run();
}


