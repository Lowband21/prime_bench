use num_bigint::{BigUint, RandBigInt};
use num_traits::{One, ToPrimitive};
use rayon::prelude::*;
use std::time::Instant;

fn generate_odd_random_number(bits: u32) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut num = rng.gen_biguint_range(
        &BigUint::from(2u128).pow(bits - 1),
        &BigUint::from(2u128).pow(bits),
    );
    if num.clone() % 2u128 == BigUint::from(0u128) {
        num += BigUint::from(1u128);
    }
    BigUint::from(num)
}

fn jacobi_symbol(mut a: BigUint, mut n: BigUint) -> i32 {
    assert!(n.clone() % 2u8 == 1u64.into());
    let mut s = 1;
    while a != 0u64.into() {
        while a.clone() % 2u8 == 0u64.into() {
            a /= 2u8;
            let n_mod_8: u8 = (&n % 8u8).to_u8().unwrap();
            if n_mod_8 == 3 || n_mod_8 == 5 {
                s = -s;
            }
        }
        std::mem::swap(&mut n, &mut a);
        if (&n % 4u8 == 3u64.into()) && (&a % 4u8 == 3u64.into()) {
            s = -s;
        }
        a %= &n;
    }
    if n == 1u64.into() {
        s
    } else {
        0
    }
}

fn mod_exp(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {
    let mut result: BigUint = BigUint::from(1u64);
    let mut base = base % &modulus;
    let mut exponent = exponent;

    while exponent > 0u8.into() {
        if &exponent % 2u8 == 1u8.into() {
            result = (result * &base) % &modulus;
        }
        base = (&base * &base) % &modulus;
        exponent >>= 1;
    }

    result
}

fn solovay_strassen(n: &BigUint, iterations: u32) -> bool {
    if n == &BigUint::from(2u8) || n == &BigUint::from(3u8) {
        return true;
    }

    let mut rng = rand::thread_rng();
    for _ in 0..iterations {
        let a: BigUint =
            rng.gen_biguint_range(&BigUint::from(2u64), &BigUint::from(n.to_u64_digits()[0]));
        let x = jacobi_symbol(a.clone(), n.clone());
        let expected_result = if x == -1 {
            n - BigUint::one()
        } else {
            BigUint::from(x.abs() as u64)
        };

        if x == 0 || mod_exp(a.clone(), (n - BigUint::one()) >> 1, n.clone()) != expected_result {
            return false;
        }
    }
    true
}

use num_cpus;
use requestty::*;

enum Thread {
    Multi,
    Single,
}

fn main() {
    let core_question = Question::select("core")
        .message("Multi-core or single-core?")
        .default(1)
        .choice("Single-thread")
        .choice("Multi-thread");
    let core_answer = &requestty::prompt_one(core_question).unwrap();
    let core = match core_answer.as_list_item().unwrap().text.as_str() {
        "Single-thread" => Thread::Single,
        "Multi-thread" => Thread::Multi,
        &_ => panic!("Impossible"),
    };

    match core {
        Thread::Single => {
            let scale_question = Question::float("scale")
                .message("Enter a scale factor: ")
                .default(0.2)
                .build();
            let answer = &requestty::prompt_one(scale_question).unwrap();
            let scale = answer.as_float().unwrap().clone();
            single_core_bench(scale);
        }
        Thread::Multi => {
            let scale_question = Question::float("scale")
                .message("Enter a scale factor: ")
                .default(3.0)
                .build();
            let answer = &requestty::prompt_one(scale_question).unwrap();
            let scale = answer.as_float().unwrap().clone();
            multi_core_bench(scale);
        }
    }
}
fn single_core_bench(scale: f64) {
    let num_cores = num_cpus::get();
    // Adjust these parameters for the workload.
    let num_tries_per_core = (1024.0 * scale) as usize;
    let num_bits = 2048;
    let num_iterations = 128;

    let total_tries = num_tries_per_core * num_cores;

    let now = Instant::now();

    let num_primes: usize = (0..total_tries)
        .into_iter()
        .map(|_| {
            let odd_num = generate_odd_random_number(num_bits);
            if solovay_strassen(&odd_num, num_iterations) {
                1
            } else {
                0
            }
        })
        .sum();

    let elapsed = now.elapsed().as_secs_f64();

    println!(
        "Found {} {} bit prime numbers in {} attempts and {}s",
        num_primes, num_bits, total_tries, elapsed
    );

    let score = total_tries as f64 / elapsed;
    println!("Score: {:>2} tries/s", score);
}

fn multi_core_bench(scale: f64) {
    let num_cores = num_cpus::get();
    // Adjust these parameters for the workload.
    let num_tries_per_core = (1024.0 * scale) as usize;
    let num_bits = 2048;
    let num_iterations = 128;

    let total_tries = num_tries_per_core * num_cores;

    let now = Instant::now();

    let num_primes: usize = (0..total_tries)
        .into_par_iter()
        .map(|_| {
            let odd_num = generate_odd_random_number(num_bits);
            if solovay_strassen(&odd_num, num_iterations) {
                1
            } else {
                0
            }
        })
        .sum();

    let elapsed = now.elapsed().as_secs_f64();

    println!(
        "Found {} {} bit prime numbers in {} attempts and {:.4}s",
        num_primes, num_bits, total_tries, elapsed
    );

    let score = total_tries as f64 / elapsed;
    println!("Score: {:.2} tries/s", score);
}
