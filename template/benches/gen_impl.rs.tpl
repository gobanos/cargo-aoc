
    {
        let input = input_day{DAY}.clone();
        let fun = Fun::new("{NAME}", move |b, _| b.iter(|| Factory::{RUNNER_NAME}(input.clone()).unwrap()));
        {GEN_NAME}.push(fun);
    }