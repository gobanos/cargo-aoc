
    {
        let runner = Factory::{RUNNER_NAME}(ArcStr::from(include_str!("../../../../input/{INPUT}")));
        let fun = Fun::new("{NAME}", move |b, _| b.iter(|| runner.run()));
        {PART_NAME}.push(fun);
    }