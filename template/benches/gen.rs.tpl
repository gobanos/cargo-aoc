
    let mut {GEN_NAME} = Vec::new();

    {IMPLS}

    c.bench_functions("Generator Day{DAY}", {GEN_NAME}, ());