// to create file in windows -> "cd . > <filename>"
// to start a project -> "cargo init"
// to build and compile the code -> "cargo run"
// for build in dev -> "cargo build" 
// for build in production -> "cargo build --release"
// for running a single file use rustc filename.rs 

mod print;
mod vars;
mod types;
mod strings;
mod taking_users;
mod inf_loop_game;
mod phils;
mod phils2;
mod phils3;

fn main() {
    // print::run();
    // vars::run();
    // types::run();
    // strings::run();
    // taking_users::run();
    // inf_loop_game::run();
    // phils::run();
    // phils2::run();
    phils3::run();
}


