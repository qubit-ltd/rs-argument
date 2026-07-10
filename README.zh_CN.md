# Qubit Argument

[![Rust CI](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-argument/coverage-badge.json)](https://qubit-ltd.github.io/rs-argument/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-argument.svg?color=blue)](https://crates.io/crates/qubit-argument)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust、保留所有权的参数校验库。

## 概述

Qubit Argument 提供扩展 trait 和职责单一的检查函数，用于校验数值、
字符串、集合、可选值、索引和边界。每次校验失败都会返回 `ArgumentError`；
错误中包含持有所有权的参数路径和可匹配的 `ArgumentErrorKind`。校验 API
统一返回 `Result`，由调用方决定恢复、转换错误，还是明确将失败视为内部
不变量遭到破坏。

校验成功时，API 会原样返回传入的所有权值或借用，不会隐式克隆，因此可以
在链式校验中继续使用原值。

## 安装

默认 feature 集为空（`default = []`），核心校验不会引入正则引擎这一运行时
依赖。

```toml
[dependencies]
qubit-argument = "0.4"

# 仅在需要正则校验时启用。
qubit-argument = { version = "0.4", features = ["regex"] }
```

## 快速开始

所有 trait 和错误类型都直接从 crate 根导入：

```rust
use qubit_argument::{ArgumentResult, NumericArgument, StringArgument};

fn validate_user(age: u8, name: String) -> ArgumentResult<(u8, String)> {
    let age = age.require_in_range("age", 0..=150)?;
    let name = name
        .require_non_blank("name")?
        .require_char_count_in("name", 3, 32)?;
    Ok((age, name))
}
```

这里的 `age` 和 `name` 都保持原来的类型；其中持有所有权的 `String` 在校验
过程中不会被克隆。

## 转换为下游错误

下游 crate 可以在自己的领域错误中保留结构化参数错误。实现
`From<ArgumentError>` 后，`?` 运算符会直接完成错误转换：

```rust
use qubit_argument::{ArgumentError, NumericArgument};

#[derive(Debug)]
enum DomainError {
    InvalidArgument(ArgumentError),
}

impl From<ArgumentError> for DomainError {
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}

fn validate_pool_size(size: u32) -> Result<u32, DomainError> {
    let size = size.require_positive("pool_size")?;
    Ok(size)
}
```

程序需要根据错误分支处理时，应使用 `ArgumentError::path()` 和
`ArgumentError::kind()`；`Display` 文本面向诊断日志，不是供程序解析的稳定
协议。调用方提供的路径、pattern、custom code/message 会在显示时转义，使
诊断保持单行，但不会改变结构字段中的原值。

默认情况下，校验失败是可恢复错误。若被校验值属于内部不变量而不是外部输入，
调用方可以明确使用带有说明的 `expect`：

```rust
use qubit_argument::NumericArgument;

fn built_in_retry_limit() -> u32 {
    3_u32
        .require_positive("retry_limit")
        .expect("内置重试次数必须是正数")
}
```

## 校验 API

### 数值

`NumericArgument` 支持全部原生整数类型以及 `f32`、`f64`：

- 校验零、非零、正数、非负数、负数和非正数；
- 通过 `require_less_than`、`require_at_most`、
  `require_greater_than` 和 `require_at_least` 进行比较；
- 通过 `require_in_range` 接受标准库中包含、排除或无界的
  `RangeBounds`。

浮点实参或区间端点为 `NaN` 时校验失败。反向区间和结构上为空的区间会与
“数值不在有效区间内”分别报告。

### 字符串

`StringArgument` 为 `String` 和 `&str` 提供实现：

- `require_non_blank` 检查 Unicode 空白字符；
- `require_byte_len*` 方法统计 UTF-8 字节数；
- `require_char_count*` 方法统计 Unicode scalar value（Unicode 标量值）
  的数量；
- 启用 `regex` feature 后，`require_match` 和 `require_not_match` 接受
  已编译的 `regex::Regex`。

字节长度和 Unicode 标量值数量是两种不同的度量。例如，`"汉😀"` 占七个
UTF-8 字节，但只包含两个 Unicode 标量值；标量值数量也不等于用户感知字符
（grapheme cluster）的数量。

正则校验直接采用 `Regex::is_match` 语义，不会隐式添加整串锚点。只有需要匹配
整个字符串时，才应在模式中显式加入 `^` 和 `$`。

### 集合和可选值

`CollectionArgument` 支持 `Vec<T>`、`&[T]` 和 `[T; N]`，校验时不会克隆
元素。它提供非空、精确长度、最小长度、最大长度和闭区间长度校验。

`OptionArgument::require_some` 会移出并返回存在的值。
`OptionArgument::validate_if_some` 只借用存在的值进行校验；遇到 `None` 时
不会执行校验器，成功后原样返回 `Option`，不会克隆内部值。

### 自定义规则和边界

- `require_that` 执行调用方提供的谓词，失败时返回结构化 `Custom` 错误；
- `check_bounds` 校验偏移量和长度，不执行可能溢出的未检查加法；
- `check_element_index` 与 `check_position_index` 区分元素索引和边界位置；
- `check_position_range` 校验半开的位置区间。

## 错误隐私

字符串校验错误只记录参数路径、错误种类、实际长度或数量以及约束，不会保存
原始被校验字符串，因此 `Debug` 和 `Display` 也不会泄露该输入。正则错误保存
模式而不是输入。使用 `require_that` 时，调用方仍需确保自己提供的自定义
`code` 和 `message` 不包含敏感信息。

`Length` 和 `InvalidLengthConstraint` 错误还会携带 `LengthMetric`：UTF-8
字节方法使用 `Bytes`，字符数量方法使用 `UnicodeScalars`，集合方法使用
`Elements`。因此，即使数值长度相同，也能从结构上区分其度量单位。

## 测试

```bash
# 使用默认的空 feature 集测试核心 API
cargo test --no-default-features

# 测试核心 API 和正则校验
cargo test --all-features

# 运行项目 CI 检查
./ci-check.sh
```

覆盖率详情请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

本项目基于 Apache License 2.0 授权。完整许可证文本请参阅
[LICENSE](LICENSE)。

## 贡献

欢迎贡献。请遵循 Rust API 指南，及时更新公共 API 文档与测试，并在提交
Pull Request 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

仓库地址：[https://github.com/qubit-ltd/rs-argument](https://github.com/qubit-ltd/rs-argument)
