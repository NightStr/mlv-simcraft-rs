use std::cmp::min;
use indoc::indoc;
use rand::Rng;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use super::format_duration_as_hms;
use statistical::{mean, median};

const RESPAWN_TIME: Decimal = dec!(3);


pub struct FightingSimResult {
    time: u16,
    enemy_killed: u16
}


pub struct FightingSimConfig {
    player_health: u16,
    player_health_regen: u16,
    player_regen_interval: Decimal,
    player_damage_min: u16,
    player_damage_max: u16,
    player_hit_chance: f32,
    player_attack_interval: Decimal,

    enemy_health: u16,
    enemy_damage_min: u16,
    enemy_damage_max: u16,
    enemy_hit_chance: f32,
    enemy_attack_interval: Decimal
}

impl FightingSimConfig {
    pub fn new(
        player_health: u16,
        player_health_regen: u16,
        player_regen_interval: u16,
        player_damage_min: u16,
        player_damage_max: u16,
        player_hit_chance: f32,
        player_attack_interval: f32,
        enemy_health: u16,
        enemy_damage_min: u16,
        enemy_damage_max: u16,
        enemy_hit_chance: f32,
        enemy_attack_interval: f32,
    ) -> Self {
        Self {
            player_health,
            player_health_regen,
            player_regen_interval: Decimal::from(player_regen_interval),
            player_damage_min,
            player_damage_max,
            player_hit_chance,
            player_attack_interval: Decimal::try_from(player_attack_interval).unwrap(),

            enemy_health,
            enemy_damage_min,
            enemy_damage_max,
            enemy_hit_chance,
            enemy_attack_interval: Decimal::try_from(enemy_attack_interval).unwrap(),
        }
    }
}


// Simulation function
pub fn sim(config: &FightingSimConfig) -> FightingSimResult {
    let mut rng = rand::thread_rng();

    let mut player_current_health = config.player_health;
    let mut time = dec!(0);
    let mut enemy_killed = 0;
    let mut regen_timer = dec!(0);
    let mut enemy_current_health = config.enemy_health;
    let max_time = Decimal::from(60 * 60 * 8);

    while player_current_health > 0 && time < max_time {
        // Player attacks
        let attack_roll = rng.gen::<f32>();
        if attack_roll <= config.player_hit_chance {
            let damage = rng.gen_range(
                config.player_damage_min..config.player_damage_max + 1
            );
            enemy_current_health = if damage > enemy_current_health { 0 } else { enemy_current_health - damage };
            if enemy_current_health == 0 {
                enemy_current_health = config.enemy_health;
                time += RESPAWN_TIME;  // Waiting for next enemy to respawn
                enemy_killed += 1;
                continue;
            }
        }
        // Enemy attacks
        let enemy_attack_roll = rng.gen::<f32>();
        if enemy_attack_roll <= config.enemy_hit_chance {
            let enemy_damage= rng.gen_range(
                config.enemy_damage_min..config.enemy_damage_max + 1
            );
            player_current_health = if enemy_damage > player_current_health { 0 } else { player_current_health - enemy_damage };
        }

        // Health regeneration for the player
        regen_timer = regen_timer + config.player_attack_interval;
        while player_current_health > 0 && regen_timer >= config.player_regen_interval {
            player_current_health += config.player_health_regen;
            regen_timer -= config.player_regen_interval;
        }
        // Update time
        time += config.player_attack_interval;

        // Health cap
        player_current_health = min(player_current_health, config.player_health);
    }
    return FightingSimResult{time: time.to_u16().unwrap(), enemy_killed}
}

fn min_max<T>(values: &[T]) -> (T, T)
where
    T: Copy + PartialOrd,
{
    let mut min = values[0];
    let mut max = values[0];
    for &v in values.iter() {
        if v.partial_cmp(&min).unwrap() == std::cmp::Ordering::Less {
            min = v;
        }
        if v.partial_cmp(&max).unwrap() == std::cmp::Ordering::Greater {
            max = v;
        }
    }
    (min, max)
}


pub fn format_fighting_results(results: &[FightingSimResult]) -> String {
    let mut simulations_time_results: Vec<f64> = Vec::new();
    let mut enemy_killed: Vec<f64> = Vec::new();

    for r in results {
        simulations_time_results.push(r.time as f64);
        enemy_killed.push(r.enemy_killed as f64);
    }
    let mean_time = format_duration_as_hms(mean(&simulations_time_results));
    let median_time = format_duration_as_hms(median(&simulations_time_results));

    let (min_time, max_time) = min_max(&simulations_time_results);

    let mean_killed = mean(&enemy_killed);
    let median_killed = median(&enemy_killed);
    let (min_killed, max_killed) = min_max(&enemy_killed);

    format!(
        indoc!(r#"
        Mean time: {}
        Median time: {}
        Min time: {}
        Max time: {}
        ------------------------
        Mean killed: {:.2}
        Median killed: {:.2}
        Min killed: {}
        Max killed: {}
        "#),
        mean_time,
        median_time,
        format_duration_as_hms(min_time),
        format_duration_as_hms(max_time),
        mean_killed,
        median_killed,
        min_killed,
        max_killed,
    )
}
