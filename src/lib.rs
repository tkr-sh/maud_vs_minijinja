use std::time::{Duration, Instant};

pub fn time_function<F, R>(f: F) -> Duration
where
    F: FnOnce() -> R,
{
    let start_time = Instant::now();
    f();
    let elapsed_time = Instant::now() - start_time;
    elapsed_time
}

pub fn get_mean_execution_time<F, R>(f: F) -> Duration
where
    F: FnOnce() -> R + Copy,
{
    const NUM_ITERATIONS: usize = 10000;

    let mut total_time = Duration::from_secs(0);

    for _ in 0..NUM_ITERATIONS {
        let result = time_function(f);
        total_time += result;
    }

    total_time / NUM_ITERATIONS as u32
}

pub fn test_maud_vs_minijinja<Fmaud, Fminijinja>(f: Fmaud, g: Fminijinja)
where
    Fmaud: FnOnce() -> String + Copy,
    Fminijinja: FnOnce() -> String + Copy,
{
    let maud = get_mean_execution_time(f);
    let minijinja = get_mean_execution_time(g);

    println!("\x1B[1mMaud\x1b[0m: {maud:#?}\n\x1B[1mMiniJinja\x1B[0m: {minijinja:#?}");
}
