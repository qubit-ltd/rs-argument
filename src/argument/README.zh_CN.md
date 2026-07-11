# 参数校验指南

作者：胡海星

## 用途

Qubit Argument 用于在 API 和配置边界校验值，同时保持值原有的所有权形态。
所有公共项都直接从 `qubit_argument` 导入，具体实现模块是私有细节。

每个校验函数都返回 `ArgumentResult<T>`，即
`Result<T, ArgumentError>` 的别名。`ArgumentError` 包含
`ArgumentPath` 和非穷尽的 `ArgumentErrorKind`，下游代码可以直接匹配
结构化错误，不必解析诊断文本。

校验扩展 trait 是封闭的，只为本指南列出的类型提供实现。
`ArgumentErrorKind`、`ArgumentValue` 和 `LengthMetric` 都是非穷尽类型；下游
模式匹配必须保留通配分支。

## 错误模型与约束类型

错误模型中的所有类型也都直接从 crate 根导出：

| API | 用途 |
| --- | --- |
| `ArgumentError::new`、`path`、`kind`、`with_path_prefix`、`into_parts` | 构造、检查或添加前缀到持有所有权的结构化错误 |
| `ArgumentPath::new`、`as_str`、`with_prefix` | 保存、借用或添加前缀到参数路径或嵌套字段路径 |
| `ArgumentResultExt::with_path_prefix` | 只为失败的嵌套校验补充父级上下文 |
| `ArgumentErrorKind` | 匹配缺失、空白、空值、长度、比较、范围、非法约束、NaN、非有限值、索引、边界、正则和自定义错误 |
| `ArgumentValue` | 无损保存有符号数、无符号数、`f32`、`f64` 或 `Duration` |
| `LengthConstraint`、`ComparisonConstraint` | 描述长度约束和标量比较约束 |
| `LengthMetric` | 在长度错误中区分 UTF-8 字节数、Unicode 标量值数量和集合元素数量 |
| `ArgumentBound`、`RangeConstraint::new`、`lower`、`upper`、`into_bounds` | 描述包含、排除或无界的数值范围 |
| `IndexRole`、`PatternExpectation` | 区分索引语义和正则匹配预期 |

`ArgumentError` 持有这些上下文并实现 `std::error::Error`。其 `Display`
输出会转义为单行文本，只用于诊断，不是稳定的解析协议；结构字段仍保留原值。
`Length` 和 `InvalidLengthConstraint` 都保留 `LengthMetric`，因此不同校验器
产生的相同数值长度不会成为相等的错误。

## 保留所有权的链式校验

扩展 trait 方法消费并返回 `Self`。持有所有权的值在成功后仍持有所有权，
借用仍是原借用，校验过程不会克隆输入。

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

## 结构化错误与下游转换

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

fn validate_workers(workers: usize) -> Result<usize, DomainError> {
    let workers = workers.require_positive("workers")?;
    Ok(workers)
}
```

这种转换会保留完整的结构化错误，并让 `?` 自然完成传播。程序逻辑应通过
`error.path()` 和 `error.kind()` 检查错误；`Display` 只用于面向用户的
诊断信息。

当嵌套校验器只报告局部路径时，可在 `?` 前调用
`ArgumentResultExt::with_path_prefix`。该方法不会为 `Ok` 分配路径字符串，
只会为 `ArgumentError` 分支添加父级前缀。

若某个值由程序构造过程保证，调用方可以显式使用带说明的 `expect`，将校验
错误升级为内部不变量错误：

```rust
use qubit_argument::NumericArgument;

let workers = 4_usize
    .require_positive("workers")
    .expect("编译期写入的工作线程数必须为正数");
assert_eq!(workers, 4);
```

## 数值校验

`NumericArgument` 支持全部原生整数类型以及 `f32`、`f64`。每个方法校验
成功后都原样返回数值，不会克隆。

| 方法 | 错误种类 |
| --- | --- |
| `require_zero`、`require_non_zero`、`require_positive`、`require_non_negative`、`require_negative`、`require_non_positive` | `Comparison`；NaN 对应 `NotANumber` |
| `require_less_than`、`require_at_most`、`require_greater_than`、`require_at_least` | `Comparison`；实参或边界为 NaN 时对应 `NotANumber` |
| `require_in_range` | `Range`、`InvalidRangeConstraint` 或 `NotANumber` |

`require_in_range` 接受标准 `RangeBounds`，支持包含端点、排除端点和无界
端点。反向边界，以及端点相等但至少一侧排除的区间，会返回
`InvalidRangeConstraint`。

## Duration 与浮点校验

`DurationArgument` 为 `std::time::Duration` 提供实现。成功时原样返回精确的
持续时间，比较失败时使用带单位的 `ArgumentValue::Duration`。

| 方法 | 错误种类 |
| --- | --- |
| `require_positive` | 与 `Duration::ZERO` 比较的 `Comparison` |
| `require_less_than`、`require_at_most`、`require_greater_than`、`require_at_least` | 保存精确持续时间边界的 `Comparison` |

`FloatArgument::require_finite` 只为 `f32` 和 `f64` 提供实现。NaN 返回
`NotANumber`，正负无穷返回 `NotFinite { actual }`。成功的有限值保留精确位
模式，并可继续调用 `NumericArgument` 方法。

## 字符串校验

`StringArgument` 为 `String` 和 `&str` 提供实现。每个方法成功后都会原样
返回字符串值或借用，不会克隆；任何字符串校验错误都不会保存原始被检查
字符串。

| 方法 | 度量方式 | 错误种类 |
| --- | --- | --- |
| `require_non_blank` | Unicode 空白 | `Blank` |
| `require_byte_len`、`require_byte_len_at_least`、`require_byte_len_at_most` | UTF-8 字节 | `Length` |
| `require_byte_len_in` | UTF-8 字节 | `Length` 或 `InvalidLengthConstraint` |
| `require_char_count`、`require_char_count_at_least`、`require_char_count_at_most` | Unicode 标量值 | `Length` |
| `require_char_count_in` | Unicode 标量值 | `Length` 或 `InvalidLengthConstraint` |
| `require_match`、`require_not_match`（`regex` feature） | `Regex::is_match` | `Pattern` |

字节长度与 Unicode 标量值数量不可混用。`"汉😀"` 的字节长度为七，标量值
数量为二；Unicode 标量值数量也不等于 grapheme cluster 数量。

正则方法仅在启用可选 `regex` feature 后可用。它们直接使用
`Regex::is_match`，不会隐式添加锚点；需要整串匹配时，应在模式中显式写出
锚点。

## 集合校验

`CollectionArgument` 为 `Vec<T>`、`&[T]` 和 `[T; N]` 提供实现。校验
成功后会原样返回集合，不会克隆元素。

| 方法 | 错误种类 |
| --- | --- |
| `require_non_empty` | `Empty` |
| `require_len`、`require_len_at_least`、`require_len_at_most` | `Length` |
| `require_len_in` | `Length` 或 `InvalidLengthConstraint` |

## Option 校验

`OptionArgument::require_some` 会移出存在的值而不克隆；`None` 对应
`Missing`。`validate_if_some` 将存在的值借给调用方校验器，成功后原样返回
`Option`，不会克隆；遇到 `None` 时不执行校验器，并原样传播校验器返回的
`ArgumentError`。

## 自定义规则与边界校验

| 函数 | 成功值 | 错误种类 |
| --- | --- | --- |
| `require_that` | 原值，不克隆 | `Custom` |
| `check_bounds` | `()` | `Bounds` |
| `check_element_index` | 原索引 | `Index`，角色为 `IndexRole::Element` |
| `check_position_index` | 原索引 | `Index`，角色为 `IndexRole::Position` |
| `check_position_range` | 校验后的 `start..end` | `IndexRange` |

`check_bounds` 先比较再相减，不执行未经检查的 `offset + length` 运算。
`require_that` 只在失败时复制调用方提供的路径、code 和 message；调用方必须
确保自定义消息不包含敏感信息。

## Features

默认 feature 集为空。仅在确实需要正则校验时启用：

```toml
[dependencies]
qubit-argument = "0.4"

# 仅在需要正则校验时启用。
qubit-argument = { version = "0.4", features = ["regex"] }
```

## 验证

```bash
cargo test --doc --no-default-features
cargo test --doc --all-features
RUSTDOCFLAGS='-D warnings' cargo doc --no-deps --all-features
```
