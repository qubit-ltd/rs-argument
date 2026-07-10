# rs-argument v0.4 重设计

日期：2026-07-11

## 1. 背景

`qubit-argument` v0.3.6 提供数值、字符串、集合、`Option`、索引、边界和运行时状态校验。现有实现本身稳定，但 Qubit 工作区内没有其他 `rs-*` 项目依赖它。真实候选下游普遍拥有自己的领域错误类型，因此当前只保存文本消息的 `ArgumentError` 无法保留字段路径、错误种类和约束详情，也无法帮助下游稳定地转换错误。

本次允许破坏性变更，不保留兼容 API。首轮只重构 `rs-argument`，通过模拟下游验证 `From<ArgumentError>` 与 `?` 的组合；真实下游改造须在本轮验证完成并获得用户确认后另行进行。

## 2. 目标

- 将 `ArgumentError` 改为可匹配、可转换、可安全记录的结构化错误。
- 保持所有校验 API 返回 `Result`；是否升级为 panic 由调用点决定。
- 让下游能够实现 `From<ArgumentError> for DomainError` 后直接使用 `?`。
- 校验成功时保留输入的所有权形态，避免隐式 clone。
- 明确字符串字节长度与 Unicode 字符数量的差异。
- 使用标准 `RangeBounds` 统一数值区间校验。
- 核心 API 不依赖第三方 crate；正则支持通过默认关闭的 feature 提供。
- 删除 Java 风格的 `null` 命名和与参数校验无关的状态检查。
- 使用真实行为测试替换重复的覆盖率导向测试。

## 3. 非目标

- 本轮不修改 `rs-event-bus`、`rs-http` 或其他真实下游。
- 不提供多错误聚合。
- 不提供业务对象 derive、过程宏、异步 API 或序列化支持。
- 不替代下游的强类型领域错误。
- 不要求所有参数检查都通过本 crate 完成；保留具体领域信息更重要时，下游应继续显式校验。
- 不提供完整的 panic 型平行 API。

## 4. 错误策略

所有校验失败返回 `ArgumentError`。下游可以透明包装，也可以根据结构化信息转换成领域 variant：

```rust
impl From<ArgumentError> for DomainError {
    fn from(error: ArgumentError) -> Self {
        Self::InvalidArgument(error)
    }
}
```

内部不变量或程序员错误由调用点通过 `expect`、`unwrap_or_else`、`assert!` 或 `debug_assert!` 主动升级为 panic。底层校验 API 不提前替调用方决定恢复策略。

`check_state` 和 `check_state_with_message` 从 crate 中删除。运行时状态错误属于领域错误，不再伪装成参数错误。

## 5. 结构化错误模型

### 5.1 ArgumentError

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentError {
    path: ArgumentPath,
    kind: ArgumentErrorKind,
}
```

提供以下公共方法：

- `new(path: &str, kind: ArgumentErrorKind) -> Self`
- `path(&self) -> &ArgumentPath`
- `kind(&self) -> &ArgumentErrorKind`
- `into_parts(self) -> (ArgumentPath, ArgumentErrorKind)`

`ArgumentError` 手动实现 `Display` 和 `std::error::Error`。不额外保存一份普通 message；`Display` 完全根据 `path` 和 `kind` 生成单行诊断，避免结构化信息与文本不一致。调用方提供的 `path`、pattern、custom code/message 在格式化时统一转义反斜线、控制字符和当前分隔符，结构字段仍保留原值。

`Debug` 用于完整结构诊断，`Display` 用于人类可读日志。`Display` 文本不是供程序解析的稳定协议，下游必须使用访问器和枚举匹配。

所有字段持有数据，使错误自然满足 `Send + Sync + 'static`。

### 5.2 ArgumentPath

`ArgumentPath` 是持有 `String` 的 newtype，表示参数或嵌套配置路径，例如 `name`、`retry.max_attempts`。它实现 `Debug`、`Clone`、`PartialEq`、`Eq`、`Hash`、`Display` 和 `AsRef<str>`。

公共构造参数使用 `&str`，仅在实际构造错误时复制路径；成功路径不为静态参数名分配内存。

### 5.3 ArgumentValue

`ArgumentValue` 只表示本 crate 支持的原生数值：

```rust
pub enum ArgumentValue {
    Signed(i128),
    Unsigned(u128),
    Float32(u32),
    Float64(u64),
}
```

浮点 variant 保存 IEEE 位表示，从而支持 `Eq`，并能准确表示负零、无穷值和 NaN。其 `Debug` 与 `Display` 实现输出重建后的数值语义，而不是仅输出位整数。

字符串实际内容不进入 `ArgumentValue`，避免令牌、密码和用户输入泄漏。字符串错误只记录长度、字符数或正则约束。

### 5.4 约束类型

公共约束类型包括：

- `LengthConstraint`：精确长度、至少、至多、闭区间。
- `LengthMetric`：UTF-8 字节数、Unicode 标量值数量或集合元素数量。
- `ComparisonConstraint`：等于、不等于、小于、至多、大于、至少。
- `ArgumentBound`：无界、包含边界、排除边界。
- `RangeConstraint`：下界和上界。
- `IndexRole`：元素索引或插入位置索引。
- `PatternExpectation`：要求匹配或要求不匹配。

这些类型实现 `Debug`、`Clone`、`PartialEq` 和 `Eq`，供下游稳定匹配；`LengthMetric` 还实现 `Copy`。

### 5.5 ArgumentErrorKind

`ArgumentErrorKind` 标记为 `#[non_exhaustive]`，首版包含：

- `Missing`
- `Blank`
- `Empty`
- `Length { actual, constraint, metric }`
- `Comparison { actual, constraint }`
- `Range { actual, constraint }`
- `InvalidLengthConstraint { constraint, metric }`
- `InvalidRangeConstraint { constraint }`
- `NotANumber`
- `Index { index, size, role }`
- `IndexRange { start, end, size }`
- `Bounds { offset, length, total_length }`
- `Pattern { pattern, expectation }`
- `Custom { code, message }`

`Custom` 的 `code` 是供下游匹配的稳定标识，`message` 只用于诊断。调用方负责确保自定义消息不包含敏感信息。

## 6. 公共 API 与所有权

所有公共类型和 trait 从 crate 根导出。`argument` 及其子模块改为私有实现细节，不再提供多套公共导入路径。

| 类别 | crate-root 导出 |
| --- | --- |
| 错误模型 | `ArgumentError`、`ArgumentErrorKind`、`ArgumentPath`、`ArgumentResult` |
| 值与约束 | `ArgumentValue`、`LengthConstraint`、`LengthMetric`、`ComparisonConstraint`、`ArgumentBound`、`RangeConstraint`、`IndexRole`、`PatternExpectation` |
| 校验 trait | `NumericArgument`、`StringArgument`、`CollectionArgument`、`OptionArgument` |
| 检查函数 | `require_that`、`check_bounds`、`check_element_index`、`check_position_index`、`check_position_range` |

### 6.1 数值校验

保留 `NumericArgument`，支持全部原生整数、`f32` 和 `f64`。方法消费并返回 `Self`：

- `require_zero`
- `require_non_zero`
- `require_positive`
- `require_non_negative`
- `require_negative`
- `require_non_positive`
- `require_less_than`
- `require_at_most`
- `require_greater_than`
- `require_at_least`
- `require_in_range<R: RangeBounds<Self>>`

浮点值和区间边界中的 NaN 返回 `NotANumber`。无穷值按正常比较语义处理。区间下界大于上界时返回 `InvalidRangeConstraint`。下界等于上界时，只有上下界均为包含边界的单点闭区间有效；任一边界为排除边界时返回 `InvalidRangeConstraint`。入口只调用一次 `start_bound` 和一次 `end_bound`，随后使用同一份持有所有权的边界快照完成错误约束构造、NaN/结构校验和成员判断。

### 6.2 字符串校验

`StringArgument` 分别为 `String` 和 `&str` 实现，成功后返回原类型：

- `require_non_blank`
- `require_byte_len`
- `require_byte_len_at_least`
- `require_byte_len_at_most`
- `require_byte_len_in`
- `require_char_count`
- `require_char_count_at_least`
- `require_char_count_at_most`
- `require_char_count_in`

`byte_len` 使用 UTF-8 字节数，`char_count` 使用 Unicode scalar value 数量。首版不引入 grapheme cluster 依赖。

开启 `regex` feature 后额外提供 `require_match` 和 `require_not_match`。正则使用 `Regex::is_match` 语义，不隐含整串锚定。

### 6.3 集合校验

`CollectionArgument` 为 `Vec<T>`、`&[T]` 和数组实现，成功后返回原值或原引用：

- `require_non_empty`
- `require_len`
- `require_len_at_least`
- `require_len_at_most`
- `require_len_in`

非法长度区间返回 `InvalidLengthConstraint`。

### 6.4 Option 校验

Java `null` 术语全部删除。`OptionArgument<T>` 提供：

- `require_some(path: &str) -> ArgumentResult<T>`
- `validate_if_some(validator) -> ArgumentResult<Self>`

`validate_if_some` 的 validator 签名为 `FnOnce(&T) -> ArgumentResult<()>`。它只做校验，不承担转换；`None` 直接通过且不执行 validator，`Some` 成功后返回原来的 `Option<T>`，不 clone 内部值。

### 6.5 自定义参数与边界校验

提供 `require_that` 作为自定义谓词入口：

```rust
pub fn require_that<T, F>(
    value: T,
    path: &str,
    predicate: F,
    code: &str,
    message: &str,
) -> ArgumentResult<T>
where
    F: FnOnce(&T) -> bool;
```

谓词成功时返回原值；失败时复制 `path`、`code` 和 `message`，构造 `Custom` 错误。调用方需要动态格式化消息时，应在调用前构造消息并以 `&str` 传入；首版不为这个低频扩展点增加第二套惰性闭包 API。

保留并改造以下边界函数，全部接受参数路径：

- `check_bounds`
- `check_element_index`
- `check_position_index`
- `check_position_range`

边界计算使用先比较后相减或 checked arithmetic，禁止 `offset + length` 的未检查溢出。

`check_bounds` 返回 `ArgumentResult<()>`；两个单索引函数返回校验后的 `usize`；`check_position_range` 返回校验后的 `Range<usize>`。

删除：

- `check_argument_fmt`
- `check_state*`
- `require_null_or`
- `require_element_non_null`
- `require_equal`
- `require_not_equal`
- 四套固定开闭区间方法
- 所有兼容 alias 和 deprecated wrapper

## 7. 模块与文件组织

保留现有 `src/argument/` 目录并将其设为私有模块。新增：

- `src/argument/argument_path.rs`
- `src/argument/argument_value.rs`
- `src/argument/constraint.rs`

将 `src/argument/condition.rs` 重命名为 `src/argument/bounds.rs`。对应测试文件同步重命名。

其余文件原地重写：

- `argument_error.rs`
- `numeric_argument.rs`
- `string_argument.rs`
- `collection_argument.rs`
- `option_argument.rs`
- `mod.rs`
- `lib.rs`

每个具体 Rust 文件显式导入直接依赖，不使用 `use super::*`。`mod.rs` 只负责模块声明和重导出。公共和私有函数均按项目 Rust 注释规范编写英文文档。

测试全部放在 `tests/`，路径与源码对应，测试函数使用 `test_{method}_{scenario}` 命名。

## 8. Cargo 与版本

crate 版本由 `0.3.6` 升至 `0.4.0`。

```toml
[features]
default = []
regex = ["dep:regex"]

[dependencies]
regex = { version = "1.12", optional = true }
```

核心实现不新增 `thiserror`。由于错误结构需要统一、自定义的 `Display`，且依赖最小化是已确认目标，`ArgumentError` 手动实现标准错误 trait。

## 9. 测试设计

测试重写为行为测试，删除重复覆盖同一分支的用例。

### 9.1 错误模型

- 精确验证 `path`、`kind`、`into_parts`。
- 验证 `Debug`、单行转义的 `Display` 和 `Error`。
- 编译期验证 `Send + Sync + 'static`。
- 验证 `Clone + PartialEq + Eq`。
- 验证字符串实际内容不会出现在错误输出中。

### 9.2 模拟下游

- 在集成测试中定义 `DomainError`。
- 实现透明的 `From<ArgumentError>` 并验证 `?` 自动转换。
- 实现根据 `path + kind` 转成强类型领域 variant 的 `From` 并验证 `?`。

### 9.3 数值

- 覆盖全部原生整数类型以及 `f32`、`f64`。
- 覆盖零、符号、比较和边界值。
- 覆盖有界、无界及不同开闭组合的 `RangeBounds`。
- 覆盖反向区间、结构性空区间、NaN、无穷值和负零。

### 9.4 字符串、集合和 Option

- 覆盖空串、Unicode 空白、UTF-8 多字节字符和字符数量。
- 精确区分字节长度、Unicode 标量值数量和集合元素数量的 `LengthMetric`。
- 验证 owned 与 borrowed 输入保持原类型。
- 覆盖集合长度边界和非法长度约束。
- 使用不可 `Clone` 类型验证 `Option` 不发生隐式 clone。
- 验证 `None` 不执行 validator。

### 9.5 bounds 与 feature

- 覆盖元素索引、位置索引、位置范围和切片边界。
- 覆盖 `usize::MAX`，证明边界检查不溢出。
- 默认关闭 regex 时运行核心测试。
- 开启全部 feature 时运行正则测试。
- README 与 rustdoc 示例在对应 feature 下可编译。

## 10. 文档

同步重写：

- `README.md`
- `README.zh_CN.md`
- crate 和模块级 rustdoc
- `src/argument/README.md`
- `src/argument/README.zh_CN.md`

文档重点说明结构化错误、下游 `From` 转换、恢复与 panic 的选择、所有权保持、字符串长度语义和 regex feature。

## 11. 实施与验证顺序

实现采用测试先行：先写会失败的行为测试，再完成最小实现并重构。

实施过程中运行针对性测试，并至少验证：

```bash
cargo test --no-default-features
cargo test --all-features
```

最终严格按用户要求执行：

```bash
./align-ci.sh
./ci-check.sh
```

`align-ci.sh` 产生的项目对齐改动属于本次范围。`ci-check.sh` 失败时修复本次重构引入的问题并重新运行，直至完整通过或报告无法在本地解决的外部阻塞。

## 12. 成功标准

- 所有批准的新 API 可从 crate 根导入，内部模块不可从外部访问。
- `ArgumentError` 提供完整结构、标准错误 trait 和安全日志输出。
- 模拟下游可以通过 `From<ArgumentError>` 与 `?` 完成透明或强类型转换。
- owned 输入在成功校验后保持所有权，不需要 clone。
- 默认构建不编译 regex，开启 feature 后正则 API 可用。
- 不存在兼容旧 API 的公开 wrapper。
- 新测试无明显重复，覆盖正常、错误和边界行为。
- `align-ci.sh` 和 `ci-check.sh` 完整通过。
- 本轮不修改任何真实下游仓库。
