// ? Notice the pub use here,
// ? this is so you may import both the Debug enum
// ?  and the Debugger trait at once
pub use debugger_rs::Debugger;

pub enum Debug {
    Main,
    Connection,
}

impl Debugger for Debug {
    fn display(&self) -> String {
        match self {
            Debug::Main => "[Main]",
            Debug::Connection => "[Connection]",
        }
        .to_string()
    }
}
