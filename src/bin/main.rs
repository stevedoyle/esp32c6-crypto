#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use esp_backtrace as _;
use esp_hal::aes::dma::{AesDma, CipherMode};
use esp_hal::aes::{Aes, Mode};
use esp_hal::clock::CpuClock;
use esp_hal::dma::{DmaRxBuf, DmaTxBuf};
use esp_hal::sha::{Sha, Sha256};
use esp_hal::time::{Duration, Instant};
use esp_hal::{dma_buffers, main};
use log::{debug, info};

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

fn benchmark_aes_dma(aes: AesDma, data_sizes: &[usize]) {
    // Pre-warm the AES DMA
    let (mut aes, throughput) = benchmark_single_aes_dma(aes, 64);

    // Benchmark for each data size
    for &size in data_sizes {
        let throughput;
        (aes, throughput) = benchmark_single_aes_dma(aes, size);
        info!(
            "AES-CTR, DataSize: {size}, Throughput: {:.2} MB/s",
            throughput / 1_000_000.0
        );
    }
}

/// Benchmark AES-CTR with DMA using a fixed buffer size.
/// This function initializes the AES DMA, processes data in chunks, and measures throughput.
/// # Arguments
/// * `aes` - The AES DMA instance to use for processing.
/// * `buffer_size` - The size of the buffer to use for each AES operation,
/// limited to a maximum of 32 KB.
/// # Returns
/// A tuple containing the AES DMA instance and the throughput in bytes per second.
///
fn benchmark_single_aes_dma(mut aes: AesDma, buffer_size: usize) -> (AesDma, f64) {
    // Use a fixed buffer size for the macro, then limit the actual processing
    const MAX_BUFFER_SIZE: usize = 32 * 1024; // 32 KB maximum buffer
    let (output, rx_descriptors, input, tx_descriptors) = dma_buffers!(MAX_BUFFER_SIZE);
    let mut output = DmaRxBuf::new(rx_descriptors, output).unwrap();
    let mut input = DmaTxBuf::new(tx_descriptors, input).unwrap();

    // Ensure buffer_size doesn't exceed maximum
    let actual_buffer_size = buffer_size.min(MAX_BUFFER_SIZE);

    let keybuf = [0_u8; 32];

    debug!(
        "AES DMA benchmark started with buffer size: {} bytes",
        actual_buffer_size
    );

    // Benchmark the AES process call
    let start_time = Instant::now();
    const ITERATIONS: usize = 100; // Reduced iterations for larger buffers
    for _ in 0..ITERATIONS {
        let transfer = aes
            .process(
                actual_buffer_size / 16,
                output,
                input,
                Mode::Encryption256,
                CipherMode::Ctr,
                keybuf,
            )
            .map_err(|e| e.0)
            .unwrap();
        (aes, output, input) = transfer.wait();
    }
    let elapsed = start_time.elapsed();

    debug!(
        "AES DMA process completed in {} microseconds for {} iterations",
        elapsed.as_micros(),
        ITERATIONS
    );
    debug!(
        "Average time per iteration: {:.2} microseconds",
        elapsed.as_micros() as f64 / ITERATIONS as f64
    );
    let data_processed = ITERATIONS * actual_buffer_size;
    let throughput: f64 = data_processed as f64 / elapsed.as_micros() as f64 * 1_000_000.0; // bytes per second
    debug!("Throughput: {:.2} MB/s", throughput / 1_000_000.0);
    debug!("Throughput: {:.2} Mbps", throughput * 8.0 / 1_000_000.0);

    (aes, throughput)
}

fn benchmark_sha256(sha: &mut Sha, data_sizes: &[usize]) {
    let mut input = [0_u8; 32 * 1024]; // Maximum buffer size for SHA-256
    input.fill(0xAB); // Fill with a pattern for testing
    let mut output = [0_u8; 32]; // SHA-256 produces a 32-byte digest

    // Pre-warm the SHA-256
    benchmark_single_sha256(sha, &input, &mut output);

    for &size in data_sizes {
        let elapsed = benchmark_single_sha256(sha, &input[..size], &mut output);
        info!(
            "SHA-256, DataSize: {size}, Time: {} us",
            elapsed.as_micros()
        );
    }
}

fn benchmark_single_sha256(sha: &mut Sha, input: &[u8], output: &mut [u8]) -> Duration {
    let start_time = Instant::now();
    let mut digest = sha.start::<Sha256>();
    digest.update(input).unwrap();
    digest.finish(output).unwrap();
    start_time.elapsed()
}

fn timestamp_overhead() -> Duration {
    // Measure the overhead of timestamping
    let start_time = Instant::now();
    start_time.elapsed()
}

#[main]
fn main() -> ! {
    // generator version: 0.4.0
    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    info!("Setting up Heap Allocator");

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let _ = timestamp_overhead(); // Pre-warm the timestamping
    let overhead = timestamp_overhead();
    info!("Timestamp overhead: {} us", overhead.as_micros());

    let data_sizes = [
        64,
        128,
        256,
        512,
        1024,
        2048,
        4096,
        8192,
        16 * 1024,
        32 * 1024,
    ];

    info!("Starting AES-CTR DMA Benchmark");
    let aes = Aes::new(peripherals.AES).with_dma(peripherals.DMA_CH0);
    benchmark_aes_dma(aes, &data_sizes);

    info!("Starting SHA256 Benchmark");
    let mut sha = Sha::new(peripherals.SHA);
    benchmark_sha256(&mut sha, &data_sizes);

    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(500) {}
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/esp-hal-v1.0.0-beta.1/examples/src/bin
}
