# RustMap

```
 ____            _   __  __
|  _ \ _   _ ___| |_|  \/  | __ _ _ __
| |_) | | | / __| __| |\/| |/ _` | '_ \
|  _ <| |_| \__ \ |_| |  | | (_| | |_) |
|_| \_\\__,_|___/\__|_|  |_|\__,_| .__/
                                  |_|
```

A fast, minimalist port scanner written in Rust with multi-threading support.

## Features

- **Fast Multi-threaded Scanning** - Leverage multiple threads for concurrent port scanning
- **Flexible Port Specification** - Scan single ports, ranges, or use default scan (1-10000)
- **DNS Resolution** - Supports both IP addresses and hostnames
- **Colorized Output** - Clear, professional terminal output with crossterm
- **Lightweight** - Minimal dependencies, pure Rust implementation
- **Configurable** - Adjust thread count for speed vs resource usage

## Installation

### Prerequisites

- Rust 1.70+ (with Cargo)

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/rustmap.git
cd rustmap

# Build the project
cargo build --release

# The binary will be at ./target/release/rustmap
```

### Install via Cargo

```bash
cargo install --path .
```

## Usage

```bash
rustmap <TARGET> [OPTIONS]
```

### Basic Examples

```bash
# Scan default ports (1-10000) on localhost
rustmap localhost

# Scan a specific port
rustmap 192.168.1.1 -p 80

# Scan a range of ports
rustmap example.com -p 1-1000

# Scan with custom thread count and timeout
rustmap 192.168.1.1 -p 1-65535 -t 200 -T 100
```

### Options

| Option  | Short | Long        | Description                                    | Default |
| ------- | ----- | ----------- | ---------------------------------------------- | ------- |
| Target  | -     | -           | IP address or hostname (required)              | -       |
| Ports   | `-p`  | `--ports`   | Port(s) to scan: single (80) or range (1-1000) | 1-10000 |
| Threads | `-t`  | `--threads` | Number of concurrent threads                   | 100     |
| Timeout | `-T`  | `--timeout` | Timeout for scanning port                      | 500     |
| Banner  | `-g`  | `--banner`  | Enable banner grabbing on open ports           | -       |

### Advanced Examples

```bash
# Fast scan with many threads
rustmap 192.168.1.1 -p 1-1000 -t 500

# Slow, stealthy scan
rustmap target.com -p 1-1000 -t 10

# Scan common web ports
rustmap example.com -p 80-443

# Scan a single port to check if it's open
rustmap localhost -p 22
```

## How It Works

1. **DNS Resolution**: Converts hostnames to IPv4 addresses
2. **Port Distribution**: Splits the port range across multiple threads
3. **Concurrent Scanning**: Each thread attempts TCP connections to its assigned ports
4. **Result Collection**: Aggregates and sorts open ports from all threads
5. **Display**: Shows formatted results with color-coded status

## Architecture

```
rustmap/
├── src/
│   ├── main.rs             # Entry point and orchestration
│   ├── args.rs             # CLI argument parsing (clap)
│   ├── target.rs           # DNS resolution and IP handling
│   ├── ports.rs            # Port range parsing
│   ├── scanner.rs          # Multi-threaded port scanning logic
│   ├── style.rs            # ASCII art and branding
|   └── banner_grab.rs      # Banner Grabbing
└── Cargo.toml
```

## Technical Details

### Dependencies

- **clap** - Command-line argument parsing
- **crossterm** - Cross-platform terminal manipulation and colors

### Performance Considerations

- **Thread Count**: More threads = faster scans, but diminishing returns after ~500 threads
- **Timeout**: 500ms per connection attempt (configurable in code)
- **Network I/O Bound**: Performance limited by network latency, not CPU

### Why Rust?

- **Memory Safety**: No buffer overflows or memory leaks
- **Concurrency**: Fearless threading with ownership system
- **Performance**: Near C-level performance with high-level abstractions
- **Cross-platform**: Works on Linux, macOS, and Windows

## Legal Disclaimer

**IMPORTANT**: This tool is for educational and authorized testing purposes only.

- Only scan systems you own or have explicit permission to test
- Unauthorized port scanning may be illegal in your jurisdiction
- The authors are not responsible for misuse of this tool

**Responsible Use Guidelines:**

- Your own machines and networks
- Authorized penetration testing engagements
- Bug bounty programs (within scope)
- Never scan any system without explicit written permission

## Roadmap

~~[ ] Service detection (identify what's running on open ports)~~

- [ ] Output to file (JSON, CSV, XML formats)
- [ ] Progress bar with `indicatif`
- [ ] IPv6 support
- [ ] CIDR notation support (scan entire subnets)
- [ ] TUI interface with `ratatui`
- [ ] SYN scan option (requires elevated privileges)
- [ ] Configurable timeouts via CLI

## Contributing

Contributions are welcome! Please feel free to submit issues or pull requests.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Inspired by [RustScan](https://github.com/RustScan/RustScan)
- Built as a learning project for Rust systems programming
- Thanks to the Rust community for excellent documentation
