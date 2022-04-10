// to create file in windows -> "cd . > <filename>"
// to start a project -> "cargo init"
// to build and compile the code -> "cargo run"
// for build in dev -> "cargo build" 
// for build in production -> "cargo build --release"
mod print;
mod vars;
mod types;
mod strings;
mod taking_users;

fn main() {
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    taking_users::run();
}


