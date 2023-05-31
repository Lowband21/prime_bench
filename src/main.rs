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
//use requestty::*;

use iced::widget::{Button, Column, Row, Text, TextInput};
use iced::{Alignment, Element, Sandbox, Settings};

// Reuse your existing functions here...

#[derive(Default)]
struct GUI {
    thread_choice: Thread,
    scale_input: String,
    compute_button: Message,
    scale: f64,
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

impl Default for Message {
    fn default() -> Self {
        Message::Compute
    }
}

impl Sandbox for GUI {
    type Message = Message;

    fn new() -> GUI {
        GUI {
            scale_input: "3.0".to_string(),
            compute_button: Message::Compute,
            thread_choice: Thread::Multi, // initialize other fields...
            scale: 2.0,
            result: None,
        }
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
                self.scale_input = scale.clone();
                match scale.parse::<f64>() {
                    Ok(value) => {
                        self.scale = value;
                    }
                    Err(_) => {
                        // Handle the case where `scale` couldn't be parsed as an `f32`
                        // For example, you might want to log an error, or set `scale` to a default value
                    }
                }
            }
            Message::Compute => {
                let scale = self.scale;
                let result = match self.thread_choice {
                    Thread::Single => single_core_bench(scale),
                    Thread::Multi => multi_core_bench(scale),
                };
                self.result = Some(result);
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let scale_input: iced_native::widget::text_input::TextInput<
            '_,
            Message,
            iced_wgpu::Renderer,
        > = TextInput::new(&self.scale_input, &self.scale_input).on_input(Message::ScaleChanged);

        let compute_button = Button::new(Text::new("Compute")).on_press(Message::Compute);

        let result_text = if let Some(result) = &self.result {
            Text::new(result)
        } else {
            Text::new("")
        };

        let single_thread_button = Button::new(Text::new("Single-thread"))
            .on_press(Message::ThreadChanged(Thread::Single));

        let multi_thread_button =
            Button::new(Text::new("Multi-thread")).on_press(Message::ThreadChanged(Thread::Multi));

        // The thread choice buttons are placed in a row.
        let thread_choice_row = Row::new()
            .push(single_thread_button)
            .push(multi_thread_button)
            .spacing(20)
            .align_items(Alignment::Center);

        // The compute button and result text are grouped together.
        let compute_result_column = Column::new()
            .push(compute_button)
            .push(result_text)
            .spacing(10);

        // The main column layout is simplified.
        Column::new()
            .push(thread_choice_row)
            .push(scale_input)
            .push(compute_result_column)
            .padding(20)
            .align_items(Alignment::Center)
            .into()
    }
}

use iced_native;

fn main() {
    match GUI::run(Settings::default()) {
        Ok(_) => println!("Program exited with status code 1"),
        Err(_) => println!("Program exited with status code -1"),
    };
}
fn single_core_bench(scale: f64) -> String {
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

    let score = total_tries as f64 / elapsed;

    let print = format!(
        "Found {} {} bit prime numbers in {} attempts and {:.4}s\nScore: {:.2} tries/s",
        num_primes, num_bits, total_tries, elapsed, score
    );
    print
}

fn multi_core_bench(scale: f64) -> String {
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
    let score = total_tries as f64 / elapsed;

    let print = format!(
        "Found {} {} bit prime numbers in {} attempts and {:.4}s\nScore: {:.2} tries/s",
        num_primes, num_bits, total_tries, elapsed, score
    );
    print
}
