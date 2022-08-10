use crate::Debugger;

pub enum Debug {
    Test,
}

impl Debugger for Debug {
    fn display(&self) -> String {
        match self {
            Debug::Test => String::from("[Test]"),
        }
    }
}

#[test]
fn test_debug() {
    Debug::Test.debug("Sent from the test module")
}
