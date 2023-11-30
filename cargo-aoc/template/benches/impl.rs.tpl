
    {
        let runner = Factory::{RUNNER_NAME}(input_day{DAY}.clone())
            .expect("failed to generate input for {NAME}");
        group.bench_function("{NAME}", move |b| b.iter(|| runner.bench(black_box)));
    }