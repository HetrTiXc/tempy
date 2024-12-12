extern crate rand;

use rand::Rng;
use std::f64::consts::PI;

const HOURS_IN_DAY: usize = 24;
const AVERAGE_TEMP: f64 = 22.0;
const AMPLITUDE: f64 = 5.0;
const NOISE_LEVEL: f64 = 0.5;

pub fn simulate_temperatures(days: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();
    let mut yearly_temperatures = Vec::with_capacity(days);

    for _ in 0..days {
        let time: Vec<i32> = (0..HOURS_IN_DAY as i32).collect();
        let base_temp: Vec<f64> = time.iter()
            .map(|&t| AVERAGE_TEMP + AMPLITUDE * ((2.0 * PI * (t as f64) / HOURS_IN_DAY as f64) - (PI / 2.0)).sin())
            .collect();
        let noise: Vec<f64> = (0..HOURS_IN_DAY)
            .map(|_| rng.gen_range(-NOISE_LEVEL..NOISE_LEVEL))
            .collect();
        let day_temperatures: Vec<f64> = base_temp.iter().zip(noise.iter()).map(|(&t, &n)| t + n).collect();
        yearly_temperatures.push(day_temperatures);
    }

    yearly_temperatures
}