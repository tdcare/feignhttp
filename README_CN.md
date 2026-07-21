# FeignHTTP

[![crates.io](https://img.shields.io/crates/v/feignhttp-rs.svg)](https://crates.io/crates/feignhttp-rs)
[![Documentation](https://docs.rs/feignhttp-rs/badge.svg)](https://docs.rs/feignhttp-rs)
[![MIT licensed](https://img.shields.io/github/license/tdcare/feignhttp.svg?color=blue)](./LICENSE)
[![CI](https://github.com/tdcare/feignhttp/workflows/CI/badge.svg)](https://github.com/tdcare/feignhttp/actions?query=workflow%3ACI)

FeignHTTP 是一个声明式 HTTP 客户端，基于 Rust 宏实现。本项目 fork 自 [dxx/feignhttp](https://github.com/dxx/feignhttp) v0.5.x。

> 📖 [English version](./README.md)

## 特性

* 简单易用的声明式宏（`#[get]`、`#[post]` 等）
* 异步请求（支持 tokio / async-std）
* 可配置超时时间
* 支持 form、plain text、JSON 等多种请求/响应格式
* **内置 rustls TLS 支持** — 无需依赖 OpenSSL
* 可选 HTTP 后端（[reqwest](https://github.com/seanmonstar/reqwest) 或 [isahc](https://github.com/sagebind/isahc)）

## 与上游的区别

| 对比项 | feignhttp-rs（本仓库） | 上游 feignhttp（v0.6.x） |
|--------|------------------------|---------------------------|
| **Crate 名** | `feignhttp-rs` | `feignhttp` |
| **rustls TLS** | ✅ 内置（`reqwest-client-rustls`、`isahc-client-rustls`） | ❌ 仅 OpenSSL |
| **reqwest** | v0.11（稳定） | v0.13（最新） |
| **isahc** | v1.7（稳定） | v2.0（最新） |
| **http crate** | v0.2 | v1.1 |
| **Feature 风格** | 扁平结构 + rustls 变体 | 后端 × 格式的矩阵组合 |
| **Error Default** | ✅ `Error` 已实现 `Default` | ❌ 未实现 |

## 快速开始

在 `Cargo.toml` 中添加依赖：

```toml
[dependencies]
feignhttp-rs = "0.5"
tokio = { version = "1", features = ["full"] }
```

示例代码：

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

## 选择 HTTP 后端

默认使用 reqwest：

```toml
feignhttp-rs = "0.5"
```

使用 isahc：

```toml
feignhttp-rs = { version = "0.5", default-features = false, features = ["isahc-client"] }
```

使用 rustls TLS（替代 OpenSSL）：

```toml
feignhttp-rs = { version = "0.5", features = ["reqwest-client-rustls"] }
```

## 更多示例

点击 [此处](./examples) 查看更多示例代码。

## 文档

访问 [docs.rs/feignhttp-rs](https://docs.rs/feignhttp-rs) 查看完整 API 文档。

## 许可证

本项目基于 MIT 许可证开源，详见 [LICENSE](./LICENSE)。
