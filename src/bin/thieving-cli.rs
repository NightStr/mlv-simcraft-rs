use sim::thieving::{ThievingSimConfig, ThievingSimResult, format_thieve_results, sim};

use rust_decimal_macros::dec;
use kdam::tqdm;


fn main() {
    // Принимать аргументы для thieving sim config как параметры коммандной строки со значениями по умолчанию
    let config = ThievingSimConfig::new(
        dec!(8), // in seconds
        8,
        720,
        dec!(2.6), // in seconds
        0.9,
        0,
        157,
        51,
        1212,
    );

    let mut sims: Vec<ThievingSimResult> = Vec::new();
    for _ in tqdm!(0..5000) {
        sims.push(sim(&config));
    }

    println!("\n{}\n", format_thieve_results(&sims));
}