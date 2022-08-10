#[cfg(test)]
mod test;

pub trait ModuleTag {
    fn display(&self) -> String;
}

#[inline]
pub fn debug(module: &dyn ModuleTag, message: &str) {
    println!("{} {}", module.display(), message)
}
