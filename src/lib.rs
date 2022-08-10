#[cfg(test)]
mod test;

pub trait Debugger {
    fn display(&self) -> String;

    #[inline]
    fn debug(&self, message: &str) {
        println!("{} {}", self.display(), message)
    }
}
