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

use iced::widget::{button, text_input, Button, Column, Text, TextInput};
use iced::{Alignment, Element, Sandbox, Settings};

// Reuse your existing functions here...

#[derive(Default)]
struct GUI {
    thread_choice: Thread,
    scale_input: text_input::State,
    scale: String,
    compute_button: button::State,
    result: Option<String>,
}

#[derive(Clone, Copy, Debug)]
enum Thread {
    Single,
    Multi,
}

impl Default for Thread {
    fn default() -> Self {
        Thread::Single
    }
}

#[derive(Debug, Clone)]
enum Message {
    ThreadChanged(Thread),
    ScaleChanged(String),
    Compute,
}

impl Sandbox for GUI {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Prime Number Generator")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::ThreadChanged(thread) => {
                self.thread_choice = thread;
            }
            Message::ScaleChanged(scale) => {
                self.scale = scale;
            }
            Message::Compute => {
                let scale = self.scale.parse().unwrap_or(1.0);
                let result = match self.thread_choice {
                    Thread::Single => {
                        // single_core_bench(scale);
                        "Single-thread result".to_string()
                    }
                    Thread::Multi => {
                        // multi_core_bench(scale);
                        "Multi-thread result".to_string()
                    }
                };
                self.result = Some(result);
            }
        }
    }

    fn view(&mut self) -> Element<Self::Message> {
        let scale_input = TextInput::new(
            &mut self.scale_input,
            "Enter scale factor",
            &self.scale,
            Message::ScaleChanged,
        );

        let compute_button =
            Button::new(&mut self.compute_button, Text::new("Compute")).on_press(Message::Compute);

        let result_text = if let Some(result) = &self.result {
            Text::new(result)
        } else {
            Text::new("")
        };

        let single_thread_button = Button::new(Text::new("Single-thread"))
            .on_press(Message::ThreadChanged(Thread::Single));

        let multi_thread_button =
            Button::new(Text::new("Multi-thread")).on_press(Message::ThreadChanged(Thread::Multi));

        Column::new()
            .push(single_thread_button)
            .push(multi_thread_button)
            .push(scale_input)
            .push(compute_button)
            .push(result_text)
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}

fn main() {
    GUI::run(Settings::default());
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
    println!("Score: {:.2} tries/s", score);
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
