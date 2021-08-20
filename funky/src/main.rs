mod functions;
mod pointers;
mod structs;
mod enums;
mod cli;

fn main() {
    functions::greeting("Hello", "Hashirama");
    functions::calculate();
    pointers::reference();
    structs::objects();
    enums::lists();
    cli::io();
}
