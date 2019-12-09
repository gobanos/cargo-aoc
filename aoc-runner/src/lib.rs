use std::borrow::Borrow;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::sync::Arc;
pub use void::Void;

pub trait Generator<'a> {
    type Output;

    fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>>;

    fn is_default(&self) -> bool {
        false
    }
}

pub trait Runner<'a, I> {
    type Output;

    fn run(&self, input: I) -> Result<Self::Output, Box<dyn Error>>;

    fn is_implemented(&self) -> bool {
        true
    }
}

pub trait GeneratorDefault {}

impl<'a, T> Generator<'a> for &T where T: GeneratorDefault {
    type Output = &'a str;

    fn generate(&self, input: &'a str) -> Result<Self::Output, Box<dyn Error>> {
        Ok(input)
    }

    fn is_default(&self) -> bool {
        true
    }
}

pub trait RunnerDefault {
    type Input;
}

impl<'a, I, T> Runner<'a, I> for &T where T: RunnerDefault<Input = I> {
    type Output = Void;

    fn run(&self, _input: I) -> Result<Self::Output, Box<dyn Error>> {
        Err(Box::new(NotImplemented))
    }

    fn is_implemented(&self) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct NotImplemented;

impl Display for NotImplemented {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Not Implemented")
    }
}

impl Error for NotImplemented {}

#[derive(Debug)]
pub struct GeneratorFailed;

impl Display for GeneratorFailed {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "Generator Failed")
    }
}

impl Error for GeneratorFailed {}
