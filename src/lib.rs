use std::error::Error;
use std::fmt::Display;
use std::sync::Arc;
use std::borrow::Borrow;

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
