# rustfem 开发计划（PLAN.md）

> 说明：当前仓库 README 仅包含项目标题，缺少正式文档。本计划基于现有代码（`main.rs` 中可运行的 1D truss 示例、以及 `src/element`、`src/problem`、`src/approximation.rs`、`src/operations.rs` 等框架代码）整理出可行的演进路线。

## 1. 项目目标与定位

`rustfem` 是一个用 Rust 编写的通用有限元方法（FEM）库，目标是为结构力学/连续介质力学问题提供：

- 模块化的单元（Element）与近似（Approximation）抽象；
- 可扩展的双线性/线性形式（BilinearForm / LinearForm）组装；
- 基于稀疏矩阵（`faer`）的线性系统求解；
- 1D/2D/3D 桁架、梁、弹性体等常见问题的示例；
- 清晰的 API、完整文档与回归测试。

## 2. 里程碑（Milestones）

| 里程碑 | 目标 | 主要交付物 | 验收标准 |
|--------|------|------------|----------|
| **M1：基础设施与代码整理** | 统一现有代码结构，建立可维护的模块边界 | 模块合并、统一 `Node`/`Element` 定义、可编译通过的库入口、基础测试、CI 配置 | `cargo test`、`cargo clippy` 零警告，GitHub Actions 通过 |
| **M2：1D 桁架完整求解链路** | 把 `main.rs` 中的示例提炼为库 API，支持边界条件、载荷与求解 | `Truss1D` 问题类型、Dirichlet/Neumann 边界条件、稀疏矩阵组装与求解器封装、示例程序 | 1D 杆端受拉示例与当前 `main.rs` 结果一致，并可通过 API 调用 |
| **M3：2D/3D 单元支持** | 引入常见低阶单元 | `Seg2`（1D/2D/3D）、`Tri3`、`Quad4` 单元，统一的形函数与雅可比接口 | 单元测试覆盖形函数、导数、雅可比计算 |
| **M4：材料与物理问题扩展** | 从桁架扩展到更多物理模型 | 线弹性材料（`LinearElastic`）、2D/3D 泊松/拉普拉斯问题、梁单元骨架 | 至少新增 2 个可运行的示例 |
| **M5：求解器与性能优化** | 完善稀疏矩阵求解、性能剖析与可配置性 | 求解器 trait、直接/迭代法切换、稀疏 feature 优化、可选 `dhat` 性能分析 | 中等规模网格（>1e4 DOF）可在 release 下 5 秒内完成组装+求解 |
| **M6：文档、示例与发布** | 完成用户文档并准备 crates.io 发布 | 完整 README、API 文档、教程示例、CHANGELOG、版本号升级 | docs.rs 可编译，示例可一键运行，通过 `cargo publish --dry-run` |

## 3. 任务分解与时间线（建议）

每个里程碑预计 1–2 周，视投入时间而定。建议按以下顺序推进：

### M1：基础设施与代码整理（第 1 周）

- [ ] 合并 `node.rs` 与 `approximation.rs` 中重复的 `Node` 定义；
- [ ] 明确 `Element` trait 与 `Approximation` trait 的关系，合并或重命名为单一接口；
- [ ] 整理 `src/element/mod.rs`、`src/element/seg2.rs`，使 `main.rs` 中的示例能引用库模块；
- [ ] 将 `main.rs` 中的实现迁移到 `src/` 子模块，仅保留调用示例；
- [ ] 添加 `cargo test` 可运行的最小单元测试；
- [ ] 配置 `.github/workflows/ci.yml`（fmt + clippy + test）。

### M2：1D 桁架完整求解链路（第 2 周）

- [ ] 定义 `Problem` trait（几何、材料、边界、载荷、求解、后处理）；
- [ ] 实现 `Truss1D` 问题类型；
- [ ] 抽象 `BoundaryCondition`（Dirichlet / Neumann）与 `Load`；
- [ ] 封装稀疏矩阵求解器（默认 `faer` Cholesky/LU）；
- [ ] 用新 API 重写 `examples/truss1d.rs` 示例；
- [ ] 与当前 `main.rs` 结果做对比测试。

### M3：2D/3D 单元支持（第 3–4 周）

- [ ] 定义通用 `Quadrature` trait（当前仅 `GaussSeg2`）；
- [ ] 实现 `Gauss` 一维、二维、三维积分规则；
- [ ] 实现 `Seg2<2>` / `Seg2<3>` 作为边界/桁架单元；
- [ ] 实现 `Tri3`、`Quad4` 单元及形函数、导数、雅可比；
- [ ] 为每个单元编写单元测试。

### M4：材料与物理问题扩展（第 5–6 周）

- [ ] 引入 `Material` trait（`Truss`、`LinearElastic`）；
- [ ] 实现 2D 平面应力/平面应变问题；
- [ ] 实现 3D 线弹性问题骨架；
- [ ] 添加 `examples/poisson2d.rs`、`examples/elasticity2d.rs`。

### M5：求解器与性能优化（第 7–8 周）

- [ ] 抽象 `Solver` trait（`src/solver.rs` 目前为空）；
- [ ] 支持直接法（Cholesky/LU）与迭代法占位；
- [ ] 优化 `sparse` feature 下的组装逻辑；
- [ ] 使用 `dhat` 进行堆分配分析，减少冗余分配；
- [ ] 添加中等规模网格基准测试。

### M6：文档、示例与发布（第 9–10 周）

- [ ] 重写 `README.md`（简介、安装、快速开始、API 概览、示例链接）；
- [ ] 补齐所有公共 API 的 rustdoc；
- [ ] 编写 `docs/element.md`、`docs/solver.md` 等设计文档；
- [ ] 维护 `CHANGELOG.md`；
- [ ] 升级到 `0.2.0` 并执行 `cargo publish --dry-run`。

## 4. Git Commit 规范化

本项目采用 [Conventional Commits](https://www.conventionalcommits.org/) 作为提交信息规范，便于自动生成 CHANGELOG 和版本号决策。

### 4.1 提交信息格式

```text
<type>(<scope>): <subject>

<body>

<footer>
```

- **type**（必填）：提交类别，见下表；
- **scope**（可选）：模块或功能范围，如 `element`、`solver`、`truss1d`、`ci`；
- **subject**（必填）：简短描述，使用祈使句，首字母小写，结尾不加句号；
- **body**（可选）：详细说明动机、实现细节、设计取舍；
- **footer**（可选）：关联 issue、破坏性变更说明等。

### 4.2 类型说明

| 类型 | 含义 | 示例 |
|------|------|------|
| `feat` | 新功能 | `feat(element): add Tri3 shape functions` |
| `fix` | 修复 bug | `fix(solver): correct boundary condition indexing` |
| `docs` | 文档变更（README、rustdoc、设计文档） | `docs: add quickstart guide` |
| `style` | 代码格式调整（不影响逻辑） | `style: apply rustfmt` |
| `refactor` | 重构（不新增功能也不修复 bug） | `refactor(node): unify Node definitions` |
| `perf` | 性能优化 | `perf(sparse): reduce triplet allocations` |
| `test` | 测试相关 | `test(truss1d): add regression test` |
| `chore` | 构建/工具/杂项 | `chore(ci): add GitHub Actions workflow` |
| `build` | 构建系统、依赖变更 | `build: bump faer to 0.25` |
| `ci` | CI/CD 配置 | `ci: run clippy on PR` |
| `revert` | 回滚提交 | `revert: feat(element): add Tri3 shape functions` |

### 4.3 破坏性变更

若提交引入不兼容变更，在 `type` 后加 `!`，并在 `footer` 中说明迁移方式：

```text
feat(element)!: replace Element trait with Approximation

BREAKING CHANGE: The `Element<P, N>` trait is removed. Use `Approximation<DIM, NDOF>` instead.
```

### 4.4 示例提交

```text
feat(solver): wrap faer sparse Cholesky in Solver trait

- Add `Solver` trait with `solve` and `factorize` methods.
- Implement `FaerCholeskySolver` for symmetric positive definite matrices.
- Use `Side::Lower` as default fill-reducing ordering.

Refs: #12
```

### 4.5 推荐分支与合并策略

- 功能开发：`feat/<scope>-<description>`（如 `feat/element-tri3`）；
- Bug 修复：`fix/<scope>-<description>`；
- 文档：`docs/<description>`；
- 每个 PR/MR 尽量只对应一个 `type`，标题按规范书写；
- 合并前要求 CI 通过、至少一名 reviewer 通过。

## 5. 设计原则

- **最小化修改**：每次提交只解决一个明确问题；
- ** trait 优先**：用 Rust trait 抽象单元、材料、问题、求解器，方便扩展；
- **测试驱动**：新增物理模型或单元时，先写单元测试与回归测试；
- **文档同步**：每次 API 变更同步更新 rustdoc 与 README；
- **性能可测**：关键路径使用 `dhat` 或自定义 benchmark 验证。

## 6. 风险与依赖

| 风险 | 应对 |
|------|------|
| `faer` 版本升级可能带来 API 不兼容 | 锁定版本并在升级时单独提 `build` 类型提交 |
| 模块间 `Node`/`Element` 定义冲突 | 在 M1 中统一，避免后续重构成本 |
| 缺少文档导致协作困难 | 每个 milestone 结束时更新 README 与设计文档 |
| 性能瓶颈未及时发现 | M5 引入基准测试与性能剖析 |

## 7. 结论

本计划把 `rustfem` 从当前可运行的 1D 桁架原型，逐步演进为结构清晰、可扩展、文档完善的通用 FEM 库。每个里程碑都有明确的交付物和验收标准，建议按 M1 → M6 顺序推进，并严格遵守 Conventional Commits 规范。
