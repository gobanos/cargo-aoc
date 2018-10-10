extern crate reqwest;
use reqwest::Client;

use std::time::{Instant, Duration};
use std::marker::PhantomData;

#[inline]
pub fn identity<T>(t: T) -> T { t }

const COOKIE: &str = "session=53616c7465645f5f5be86546add1d74265114cd6b3617545ba301f4df94136496d32bcfd2386a5ad82ed44dc1ab1d052";

pub struct Generator<I, G, F, R, A> where G: Fn(String) -> I, F: Fn(&A) -> R, I: AsRef<A>, A: ?Sized{
    day: u8,
    generator: G,
    processor: F,
    _marker_a: PhantomData<A>,
}

impl<I, G, F, R, A> Generator<I, G, F, R, A> where G: Fn(String) -> I, F: Fn(&A) -> R, I: AsRef<A>, A: ?Sized {
    pub fn new(day: u8, generator: G, processor: F) -> Generator<I, G, F, R, A> {
        Generator {
            day,
            generator,
            processor,
            _marker_a: PhantomData,
        }
    }

    pub fn generate(self) -> Runner<I, F, R, A> {
        let input_str = Client::new().get(&format!("https://adventofcode.com/2015/day/{}/input", self.day))
            .header("Cookie", COOKIE)
            .send()
            .unwrap()
            .text()
            .unwrap();

        let start_time = Instant::now();
        let input = (self.generator)(input_str);
        let duration = Instant::now() - start_time;

        Runner {
            day: self.day,
            input,
            processor: self.processor,
            generator_duration: duration,
            _marker_a: PhantomData,
        }
    }
}

impl<F, R, A> Generator<String, fn(String) -> String, F, R, A> where F: Fn(&A) -> R, String: AsRef<A>, A: ?Sized {
    pub fn simple(day: u8, processor: F) -> Generator<String, fn(String) -> String, F, R, A> {
        Generator::new(day, identity, processor)
    }
}

pub struct Runner<I, F, R, A> where F: Fn(&A) -> R, I: AsRef<A>, A: ?Sized {
    day: u8,
    input: I,
    processor: F,
    generator_duration: Duration,
    _marker_a: PhantomData<A>,
}

impl<I, F, R, A> Runner<I, F, R, A> where F: Fn(&A) -> R, I: AsRef<A>, A: ?Sized {
    #[inline]
    pub fn run(&self) -> R {
        (self.processor)(self.input.as_ref())
    }
}
