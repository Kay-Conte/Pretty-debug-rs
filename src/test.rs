use crate::{debug, ModuleTag};

pub enum Modules {
    Test,
}

impl ModuleTag for Modules {
    fn display(&self) -> String {
        match self {
            Modules::Test => String::from("[Test]"),
        }
    }
}

#[test]
fn test_debug() {
    debug(&Modules::Test, "Test")
}
