# 参数验证模块 (Argument)

作者: 胡海星

## 概述

本模块提供参数验证功能，采用符合 Rust 习惯的设计。通过 trait 扩展模式，为各种类型提供验证方法，支持链式调用。

## 模块结构

```
argument/
├── argument_error.rs       ## 错误类型定义
├── numeric_argument.rs     ## 数值参数验证
├── string_argument.rs      ## 字符串参数验证
├── collection_argument.rs  ## 集合参数验证
├── option_argument.rs      ## Option 参数验证
├── condition.rs            ## 条件和状态验证
└── mod.rs                  ## 模块入口
```

## 核心特性

### 1. 数值验证 (`NumericArgument`)

支持本 crate 已实现的内置数值类型：有符号整数（`i8`、`i16`、`i32`、
`i64`、`i128`、`isize`）、无符号整数（`u8`、`u16`、`u32`、`u64`、
`u128`、`usize`）以及浮点数（`f32`、`f64`）。浮点数验证会拒绝 `NaN`。

```rust
use qubit_argument::argument::NumericArgument;

// 验证非负
let count = 10;
let count = count.require_non_negative("count")?;

// 验证范围
let volume = 50;
let volume = volume.require_in_closed_range("volume", 0, 100)?;

// 链式调用
let age = 25;
let age = age
    .require_non_negative("age")
    .and_then(|a| a.require_in_closed_range("age", 0, 150))?;
```

**可用方法：**
- `require_zero()` - 验证为零
- `require_non_zero()` - 验证非零
- `require_positive()` - 验证为正
- `require_non_negative()` - 验证非负
- `require_negative()` - 验证为负
- `require_non_positive()` - 验证非正
- `require_in_closed_range()` - 闭区间 [min, max]
- `require_in_open_range()` - 开区间 (min, max)
- `require_in_left_open_range()` - 左开右闭 (min, max]
- `require_in_right_open_range()` - 左闭右开 [min, max)
- `require_less()` - 小于
- `require_less_equal()` - 小于等于
- `require_greater()` - 大于
- `require_greater_equal()` - 大于等于

### 2. 字符串验证 (`StringArgument`)

支持 `&str` 和 `String` 类型。

```rust
use qubit_argument::argument::StringArgument;

// 验证非空白
let username = "alice";
let username = username.require_non_blank("username")?;

// 验证长度
let password = "secret123";
let password = password
    .require_length_at_least("password", 8)
    .and_then(|p| p.require_length_at_most("password", 20))?;

// 正则匹配
use regex::Regex;
let email = "user@example.com";
let pattern = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")?;
let email = email.require_match("email", &pattern)?;
```

**可用方法：**
- `require_non_blank()` - 验证非空白
- `require_length_be()` - 长度等于
- `require_length_at_least()` - 最小长度
- `require_length_at_most()` - 最大长度
- `require_length_in_range()` - 长度范围
- `require_match()` - 正则匹配
- `require_not_match()` - 正则不匹配

### 3. 集合验证 (`CollectionArgument`)

支持 `&[T]` 和 `Vec<T>` 类型。

```rust
use qubit_argument::argument::CollectionArgument;

// 验证非空
let items = vec![1, 2, 3];
let items = items.require_non_empty("items")?;

// 验证长度
let tags = vec!["rust", "programming"];
let tags = tags
    .require_non_empty("tags")
    .and_then(|t| t.require_length_at_most("tags", 10))?;
```

**可用方法：**
- `require_non_empty()` - 验证非空
- `require_length_be()` - 长度等于
- `require_length_at_least()` - 最小长度
- `require_length_at_most()` - 最大长度
- `require_length_in_range()` - 长度范围

### 4. Option 验证 (`OptionArgument`)

支持 `Option<T>` 类型。

```rust
use qubit_argument::argument::OptionArgument;

// 验证非空
let timeout: Option<u64> = Some(30);
let timeout = timeout.require_non_null("timeout")?;

// 验证非空且满足条件
let port: Option<u16> = Some(8080);
let port = port.require_non_null_and(
    "port",
    |&p| p >= 1024,
    "必须大于等于 1024"
)?;

// 可选验证
let max_conn: Option<usize> = Some(100);
let max_conn = max_conn.validate_if_present("max_connections", |c| {
    if *c > 10000 {
        Err("连接数过大".into())
    } else {
        Ok(*c)
    }
})?;
```

**可用方法：**
- `require_non_null()` - 验证非 None
- `require_non_null_and()` - 验证非 None 且满足条件
- `validate_if_present()` - 如果存在则验证

### 5. 条件验证

通用的条件和状态验证函数。

```rust
use qubit_argument::argument::{check_argument, check_state, check_bounds};

// 基本条件检查
let is_valid = true;
check_argument(is_valid)?;

// 带消息的检查
check_argument_with_message(count > 0, "计数必须为正")?;

// 状态检查
check_state(is_initialized)?;

// 边界检查
check_bounds(offset, length, total_length)?;

// 索引检查
let index = check_element_index(5, list_size)?;
```

**可用函数：**
- `check_argument()` - 检查参数条件
- `check_argument_with_message()` - 带消息的参数检查
- `check_argument_fmt()` - 格式化消息的参数检查
- `check_state()` - 检查状态条件
- `check_state_with_message()` - 带消息的状态检查
- `check_bounds()` - 边界检查
- `check_element_index()` - 元素索引检查
- `check_position_index()` - 位置索引检查
- `check_position_indexes()` - 位置索引范围检查

## 错误处理

所有验证方法返回 `ArgumentResult<T>`，这是 `Result<T, ArgumentError>` 的类型别名。

```rust
use qubit_argument::argument::{ArgumentError, ArgumentResult};

fn validate_config(port: u16, timeout: u64) -> ArgumentResult<()> {
    port.require_in_closed_range("port", 1024, 65535)?;
    timeout.require_positive("timeout")?;
    Ok(())
}

// 错误处理
match validate_config(80, 30) {
    Ok(_) => println!("配置有效"),
    Err(e) => eprintln!("配置错误: {}", e.message()),
}
```

## 设计理念

1. **类型安全**：利用 Rust 类型系统在编译期保证安全
2. **零成本抽象**：编译后与手动检查性能相同
3. **链式调用**：返回 `Result<Self>` 支持优雅的链式验证
4. **清晰错误**：提供友好的错误消息，包含参数名和值
5. **符合习惯**：遵循 Rust 的设计哲学和最佳实践

## 测试

```bash
cargo test --test argument_tests argument
```

## 文档

运行 `cargo doc --open` 查看完整 API 文档。
