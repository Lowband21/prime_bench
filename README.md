# Prime Bench - A Prime Number Finding Benchmark Program
## Overview
Do you have hardware just sitting there after burning a hole in your wallet? Do you want to see it sweat?
Have I got the program for you, Prime Bench is a benchmark program written in Rust, optimized for multi-threaded performance and single-threaded use. The program aims to search for prime numbers using a Monte Carlo method based on the Solovay-Strassen primality test, and in the process, measure the performance of your system.
The program allows users to perform this benchmark in either single core or all core mode, giving flexibility in terms of understanding the capability of the hardware in both scenarios.

## Key Features
- Solovay-Strassen Primality Test: The program utilizes the Solovay-Strassen algorithm to perform a Monte Carlo method of prime number identification. This approach ensures that the search is both efficient and highly accurate.
- Concurrency: The program is designed to operate both in a single-threaded environment and a multi-threaded environment. It utilizes the power of concurrent computing to speed up the process when multiple cores are available.
- Scalable Workload: The workload for each core can be scaled using a user input "scale factor", which makes the benchmarking process adaptable to various system capabilities.
- Interactive User Interface: The program includes a command-line interactive user interface for an easy-to-navigate experience.

## Installation
First, clone the repository:

```
git clone https://github.com/YourGitHub/prime_explorer.git
cd prime_bench
```
Then, run the project with cargo:
```
cargo run --release

```

The program will prompt you to choose between "Multi-thread" or "Single-thread". Depending on your choice, it will run the program utilizing either multiple cores or a single core.

Next, you will be asked to enter a scale factor. This number is a multiplier for the base workload per core, allowing you to adjust the duration and intensity of the benchmark.

The program will then generate and test random numbers for primality. Finally, it will output the number of primes found, total attempts made, time taken, and the score which indicates the number of tries per second.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License. See the LICENSE file for details.
