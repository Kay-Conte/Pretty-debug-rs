use crate::debug::*;

pub fn test() {
    // ? Calling debug on a different enum variant
    Debug::Connection.debug("Failed to connect")
}
