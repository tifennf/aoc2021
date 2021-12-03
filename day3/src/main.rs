use std::fs;

const ZERO: u8 = 48;
// const ONE: u8 = 49;

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();

    let gamma_rate = gamma_rate(&inputs);
    let espilon_rate = espsilon_rate(&gamma_rate);

    let gamma_rate = binary_to_decimal(&gamma_rate.0);
    let espilon_rate = binary_to_decimal(&espilon_rate.0);

    let power_cons = gamma_rate * espilon_rate;

    println!(
        "result: \n{}\n{}\nPOWER: {}",
        gamma_rate, espilon_rate, power_cons
    );
}

struct GammaRate(String);
struct EpsilonRate(String);

fn binary_to_decimal(n: &str) -> u32 {
    let len = n.len();

    let mut i = 1;

    let mut result = 0.0;

    for bit in n.chars() {
        if bit == '1' {
            result += libm::pow(2.0, (len - i) as f64);
        }
        i += 1;
    }

    result as u32
}

fn espsilon_rate(gamma: &GammaRate) -> EpsilonRate {
    let bytes: String = gamma
        .0
        .chars()
        .map(|bit| if bit == '0' { '1' } else { '0' })
        .collect();

    // u32::from_str_radix(&bytes, 10).unwrap();
    EpsilonRate(bytes)
}

fn gamma_rate(inputs: &str) -> GammaRate {
    let mut bit_count = [0_u8; 12];
    let mut i = 0;

    while i < 12 {
        let mut zero = 0;
        let mut one = 0;

        for bytes in inputs.lines() {
            let bytes = bytes.as_bytes();
            let bit = bytes[i];

            if bit == ZERO {
                zero += 1;
            } else {
                one += 1;
            }
        }

        if one >= zero {
            bit_count[i] = 1;
        }

        i += 1;
    }

    GammaRate(bit_count.iter().map(|bit| bit.to_string()).collect())
}
