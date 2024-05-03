use rand::Rng;
use std::cmp;
use indoc::indoc;
use rust_decimal_macros::dec;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use super::format_duration_as_hms;

// Определение структур, аналогичных NamedTuple в Python
#[derive(Debug)]
pub struct ThievingSimResult {
    time: i32,
    money_earned: i32,
    success_thieving_count: i32,
    failed_thieving_count: i32,
    thieving_count: i32,
}

#[derive(Debug)]
pub struct ThievingSimConfig {
    health_regeneration_interval: Decimal, // in seconds
    health_regeneration_amount: i32,
    max_health: i32,
    steal_interval: Decimal, // in seconds
    steal_success_chance: f32,
    min_damage: i32,
    max_damage: i32,
    min_gold: i32,
    max_gold: i32,
}

impl ThievingSimConfig {
    pub fn new(
        health_regeneration_interval: Decimal, health_regeneration_amount: i32, max_health: i32, steal_interval: Decimal, steal_success_chance: f32, min_damage: i32, max_damage: i32, min_gold: i32, max_gold: i32) -> Self {
        Self {
            health_regeneration_interval,
            health_regeneration_amount,
            max_health,
            steal_interval,
            steal_success_chance,
            min_damage,
            max_damage,
            min_gold,
            max_gold,
        }
    }
}

pub fn sim(config: &ThievingSimConfig) -> ThievingSimResult {
    let mut rng = rand::thread_rng();
    let mut current_health = config.max_health;
    let mut gold_earn = 0;
    let mut success_thieving_count = 0;
    let mut failed_thieving_count = 0;
    let mut thieving_count = 0;
    let mut time = dec!(0.0);
    let max_time = Decimal::from(60 * 60 * 8);

    while current_health > 0 && time < max_time {
        // Attempt to steal every 3 seconds
        if time % config.steal_interval == dec!(0) {
            thieving_count += 1;
            if rng.gen::<f32>() > config.steal_success_chance {
                // Failed steal attempt, take damage and get stunned
                failed_thieving_count += 1;
                let damage = rng.gen_range(config.min_damage..=config.max_damage);
                current_health -= damage;

                // Check if health drops below zero
                if current_health <= 0 {
                    break;
                }

                // Stunned for 3 seconds
                time += dec!(3.0);
            } else {
                success_thieving_count += 1;
                gold_earn += rng.gen_range(config.min_gold..=config.max_gold);
            }
        }

        // Health regeneration every 8 seconds
        if time % config.health_regeneration_interval == dec!(0) {
            current_health = cmp::min(current_health + config.health_regeneration_amount, config.max_health);
        }

        // Increment time
        time += dec!(0.1);
    }

    ThievingSimResult {
        time: time.to_i32().unwrap(),
        money_earned: gold_earn,
        success_thieving_count,
        failed_thieving_count,
        thieving_count,
    }
}


pub fn format_thieve_results(results: &[ThievingSimResult]) -> String {
    let mut mean_time = 0.0;
    let mut mean_money_earned = 0.0;
    let mut success_thieving_count_sum = 0;
    let mut failed_thieving_count_sum = 0;
    let mut thieving_count_sum = 0;

    let mut sorted_seconds = Vec::new();
    let mut sorted_money_earned = Vec::new();

    for result in results {
        mean_time += result.time as f64;
        mean_money_earned += result.money_earned as f64;
        success_thieving_count_sum += result.success_thieving_count;
        failed_thieving_count_sum += result.failed_thieving_count;
        thieving_count_sum += result.thieving_count;

        sorted_seconds.push(result.time);
        sorted_money_earned.push(result.money_earned);
    }

    mean_time /= results.len() as f64;
    mean_money_earned /= results.len() as f64;

    sorted_seconds.sort();
    sorted_money_earned.sort();

    let min_mean_time = sorted_seconds.iter().take(500).sum::<i32>() as f64 / 500.0;
    let min_money_earned = sorted_money_earned.iter().take(500).sum::<i32>() as f64 / 500.0;

    let max_mean_time = sorted_seconds.iter().rev().take(500).sum::<i32>() as f64 / 500.0;
    let max_money_earned = sorted_money_earned.iter().rev().take(500).sum::<i32>() as f64 / 500.0;

    format!(
        indoc!(r#"
        Mean time: {}
        Max mean time: {}
        Min mean time: {}
        --------------------
        Mean money earned: {:.2}
        Max money earned: {:.2}
        Min money earned: {:.2}
        --------------------
        Success thieving: {:.2}
        Failed thieving: {:.2}
        Thieving count: {:.2}
        "#),
        format_duration_as_hms(mean_time),
        format_duration_as_hms(max_mean_time),
        format_duration_as_hms(min_mean_time),
        mean_money_earned,
        max_money_earned,
        min_money_earned,
        success_thieving_count_sum as f64 / results.len() as f64,
        failed_thieving_count_sum as f64 / results.len() as f64,
        thieving_count_sum as f64 / results.len() as f64,
    )
}