use std::io::{stdout, Write};
use std::time::{Duration, Instant};

const BUF_SIZE: usize = 8;
const WRITE_CNT: usize = 1;
const BYTES_LEN: usize = BUF_SIZE * WRITE_CNT;
const SAMPLES: usize = 2;

const BUF: [u8; BUF_SIZE] = [0xFF; BUF_SIZE];

fn main() {
    let mut stdout = stdout().lock();

    let mut samples: Vec<_> = (0..SAMPLES).map(|_| run(&mut stdout)).collect();
    samples.sort();

    let min = samples[0];
    let max = samples[SAMPLES - 1];
    let med = samples[SAMPLES / 2];

    eprintln!("Elapsed: [{min:?}, {med:?}, {max:?}]");
    eprintln!(
        "Throughput: [{:.2} MB/s, {:.2} MB/s, {:.2} MB/s]",
        thpt(min),
        thpt(med),
        thpt(max)
    );
}

fn run<W: Write>(out: &mut W) -> Duration {
    let start = Instant::now();
    for _ in 0..WRITE_CNT {
        out.write_all(&BUF).unwrap();
    }

    start.elapsed()
}

fn thpt(duration: Duration) -> f64 {
    (BYTES_LEN as f64 / 1000000.0) / duration.as_secs_f64()
}
