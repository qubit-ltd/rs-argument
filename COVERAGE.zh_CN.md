# 代码覆盖率统计指南

本项目使用 `cargo-llvm-cov` 进行代码覆盖率统计。

## 安装依赖

如果还没有安装 `cargo-llvm-cov`，请先安装：

```bash
cargo install cargo-llvm-cov
```

## 快速开始

### 使用便捷脚本（推荐）

我们提供了一个便捷脚本 `.infra/bin/coverage.sh`，可以快速生成各种格式的覆盖率报告：

```bash
# 生成 HTML 报告并在浏览器中打开（默认）
./.infra/bin/coverage.sh

# 或指定格式
./.infra/bin/coverage.sh html       # HTML 报告（在浏览器中打开）
./.infra/bin/coverage.sh text       # 终端文本报告
./.infra/bin/coverage.sh lcov       # LCOV 格式
./.infra/bin/coverage.sh json       # JSON 格式
./.infra/bin/coverage.sh cobertura  # Cobertura XML 格式
./.infra/bin/coverage.sh all        # 生成所有格式

# 查看帮助
./.infra/bin/coverage.sh help
```

### 使用 cargo 命令

你也可以直接使用 `cargo llvm-cov` 命令：

```bash
# 清理旧的覆盖率数据
cargo llvm-cov clean

# 生成 HTML 报告并在浏览器中打开
cargo llvm-cov --html --open

# 生成文本格式报告（输出到终端）
cargo llvm-cov

# 生成 LCOV 格式报告
cargo llvm-cov --lcov --output-path target/llvm-cov/lcov.info

# 生成 JSON 格式报告
cargo llvm-cov --json --output-path target/llvm-cov/coverage.json

# 生成 Cobertura XML 格式报告
cargo llvm-cov --cobertura --output-path target/llvm-cov/cobertura.xml
```

## 报告位置

生成的报告默认保存在以下位置：

- **HTML 报告**: `target/llvm-cov/html/index.html`
- **LCOV 报告**: `target/llvm-cov/lcov.info`
- **JSON 报告**: `target/llvm-cov/coverage.json`
- **Cobertura 报告**: `target/llvm-cov/cobertura.xml`

## 只测试特定模块

如果只想测试特定的模块，可以使用：

```bash
# 只测试 argument 模块
cargo llvm-cov --html --open -- argument::

# 只测试特定的测试文件
cargo llvm-cov --html --open --test argument_tests -- argument
```

## 排除特定文件

在 `.llvm-cov.toml` 配置文件中，我们已经排除了以下文件：

- `tests/*` - 测试文件
- `benches/*` - 性能测试文件
- `examples/*` - 示例文件

如果需要修改排除规则，请编辑 `.llvm-cov.toml` 文件。

## CI/CD 集成

本项目通过 [Rust CI](.github/workflows/ci.yml) 接入持续集成。
其中 `rust-ci` 任务调用共享工作流
`qubit-ltd/rs-infra-ci/.github/workflows/github-ci.yml@main`。
当前 CI 配置请以该入口文件为准。

需要在本地调用项目覆盖率工具时，请从仓库根目录执行：

```bash
./.infra/bin/coverage.sh
```

## 常见问题

### 1. 找不到 `cargo-llvm-cov` 命令

确保已经安装了 `cargo-llvm-cov`：

```bash
cargo install cargo-llvm-cov
```

### 2. 覆盖率数据不准确

先清理旧的覆盖率数据：

```bash
cargo llvm-cov clean
```

### 3. 如何提高覆盖率？

- 为所有公共 API 编写测试
- 测试边界条件和异常情况
- 使用覆盖率报告识别未测试的代码路径
- 为复杂的逻辑分支编写测试

## 覆盖率目标

我们建议的覆盖率目标：

- **最低要求**: 60%
- **良好**: 75%
- **优秀**: 85%+
- **核心模块**: 90%+

## 参考资料

- [cargo-llvm-cov GitHub](https://github.com/taiki-e/cargo-llvm-cov)
- [LLVM Coverage Mapping](https://llvm.org/docs/CoverageMappingFormat.html)
