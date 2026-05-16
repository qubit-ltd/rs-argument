# Qubit Argument

[![Rust CI](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml/badge.svg)](https://github.com/qubit-ltd/rs-argument/actions/workflows/ci.yml)
[![Coverage](https://img.shields.io/endpoint?url=https://qubit-ltd.github.io/rs-argument/coverage-badge.json)](https://qubit-ltd.github.io/rs-argument/coverage/)
[![Crates.io](https://img.shields.io/crates/v/qubit-argument.svg?color=blue)](https://crates.io/crates/qubit-argument)
[![Rust](https://img.shields.io/badge/rust-1.94+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

面向 Rust 的参数和状态校验工具库。

## 概述

Qubit Argument 提供扩展 trait 和检查函数，用于校验函数参数、配置值、索引、范围和运行时状态。它使用 Rust 的 trait extension 模式表达可读的校验链，并统一返回轻量的 `ArgumentError`。

## 设计目标

- **可读校验**：让检查逻辑靠近被校验的值。
- **小型错误表面**：所有校验失败统一使用 `ArgumentError` 和 `ArgumentResult`。
- **方法链式调用**：返回校验后的值或引用，使多个检查自然组合。
- **默认不 panic**：通过 `Result` 报告无效参数。
- **聚焦范围**：只提供参数校验工具，不引入更宽泛的 common 工具。

## 特性

### 数值校验

- 零值和非零值检查。
- 正数、负数、非正数、非负数检查。
- 闭区间、开区间、左开右闭、左闭右开范围校验。
- 小于、小于等于、大于、大于等于检查。
- 两个具名参数之间的相等和不相等校验。

### 字符串校验

- 非空白检查。
- 精确长度、最小长度、最大长度和长度范围检查。
- 正则匹配和正则不匹配检查。
- 同时支持 `str` 和 `String`。

### 集合和 Option 校验

- 非空集合检查。
- 集合长度约束。
- 可选值存在性检查。
- 对存在的可选值进行条件校验。
- 对 `&[Option<T>]` 进行元素非空检查。

### 状态和边界检查

- 布尔参数和状态断言。
- 类似切片的边界检查。
- 元素索引、位置索引和位置索引范围校验。

## 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
qubit-argument = "0.3.2"
```

## 快速开始

### 数值和字符串校验

```rust
use qubit_argument::{
    ArgumentResult,
    NumericArgument,
    StringArgument,
};

fn validate_user(age: i32, username: &str) -> ArgumentResult<()> {
    age.require_in_closed_range("age", 0, 150)?;
    username
        .require_non_blank("username")?
        .require_length_in_range("username", 3, 20)?;
    Ok(())
}
```

### 集合校验

```rust
use qubit_argument::{
    ArgumentResult,
    CollectionArgument,
};

fn validate_tags(tags: &[String]) -> ArgumentResult<&[String]> {
    tags.require_non_empty("tags")?
        .require_length_at_most("tags", 10)
}
```

### 状态和边界检查

```rust
use qubit_argument::{
    ArgumentResult,
    check_bounds,
    check_state_with_message,
};

fn read_range(offset: usize, length: usize, total: usize) -> ArgumentResult<()> {
    check_state_with_message(total > 0, "total length must be positive")?;
    check_bounds(offset, length, total)
}
```

## API 参考

### Traits

- [`NumericArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.NumericArgument.html) - 数值校验方法。
- [`StringArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.StringArgument.html) - 字符串校验方法。
- [`CollectionArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.CollectionArgument.html) - 集合校验方法。
- [`OptionArgument`](https://docs.rs/qubit-argument/latest/qubit_argument/trait.OptionArgument.html) - 可选值校验方法。

### 错误类型

- [`ArgumentError`](https://docs.rs/qubit-argument/latest/qubit_argument/struct.ArgumentError.html) - 带可读消息的校验错误。
- [`ArgumentResult`](https://docs.rs/qubit-argument/latest/qubit_argument/type.ArgumentResult.html) - 校验 API 返回的结果别名。

### 函数

- `check_argument`、`check_argument_with_message`、`check_argument_fmt`
- `check_state`、`check_state_with_message`
- `check_bounds`、`check_element_index`、`check_position_index`、`check_position_indexes`
- `require_equal`、`require_not_equal`、`require_element_non_null`、`require_null_or`

## 测试与代码覆盖率

本项目对成功路径、失败路径、边界条件和代表性类型实例保持全面测试覆盖。

### 运行测试

```bash
# 运行所有测试
cargo test

# 运行覆盖率报告
./coverage.sh

# 生成文本格式报告
./coverage.sh text

# 运行 CI 检查（格式化、clippy、测试、覆盖率、审计）
./ci-check.sh
```

### 覆盖率指标

详细的覆盖率统计请参见 [COVERAGE.zh_CN.md](COVERAGE.zh_CN.md)。

## 依赖项

运行时依赖保持很少：

- `regex` 用于字符串正则校验。

## 许可证

Copyright (c) 2025 - 2026. Haixing Hu, Qubit Co. Ltd. All rights reserved.

根据 Apache 许可证 2.0 版（"许可证"）授权；
除非遵守许可证，否则您不得使用此文件。
您可以在以下位置获取许可证副本：

    http://www.apache.org/licenses/LICENSE-2.0

除非适用法律要求或书面同意，否则根据许可证分发的软件
按"原样"分发，不附带任何明示或暗示的担保或条件。
有关许可证下的特定语言管理权限和限制，请参阅许可证。

完整的许可证文本请参阅 [LICENSE](LICENSE)。

## 贡献

欢迎贡献！请随时提交 Pull Request。

### 开发指南

- 遵循 Rust API 指南。
- 保持全面的测试覆盖。
- 在文档能帮助理解时，为公共 API 提供示例。
- 提交 PR 前运行 `./ci-check.sh`。

## 作者

**胡海星** - *Qubit Co. Ltd.*

## 相关项目

Qubit 旗下的更多 Rust 库发布在 GitHub 组织 [qubit-ltd](https://github.com/qubit-ltd)。

---

仓库地址：[https://github.com/qubit-ltd/rs-argument](https://github.com/qubit-ltd/rs-argument)
