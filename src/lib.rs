use std::fmt::Display;
use std::sync::Arc;

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

pub trait Runner
where
    Self::Result: Display,
{
    type Result;

    fn gen(input: ArcStr) -> Self;
    fn run(&self) -> Self::Result;
}

#[macro_export]
macro_rules! aoc {
    ( @as_meta, $m:meta ) => { $m };
    ( @generator, pub fn $name:ident ( $arg:ident : $arg_t:ty ) -> $out_t:tt $body:block ) => {
        pub fn $name($arg: $arg_t) -> $out_t {
            $body
        }
    };
    ( @runner #[ aoc( $day:meta , $part:meta ) ] pub fn $name:ident ( $arg:ident : $arg_t:ty ) -> $out_t:ty $body:block ) => {
        pub use self::$name::runner as $name;

        #[allow(unused_imports)]
        mod $name {
            use super::*;
            use aoc_runner::{ArcStr, Runner};
            use std::marker::PhantomData;

            pub fn runner($arg: $arg_t) -> $out_t {
                $body
            }

            #[derive(Runner)]
            #[runner(runner)]
            pub struct RunnerStruct {
                input: ArcStr,
                output: PhantomData<$out_t>,
            }
        }
    };
    ( @runner_gen $gen_name:ident -> $gen_t:ty , #[ aoc( $day:meta , $part:meta ) ] pub fn $name:ident ( $arg:ident : $arg_t:ty ) -> $out_t:ty $body:block ) => {
        pub use self::$name::runner as $name;

        #[allow(unused_imports)]
        mod $name {
            use super::*;
            use aoc_runner::{ArcStr, Runner};
            use std::marker::PhantomData;

            pub fn runner($arg: $arg_t) -> $out_t {
                $body
            }

            #[derive(Runner)]
            #[runner(runner, $gen_name)]
            pub struct RunnerStruct {
                input: $gen_t,
                output: PhantomData<$out_t>,
            }
        }
    };
    ( #[ aoc( $day:meta , $part:meta ) ] pub fn $name:ident ( $arg:ident : $arg_t:ty ) -> $out_t:ty $body:block ) => {
        aoc! { @runner #[aoc($day, $part)] pub fn $name ($arg: $arg_t) -> $out_t { $body } }
    };
    (
     #[ aoc( $a_day:meta , $a_part:meta ) ] pub fn $a_name:ident ( $a_arg:ident : $a_arg_t:ty ) -> $a_out_t:ty $a_body:block
     #[ aoc( $b_day:meta , $b_part:meta ) ] pub fn $b_name:ident ( $b_arg:ident : $b_arg_t:ty ) -> $b_out_t:ty $b_body:block
    ) => {
        aoc! { @runner #[aoc($a_day, $a_part)] pub fn $a_name ($a_arg: $a_arg_t) -> $a_out_t { $a_body } }
        aoc! { @runner #[aoc($b_day, $b_part)] pub fn $b_name ($b_arg: $b_arg_t) -> $b_out_t { $b_body } }
    };
    (
     #[generator] pub fn $gen_name:ident ( $gen_arg:ident : $gen_arg_t:ty ) -> $gen_out_t:ty $gen_body:block
     #[ aoc( $day:meta , $part:meta ) ] pub fn $name:ident ( $arg:ident : $arg_t:ty ) -> $out_t:ty $body:block
    ) => {
        aoc! { @generator, pub fn $gen_name($gen_arg: $gen_arg_t) -> $gen_out_t { $gen_body } }
        aoc! { @runner_gen $gen_name -> $gen_out_t, #[aoc($day, $part)] pub fn $name ($arg: $arg_t) -> $out_t { $body } }
    };
    (
     #[generator] pub fn $gen_name:ident ( $gen_arg:ident : $gen_arg_t:ty ) -> $gen_out_t:ty $gen_body:block
     #[ aoc( $a_day:meta , $a_part:meta ) ] pub fn $a_name:ident ( $a_arg:ident : $a_arg_t:ty ) -> $a_out_t:ty $a_body:block
     #[ aoc( $b_day:meta , $b_part:meta ) ] pub fn $b_name:ident ( $b_arg:ident : $b_arg_t:ty ) -> $b_out_t:ty $b_body:block
    ) => {
        aoc! { @generator, pub fn $gen_name($gen_arg: $gen_arg_t) -> $gen_out_t { $gen_body } }
        aoc! { @runner_gen $gen_name -> $gen_out_t, #[aoc($a_day, $a_part)] pub fn $a_name ($a_arg: $a_arg_t) -> $a_out_t { $a_body } }
        aoc! { @runner_gen $gen_name -> $gen_out_t, #[aoc($b_day, $b_part)] pub fn $b_name ($b_arg: $b_arg_t) -> $b_out_t { $b_body } }
    };
}
