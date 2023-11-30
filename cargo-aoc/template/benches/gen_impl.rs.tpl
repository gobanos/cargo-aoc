
    {
        let input = input_day{DAY}.clone();
        group.bench_function("{NAME}", move |b| b.iter(|| Factory::{RUNNER_NAME}(input.clone()).unwrap()));
    }