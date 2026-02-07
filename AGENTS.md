# AGENTS.md

This file provides guidance to AI coding agents when working with code in this repository.

## Project Overview

nacos-sdk-rust-binding-py is a Python binding for nacos-sdk-rust using PyO3. It provides both synchronous and asynchronous APIs for Nacos configuration and service discovery.

## Build Commands

### Development Build
```bash
# Install dependencies and build
pip install maturin
maturin develop

# Build with release optimizations
maturin build --release
```

### Install from Wheel
```bash
pip install target/wheels/nacos_sdk_rust_binding_py-*.whl --force-reinstall
```

## Testing

The project uses `behave` for BDD testing:
```bash
# Install test dependencies
pip install behave

# Build and run tests
maturin develop -E test
behave tests
```

## Code Architecture

### Core Module Structure

- **`src/lib.rs`** - Module entry point and `ClientOptions` struct
- **`src/config.rs`** - Synchronous Config client (`NacosConfigClient`)
- **`src/async_config.rs`** - Asynchronous Config client (`AsyncNacosConfigClient`)
- **`src/naming.rs`** - Synchronous Naming client (`NacosNamingClient`)
- **`src/async_naming.rs`** - Asynchronous Naming client (`AsyncNacosNamingClient`)

### Key Architectural Pattern

**Global Tokio Runtime**: The project uses a global single-threaded Tokio runtime for blocking operations in sync clients:

```rust
// From src/lib.rs
static RT: OnceLock<tokio::runtime::Runtime> = OnceLock::new();

pub fn get_runtime() -> &'static tokio::runtime::Runtime {
    RT.get_or_init(|| {
        tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("Failed to create Tokio runtime")
    })
}

pub fn block_on<F>(future: F) -> F::Output
where
    F: std::future::Future,
{
    get_runtime().block_on(future)
}
```

All sync client methods use `crate::block_on()` to execute async calls from the underlying nacos-sdk.

### ClientOptions

Central configuration struct supporting:
- `server_addr`, `namespace`, `app_name` - Connection settings
- `username`, `password` - HTTP auth
- `access_key`, `access_secret`, `signature_region_id` - Aliyun RAM auth
- `naming_push_empty_protection` - Naming protection (default true)
- `naming_load_cache_at_start` - Load naming cache at startup
- `config_load_cache_at_start` - Load config cache at startup

### Dependencies

- `pyo3` - Python bindings
- `pyo3-async-runtimes` - Async runtime support
- `tokio` - Single-threaded runtime for sync operations
- `nacos-sdk` - Underlying Rust SDK nacos-sdk-rust

## Environment Variables

The underlying nacos-sdk-rust supports these environment variables:

- `NACOS_CLIENT_LOGGER_LEVEL=INFO` - Log level (default INFO)
- `NACOS_CLIENT_COMMON_THREAD_CORES=4` - Client thread count (default: 1)
- `NACOS_CLIENT_NAMING_PUSH_EMPTY_PROTECTION=false` - Disable naming push protection

Logs are written to `$HOME/logs/nacos/`.

## Type Stubs

`nacos_sdk_rust_binding_py.pyi` contains Python type definitions for IDE support. Update this when adding new public APIs.

## Version Management

Both `Cargo.toml` and `pyproject.toml` use dynamic versioning. The version in `Cargo.toml` determines the package version.

## Important Notes

- Sync clients block on async operations using the global Tokio runtime
- Async clients directly expose async methods via `pyo3-async-runtimes`
- The project requires a running Nacos server for testing (default: 127.0.0.1:8848)
- Example files in `examples/` directory demonstrate usage patterns
