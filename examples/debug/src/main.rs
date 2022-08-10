mod debug;

// ? Importing * so we dont have to manually import the Debugger trait as well. 
// ? Notice the pub use in debug.rs
use debug::*;

mod connection;

fn main() {
    Debug::Main.debug("Sent from the main module");

    connection::test();
}
