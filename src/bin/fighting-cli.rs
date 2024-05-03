use kdam::tqdm;
use sim::fighting::{FightingSimConfig, FightingSimResult, format_fighting_results, sim};

fn main() {
    let config = FightingSimConfig::new(
        720,
        8,
        8,
        1,
        111,
        0.76,
        3.0,
        300,
        0,
        116,
        0.35,
        2.4,
    );
    let mut sims: Vec<FightingSimResult> = Vec::new();
    for _ in tqdm!(0..5000) {
        sims.push(sim(&config));
    }

    println!("\n{}\n", format_fighting_results(&sims));
}
