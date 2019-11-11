use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
pub use void::Void;

#[inline]
pub fn identity<T>(t: T) -> T {
    t
}

#[derive(Clone, Debug)]
pub struct ArcStr(Arc<str>);

impl ArcStr {
    #[inline]
    pub fn from(f: &str) -> ArcStr {
        ArcStr(Arc::from(f.trim_end_matches('\n')))
    }
}

impl Borrow<str> for ArcStr {
    fn borrow(&self) -> &str {
        self.0.borrow()
    }
}

impl Borrow<[u8]> for ArcStr {
    fn borrow(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Borrow<Arc<str>> for ArcStr {
    fn borrow(&self) -> &Arc<str> {
        &self.0
    }
}

pub trait Runner {
    fn gen(input: ArcStr) -> Self
    where
        Self: Sized;

    fn run(&self) -> Box<dyn Display>;

    fn bench(&self, black_box: fn(&dyn Display));

    fn try_gen(input: ArcStr) -> Result<Self, Box<dyn Error>>
    where
        Self: Sized,
    {
        Ok(Self::gen(input))
    }

    fn try_run(&self) -> Result<Box<dyn Display>, Box<dyn Error>> {
        Ok(self.run())
    }
}

pub trait Generator<'a> {
    type Output;

    fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>>;

    fn is_default(&self) -> bool {
        false
    }
}

pub trait RunnerV2<'a, I> {
    type Output;

    fn run(&self, input: I) -> Result<Self::Output, Box<dyn Error>>;

    fn is_implemented(&self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct NotImplemented;

impl Display for NotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "NotImplemented")
    }
}

impl Error for NotImplemented {}
