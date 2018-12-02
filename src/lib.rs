use std::fmt::Display;
use std::sync::Arc;
use std::error::Error;

#[inline]
pub fn identity<T>(t: T) -> T {
    t
}

pub struct ArcStr(Arc<str>);

impl ArcStr {
    #[inline]
    pub fn from(f: &str) -> ArcStr {
        ArcStr(Arc::from(f))
    }
}

impl AsRef<str> for ArcStr {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl AsRef<[u8]> for ArcStr {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

pub trait Runner {
    fn gen(input: ArcStr) -> Self where Self: Sized;
    fn run(&self) -> Box<dyn Display>;
    fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(self.run())
    }
}