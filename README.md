# FeignHTTP

[![crates.io](https://img.shields.io/crates/v/feignhttp-rs.svg)](https://crates.io/crates/feignhttp-rs)
[![Documentation](https://docs.rs/feignhttp-rs/badge.svg)](https://docs.rs/feignhttp-rs)
[![MIT licensed](https://img.shields.io/github/license/tdcare/feignhttp.svg?color=blue)](./LICENSE)
[![CI](https://github.com/tdcare/feignhttp/workflows/CI/badge.svg)](https://github.com/tdcare/feignhttp/actions?query=workflow%3ACI)

FeignHTTP is a declarative HTTP client. Based on rust macros.

## Features

* Easy to use
* Asynchronous request
* Configurable timeout settings
* Supports form, plain text and JSON
* **Built-in rustls TLS support** — no OpenSSL dependency required
* Selectable HTTP backends ([reqwest](https://github.com/seanmonstar/reqwest) or [isahc](https://github.com/sagebind/isahc))

## Differences from upstream

This project is a fork of [dxx/feignhttp](https://github.com/dxx/feignhttp) (v0.5.x). Key differences:

| Aspect | feignhttp-rs (this) | Upstream feignhttp (v0.6.x) |
|--------|---------------------|------------------------------|
| **Crate name** | `feignhttp-rs` | `feignhttp` |
| **rustls TLS** | ✅ Built-in (`reqwest-client-rustls`, `isahc-client-rustls`) | ❌ OpenSSL only |
| **reqwest** | v0.11 (stable) | v0.13 (latest) |
| **isahc** | v1.7 (stable) | v2.0 (latest) |
| **http crate** | v0.2 | v1.1 |
| **Feature style** | Flat: `reqwest-client`, `isahc-client`, plus rustls variants | Matrix: backend × format × TLS |
| **Error Default** | ✅ `Error` implements `Default` | ❌ Not implemented |

## Usage

FeignHTTP mark macros on asynchronous functions, you need a runtime for support async/await. You can use [async-std](https://github.com/async-rs/async-std) or [tokio](https://github.com/tokio-rs/tokio).

async-std:

```toml
[dependencies]
async-std = { version = "1", features = ["attributes", "tokio1"] }
```

The feature `tokio1` is need when use reqwest as the HTTP backend.

tokio:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

Add `feignhttp-rs` in your `Cargo.toml` and use default feature:

```toml
feignhttp-rs = "0.5"
```

Then add the following code:

```rust
use feignhttp_rs::get;

#[get("https://api.github.com")]
async fn github() -> feignhttp_rs::Result<String> {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let r = github().await?;
    println!("result: {}", r);

    Ok(())
}
```

The `get` attribute macro specifies get request, `feignhttp_rs::Result<String>` specifies the return result.
It will send get request to `https://api.github.com` and receive a plain text body.

Using non-default HTTP backend:

```toml
feignhttp-rs = { version = "0.5", default-features = false, features = ["isahc-client"] }
```

The `default-features = false` option disable default reqwest.

For more examples, click [here](./examples).

## Documentation

Read the [documentation](https://docs.rs/feignhttp-rs) for more details.

## License

FeignHTTP is provided under the MIT license. See [LICENSE](./LICENSE).

> 📖 中文文档请见 [README_CN.md](./README_CN.md)
