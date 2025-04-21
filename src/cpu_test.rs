use futures;
use rayon::prelude::*;
use std::env;
use std::thread::available_parallelism;
use std::time::Instant;
use tokio;
use tokio::runtime::Runtime;

const CPU_NUM_CALCS_ENV: &str = "CPU_NUM_CALCS";
const CPU_NUM_ITERS_ENV: &str = "CPU_NUM_ITERS";
const PRINT_WIDHT: usize = 50;

fn factorial(num: u128) -> u128 {
  (1..=num).product()
}

fn add_one_loop(&n_loops: &u64) -> u128 {
  let mut sum = 0;
  for _in in 0..n_loops {
    sum += factorial(20);
  }

  sum
}

fn get_cpu_num() -> usize {
  available_parallelism().unwrap().get()
}

fn run_native_threads(num_iters: u64, available_cores: u64, iter_per_core: &u64, total_calc: &f64) {
  println!("\nRunning in native threads...");
  let now = Instant::now();

  for _i in 0..num_iters {
    let mut results = Vec::new();
    let mut threads = Vec::new();
    for _i in 0..available_cores {
      let iter_per_core_clone = iter_per_core.clone();
      threads.push(std::thread::spawn(move || {
        add_one_loop(&iter_per_core_clone)
      }));
    }
    for thread in threads {
      results.extend(thread.join());
    }
  }
  let elapsed = now.elapsed();
  let calc_per_sec: f64 = (total_calc) / (elapsed.as_secs() as f64);
  println!(
    "{:.<width$}{:.2?}",
    "Total native threads runtime:",
    elapsed,
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Calculations in native threads per second:",
    calc_per_sec,
    width = PRINT_WIDHT
  );
}

fn run_rayon_threads(num_iters: u64, available_cores: u64, iter_per_core: &u64, total_calc: &f64) {
  println!("\nRunning in rayon threads...");
  let now = Instant::now();

  for _i in 0..num_iters {
    (0..available_cores).into_par_iter().for_each(|_| {
      add_one_loop(iter_per_core);
    });
  }

  let elapsed = now.elapsed();
  let calc_per_sec: f64 = (total_calc) / (elapsed.as_secs() as f64);

  println!(
    "{:.<width$}{:.2?}",
    "Total rayon threads runtime:",
    elapsed,
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Calculations in rayon threads per second:",
    calc_per_sec,
    width = PRINT_WIDHT
  );
}

fn run_tokio_threads(num_iters: u64, available_cores: u64, iter_per_core: &u64, total_calc: &f64) {
  let rt = Runtime::new().unwrap();
  rt.block_on(async {
    println!("\nRunning in tokio threads...");
    let now = Instant::now();

    for _i in 0..num_iters {
      let mut handles = Vec::new();
      for _i in 0..available_cores {
        let iter_per_core_clone = iter_per_core.clone();

        handles.push(tokio::spawn(
          async move { add_one_loop(&iter_per_core_clone) },
        ));
      }

      let _ = futures::future::try_join_all(handles).await.unwrap();
    }
    let elapsed = now.elapsed();
    let calc_per_sec: f64 = (total_calc) / (elapsed.as_secs() as f64);
    println!(
      "{:.<width$}{:.2?}",
      "Total tokio threads runtime:",
      elapsed,
      width = PRINT_WIDHT
    );
    println!(
      "{:.<width$}{:.2?}",
      "Calculations in tokio threads per second:",
      calc_per_sec,
      width = PRINT_WIDHT
    );
  });
}

pub fn run_benchmark() {
  println!("\nRunning CPU benchmark test using factorial function");
  // Number of CPUs:
  println!("Number of available threads: {}", get_cpu_num());

  let num_calcs = match env::var(CPU_NUM_CALCS_ENV) {
    Ok(n) => n.parse::<u64>().unwrap_or(10_000_00),
    Err(_) => 10_000_00,
  };

  let num_iters: u64 = match env::var(CPU_NUM_ITERS_ENV) {
    Ok(n) => n.parse::<u64>().unwrap_or(10_000_00),
    Err(_) => 20000,
  };
  let total_calc: u64 = num_calcs * num_iters;
  println!(
    "Running {} calculations over {} iterations each with a total of {} calculations.",
    &num_calcs, &num_iters, &total_calc,
  );

  let available_cores: u64 = get_cpu_num() as u64;
  let iter_per_core: u64 = num_calcs / available_cores;

  run_native_threads(
    num_iters,
    available_cores,
    &iter_per_core,
    &(total_calc as f64),
  );

  run_rayon_threads(
    num_iters,
    available_cores,
    &iter_per_core,
    &(total_calc as f64),
  );

  run_tokio_threads(
    num_iters,
    available_cores,
    &iter_per_core,
    &(total_calc as f64),
  );
}
