
    {
        let start_time = Instant::now();

        match Factory::{RUNNER_NAME}(input_day{DAY}.clone()) {
            Ok(runner) => {
                let inter_time = Instant::now();

                match runner.try_run() {
                    Ok(result) => {
                        let final_time = Instant::now();
                        println!("{RUNNER_DISPLAY} : {}\n\tgenerator: {:?},\n\trunner: {:?}\n", result, (inter_time - start_time), (final_time - inter_time));
                    },
                    Err(e) => eprintln!("{RUNNER_DISPLAY} : FAILED while running :\n{:?}\n", e)
                }
            },
            Err(e) => eprintln!("{RUNNER_DISPLAY} : FAILED while generating :\n{:?}\n", e)
        }
    }
