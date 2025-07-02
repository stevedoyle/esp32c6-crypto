# ESP32-C6 Crypto Benchmarking

This project benchmarks cryptographic performance on the ESP32-C6 microcontroller using hardware-accelerated crypto engines for both AES encryption and SHA-256 hashing.

## Overview

The ESP32-C6 features dedicated hardware acceleration for cryptographic operations. This benchmark suite measures the performance of:

- **AES-256-CTR encryption** with DMA acceleration for high-throughput data processing
- **SHA-256 hashing** for data integrity and authentication applications

## Features

- AES-256-CTR encryption benchmarking with DMA acceleration
- SHA-256 hash function benchmarking
- Multiple buffer size testing (64 bytes to 32 KB)
- Detailed performance metrics including:
  - AES throughput in MB/s
  - SHA-256 processing time in microseconds
  - Timing overhead measurement

## Hardware Requirements

- ESP32-C6 development board
- USB cable for programming and serial output

## Software Dependencies

- Rust with ESP32 toolchain
- `esp-hal` v1.0.0-beta.1
- `esp-println` for logging
- `esp-alloc` for heap allocation
- `log` for structured logging

## Building and Running

### Prerequisites

1. Install Rust and the ESP32 toolchain:
   ```bash
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

   # Install ESP32 toolchain
   cargo install espup
   espup install
   ```

2. Source the environment:
   ```bash
   source ~/export-esp.sh
   ```

### Build and Flash

```bash
# Build the project
cargo build --release

# Flash to ESP32-C6
cargo run --release
```

### Monitor Output

```bash
# View serial output
cargo monitor
```

## Benchmark Results

The benchmark tests both AES-256-CTR encryption and SHA-256 hashing performance across different buffer sizes:

- **64 bytes** - Small packet encryption/hashing
- **128 bytes** - Typical IoT message size
- **256 bytes** - Medium data blocks
- **512 bytes** - Larger data blocks
- **1 KB** - File chunk processing
- **2 KB** - Network packet processing
- **4 KB** - Memory page size
- **8 KB** - Large data blocks
- **16 KB** - High-throughput processing, TLS
- **32 KB** - Maximum buffer size

### AES-256-CTR Performance

Actual performance results on ESP32-C6 at 40MHz CPU clock using AES-256-CTR with DMA:

| Buffer Size | Throughput (MB/s) |
|-------------|-------------------|
| 64 bytes    | 1.85 MB/s        |
| 128 bytes   | 4.04 MB/s        |
| 256 bytes   | 7.44 MB/s        |
| 512 bytes   | 11.74 MB/s       |
| 1 KB        | 16.19 MB/s       |
| 2 KB        | 20.18 MB/s       |
| 4 KB        | 22.97 MB/s       |
| 8 KB        | 24.64 MB/s       |
| 16 KB       | 25.56 MB/s       |
| 32 KB       | 26.06 MB/s       |

### SHA-256 Performance

SHA-256 hashing performance with hardware acceleration:

| Buffer Size | Processing Time |
|-------------|-----------------|
| 64 bytes    | 28 μs           |
| 128 bytes   | 24 μs           |
| 256 bytes   | 24 μs           |
| 512 bytes   | 24 μs           |
| 1 KB        | 23 μs           |
| 2 KB        | 24 μs           |
| 4 KB        | 24 μs           |
| 8 KB        | 24 μs           |
| 16 KB       | 24 μs           |
| 32 KB       | 24 μs           |

*Performance measured using hardware-accelerated crypto engines. Results may vary based on system configuration and workload.*

## Code Structure

```
src/
├── bin/
│   └── main.rs          # Main benchmark application
└── lib.rs               # Library (if any)
Cargo.toml               # Project dependencies
README.md                # This file
```

### Key Components

- **`benchmark_aes_dma()`**: High-level AES benchmarking function that tests multiple buffer sizes
- **`benchmark_single_aes_dma()`**: DMA-based AES benchmarking for a single buffer size
- **`benchmark_sha256()`**: SHA-256 hashing benchmark across multiple buffer sizes
- **`benchmark_single_sha256()`**: Single SHA-256 hash operation timing
- **`timestamp_overhead()`**: Measures timing overhead for accurate performance measurement
- **Buffer management**: Efficient DMA buffer allocation and reuse
- **Performance measurement**: High-precision timing using ESP32-C6 hardware timers

## Configuration

### Heap Allocation
The project allocates 64 KB of heap memory for DMA buffers and general use:

```rust
esp_alloc::heap_allocator!(size: 64 * 1024);
```

### CPU Clock
Runs at maximum CPU clock for optimal performance:

```rust
let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
```

### Logging Levels
Adjust logging verbosity by setting the `RUST_LOG` environment variable:

- `RUST_LOG=info` - Basic benchmark results
- `RUST_LOG=debug` - Detailed timing information
- `RUST_LOG=trace` - Verbose debugging

## Security Considerations

This benchmark uses:
- **AES-256**: 256-bit key size for strong encryption
- **CTR Mode**: Counter mode for stream cipher operation
- **SHA-256**: 256-bit cryptographic hash function
- **Test vectors**: Zero-filled keys and test patterns for consistent benchmarking

⚠️ **Warning**: This code is for benchmarking purposes only. Do not use the test keys, initialization vectors, or hash inputs in production applications.

## Performance Optimization Tips

1. **Use DMA for large buffers**: DMA provides significant performance improvements for buffers > 1 KB
2. **Align buffer sizes**: Use multiples of 16 bytes for optimal AES block processing
3. **Minimize heap allocation**: Reuse DMA buffers when possible
4. **CPU clock settings**: Run at maximum clock speed for best performance

## Troubleshooting

### Common Issues

1. **Build errors**: Ensure ESP32 toolchain is properly installed and sourced
2. **Flash failures**: Check USB connection and board selection
3. **Performance variations**: Ensure stable power supply and minimal system load

### Debug Information

Enable debug logging to see detailed performance metrics:

```bash
RUST_LOG=debug cargo run --release
```

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues for:

- Additional cipher modes (CBC, GCM, etc.)
- Performance optimizations
- Documentation improvements
- Bug fixes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References

- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [ESP-HAL Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [AES Specification (FIPS 197)](https://csrc.nist.gov/publications/detail/fips/197/final)
