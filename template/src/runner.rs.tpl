{
    use std::time::Instant;
    use aoc_runner::ArcStr;

    let start_time = Instant::now();
    let runner = Factory::{RUNNER_NAME}(ArcStr::from(include_str!("../../../../input/{INPUT}")));
    let inter_time = Instant::now();
    let result = runner.run();
    let final_time = Instant::now();
    println!("{RUNNER_DISPLAY} : {}\n\tgenerator: {:?},\n\trunner: {:?}\n", result, (inter_time - start_time), (final_time - inter_time));
}
