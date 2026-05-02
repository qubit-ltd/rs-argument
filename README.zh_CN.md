# Qubit Argument

面向 Qubit Rust 项目的参数和状态校验工具库。

`qubit-argument` 提供一组扩展 trait 和函数，用于校验数值、字符串、集合、`Option`、索引边界和状态条件。

## 功能

- 数值范围、相等和不相等校验。
- 字符串空白、长度和正则匹配校验。
- 集合长度和元素约束校验。
- `Option` 校验辅助函数。
- 状态和边界检查函数。
- 轻量的 `ArgumentError` 和 `ArgumentResult`。

## 安装

```toml
[dependencies]
qubit-argument = "0.1.0"
```

## 快速开始

```rust
use qubit_argument::{ArgumentResult, NumericArgument, StringArgument};

fn validate(age: i32, name: &str) -> ArgumentResult<()> {
    age.require_in_closed_range("age", 0, 150)?;
    name.require_non_blank("name")?;
    Ok(())
}
```

## 模块

- `argument`：校验 trait、检查函数、`ArgumentError` 和 `ArgumentResult`。

常用 trait 和函数已在 crate 顶层重新导出。
