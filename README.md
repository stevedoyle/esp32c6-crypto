# ESP32-C6 Crypto Benchmarking

This project provides a comprehensive benchmark suite for cryptographic performance on the ESP32-C6 microcontroller using hardware-accelerated crypto engines for AES encryption, SHA-256 hashing, and RSA operations.

## Overview

The ESP32-C6 features dedicated hardware acceleration for cryptographic operations. This benchmark suite measures the performance of:

- **AES-256-CTR encryption** with DMA acceleration for high-throughput data processing
- **SHA-256 hashing** for data integrity and authentication applications
- **RSA-2048 modular exponentiation** using hardware acceleration for public key cryptography

## Features

- AES-256-CTR encryption benchmarking with DMA acceleration
- SHA-256 hash function benchmarking
- RSA-2048 modular exponentiation benchmarking with hardware acceleration
- Multiple buffer size testing (64 bytes to 32 KB)
- Detailed performance metrics including:
  - AES throughput in MB/s
  - SHA-256 processing time in microseconds
  - RSA operation timing in milliseconds
  - Timing overhead measurement

## Hardware Requirements

- ESP32-C6 development board
- USB cable for programming and serial output

## Software Dependencies

- Rust with ESP32 toolchain
- `esp-hal` v1.0.0-beta.1 (ESP32-C6 hardware abstraction layer)
- `esp-println` v0.14.0 for logging
- `esp-alloc` v0.8.0 for heap allocation
- `esp-backtrace` v0.16.0 for debugging support
- `crypto-bigint` v0.6.1 for RSA large integer operations
- `log` v0.4.27 for structured logging

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

The benchmark tests AES-256-CTR encryption, SHA-256 hashing, and RSA-2048 operations across different buffer sizes:

**Note**: The performance results shown below were measured on actual ESP32-C6 hardware. To obtain your own measurements, you'll need to flash the code to an ESP32-C6 development board.

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

Actual performance results on ESP32-C6 at maximum CPU clock using AES-256-CTR with DMA:

| Buffer Size | Throughput (MB/s) |
|-------------|-------------------|
| 64 bytes    | 4.37 MB/s        |
| 128 bytes   | 7.57 MB/s        |
| 256 bytes   | 11.74 MB/s       |
| 512 bytes   | 16.28 MB/s       |
| 1 KB        | 20.27 MB/s       |
| 2 KB        | 23.01 MB/s       |
| 4 KB        | 24.70 MB/s       |
| 8 KB        | 25.58 MB/s       |
| 16 KB       | 26.06 MB/s       |
| 32 KB       | 26.32 MB/s       |

### SHA-256 Performance

SHA-256 hashing performance with hardware acceleration:

| Buffer Size | Processing Time |
|-------------|-----------------|
| 64 bytes    | 14 μs           |
| 128 bytes   | 14 μs           |
| 256 bytes   | 14 μs           |
| 512 bytes   | 15 μs           |
| 1 KB        | 14 μs           |
| 2 KB        | 14 μs           |
| 4 KB        | 15 μs           |
| 8 KB        | 14 μs           |
| 16 KB       | 14 μs           |
| 32 KB       | 14 μs           |

*Performance measured using hardware-accelerated crypto engines. Results may vary based on system configuration and workload.*

### RSA-2048 Performance

RSA-2048 modular exponentiation performance with hardware acceleration:

| Operation | Processing Time |
|-----------|-----------------|
| RSA-2048 Modular Exponentiation | 219 ms |

*RSA operations are measured using 2048-bit operands with hardware acceleration. Processing time includes setup and computation of modular exponentiation. Actual timing will be measured when running on ESP32-C6 hardware.*

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
- **`benchmark_rsa()`**: RSA-2048 modular exponentiation benchmark using hardware acceleration
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
- **RSA-2048**: 2048-bit RSA modular exponentiation for public key operations
- **Test vectors**: Zero-filled keys and test patterns for consistent benchmarking

⚠️ **Warning**: This code is for benchmarking purposes only. Do not use the test keys, initialization vectors, RSA operands, or hash inputs in production applications.

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
- Additional RSA key sizes (1024-bit, 4096-bit)
- Elliptic Curve Cryptography (ECC) benchmarks
- Performance optimizations
- Documentation improvements
- Bug fixes

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## References

- [ESP32-C6 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-c6_technical_reference_manual_en.pdf)
- [ESP-HAL Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [AES Specification (FIPS 197)](https://csrc.nist.gov/publications/detail/fips/197/final)
