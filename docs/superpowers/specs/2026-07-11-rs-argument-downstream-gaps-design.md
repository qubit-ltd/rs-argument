# rs-argument 下游驱动能力补全设计

日期：2026-07-11

## 1. 背景

`qubit-argument` 0.4 已能覆盖字符串、原生数值、集合、`Option`、索引和
切片边界，并能通过 `From<ArgumentError>` 与 `?` 接入下游领域错误。
对 `rs-event-bus`、执行器家族、`rs-http` 和 `rs-retry` 的实际校验代码进行
复核后，剩余阻碍集中在嵌套路径、`Duration` 和非有限浮点值三类。

本轮只修改 `rs-argument`。真实下游代码保持不变；其用法在集成测试中以最小
消费端模型复现，作为公共 API 实用性的验收标准。

## 2. 目标

- 嵌套验证失败时，仅在错误路径上追加前缀，成功路径不分配路径字符串。
- 结构化表达 `std::time::Duration` 的实际值和比较边界。
- 显式拒绝正负无穷；NaN 继续使用现有 `NotANumber` 语义。
- 用事件总线、线程池、HTTP 嵌套配置和重试配置场景验证 `?` 传播及领域错误转换。

## 3. 非目标

- 不增加多错误聚合、derive、过程宏或验证上下文对象。
- 不为任意领域 newtype 开放可扩展的值注册机制。
- 不添加未经当前下游证明需要的字符串或容器实现。
- 不把解析错误、I/O 错误、状态错误或内部不变量改成 `ArgumentError`。
- 不修改任何 `rs-*` 下游仓库。

## 4. 公共 API

### 4.1 嵌套路径

新增以下消费式方法：

```rust
impl ArgumentPath {
    pub fn with_prefix(self, prefix: &str) -> Self;
}

impl ArgumentError {
    pub fn with_path_prefix(self, prefix: &str) -> Self;
}

pub trait ArgumentResultExt<T> {
    fn with_path_prefix(self, prefix: &str) -> ArgumentResult<T>;
}
```

非空前缀与非空原路径之间使用 `.`。任一侧为空时不产生多余分隔符。
`ArgumentResultExt` 对 `Ok` 原样返回，只有 `Err` 才组合路径并分配字符串。

### 4.2 Duration

`ArgumentValue` 增加 `Duration(std::time::Duration)`，使错误保留秒和纳秒精度，
不把持续时间降级为缺少单位的整数。

新增 `DurationArgument`，只提供当前下游实际需要的正值和跨字段比较：

```rust
pub trait DurationArgument: Sized {
    fn require_positive(self, path: &str) -> ArgumentResult<Self>;
    fn require_less_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_at_most(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_greater_than(self, path: &str, bound: Self) -> ArgumentResult<Self>;
    fn require_at_least(self, path: &str, bound: Self) -> ArgumentResult<Self>;
}
```

该 trait 只为 `Duration` 实现，失败继续复用 `ArgumentErrorKind::Comparison` 和
`ComparisonConstraint`。`require_positive` 等价于严格大于 `Duration::ZERO`。

### 4.3 浮点有限性

新增仅为 `f32`、`f64` 实现的 `FloatArgument`：

```rust
pub trait FloatArgument: Sized {
    fn require_finite(self, path: &str) -> ArgumentResult<Self>;
}
```

NaN 返回 `ArgumentErrorKind::NotANumber`；正负无穷返回新增的
`ArgumentErrorKind::NotFinite { actual }`，其中 `actual` 保留原浮点位模式。
有限值原样返回，可继续链式调用 `NumericArgument`。

## 5. 下游场景测试

- `rs-event-bus`：拥有所有权的 topic 名称校验后，通过 `From<ArgumentError>` 和
  `?` 进入模拟领域错误；该场景确认现有字符串 API 无需扩展。
- 执行器家族：最大线程数、核心线程数、可选队列容量、可选栈大小和 keep-alive
  组成一个模拟 builder；该场景验证数值、现有 `Option` API 和 `Duration` 可以组合。
- `rs-http`：子配置先返回局部路径，再通过 `with_path_prefix("timeouts")` 形成
  `timeouts.connect`；该场景验证成功路径透明和失败路径组合。
- `rs-retry`：initial/max 使用 `Duration` 比较，multiplier 先要求有限再要求大于
  1.0；该场景验证链式校验及结构化错误。

## 6. 文档与兼容性

所有新增类型从 crate 根导出。README、crate rustdoc 和 argument guide 同步说明
新增能力，但不把 `rs-argument` 描述成通用领域验证框架。

发布前封闭所有校验扩展 trait，只允许 crate 内维护文档明确列出的实现。这避免
下游实现因可选 feature 新增必需方法或内部语义演进而失效。`ArgumentValue` 与
`LengthMetric` 标记为非穷尽；它们与既有的 `ArgumentErrorKind` 一样要求下游
匹配保留通配分支，从而允许后续兼容地增加结构化值类型和长度度量。

## 7. 验证

每项能力按测试先行独立验证。最终依次执行：

```bash
./align-ci.sh
./ci-check.sh
```

成功标准是消费端场景不需要 `map_err` 来完成领域错误转换；嵌套路径只使用专用
结果扩展方法；结构化错误精确保留路径、持续时间、浮点值和比较约束。
