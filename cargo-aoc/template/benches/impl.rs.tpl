
    {
        let runner = Factory::{RUNNER_NAME}(input_day{DAY}.clone())
            .expect("failed to generate input for {NAME}");
        let fun = Fun::new("{NAME}", move |b, _| b.iter(|| runner.bench(black_box)));
        {PART_NAME}.push(fun);
    }