use std::fs;

const ZERO: u8 = 48;
// const ONE: u8 = 49;

fn main() {
    let inputs = fs::read_to_string("./inputs.txt").unwrap();

    let gamma_rate = gamma_rate(&inputs);
    let espilon_rate = EpsilonRate(reverse_bits(&gamma_rate.0));

    let gamma_rate = binary_to_decimal(&gamma_rate.0);
    let espilon_rate = binary_to_decimal(&espilon_rate.0);

    let power_cons = gamma_rate * espilon_rate;

    println!(
        "result: \n{}\n{}\nPOWER: {}",
        gamma_rate, espilon_rate, power_cons
    );
    let bytes_list: Vec<_> = inputs.lines().map(|s| s.as_bytes()).collect();

    let mut i = 0;
    let oxy = aaaaaaaaaaaaa(&bytes_list, &mut i);
    let mut j = 0;
    let co2 = bbbbbbbbbbbb(&bytes_list, &mut j);

    let oxy = binary_to_decimal(&oxy);
    let co2 = binary_to_decimal(&co2);

    let result = oxy * co2;

    println!("oxy: {}\nco2: {}\nresult: {}", oxy, &co2, result);
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

fn reverse_bits(bytes: &str) -> String {
    bytes
        .chars()
        .map(|bit| if bit == '0' { '1' } else { '0' })
        .collect()
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

fn aaaaaaaaaaaaa(inputs: &Vec<&[u8]>, i: &mut usize) -> String {
    if inputs.len() < 2 {
        let x = inputs.iter().next().unwrap();

        x.iter()
            .map(|bit| {
                if *bit == ZERO {
                    0.to_string()
                } else {
                    1.to_string()
                }
            })
            .collect()
    } else {
        let mut zero_list = Vec::new();
        let mut one_list = Vec::new();

        for bytes in inputs.iter() {
            let bit = bytes[*i];

            if bit == ZERO {
                zero_list.push(*bytes);
            } else {
                one_list.push(*bytes);
            }
        }

        *i += 1;
        if one_list.len() >= zero_list.len() {
            aaaaaaaaaaaaa(&one_list, i)
        } else {
            aaaaaaaaaaaaa(&zero_list, i)
        }
    }
}
fn bbbbbbbbbbbb(inputs: &Vec<&[u8]>, i: &mut usize) -> String {
    if inputs.len() < 2 {
        let x = inputs.iter().next().unwrap();

        x.iter()
            .map(|bit| {
                if *bit == ZERO {
                    0.to_string()
                } else {
                    1.to_string()
                }
            })
            .collect()
    } else {
        let mut zero_list = Vec::new();
        let mut one_list = Vec::new();

        for bytes in inputs.iter() {
            let bit = bytes[*i];

            if bit == ZERO {
                zero_list.push(*bytes);
            } else {
                one_list.push(*bytes);
            }
        }

        *i += 1;
        if one_list.len() < zero_list.len() {
            bbbbbbbbbbbb(&one_list, i)
        } else {
            bbbbbbbbbbbb(&zero_list, i)
        }
    }
}
