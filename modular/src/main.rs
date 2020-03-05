// declare a module, we could have a block definition for the module, but when we just provide the
// name the compiler will look for a file named "printer.rs"
mod printer;

// bring printer::greetings into scope, so instead of doing printer::greetings::hello we can just
// do greetings::hello
use printer::greetings;

fn main() {
    greetings::hello("world");
}
