# Woof 256-Core 扩展性优化路线图

> 从 54 条规则、29% 效率 到 400+ 规则、65% 效率的完整实施路径

## 执行摘要

当前 woof 在 256 核下的扩展性受限（29% 效率），主要原因：
1. **规则数量过少**（54 条）导致单文件处理时间太短（~2.5ms）
2. **固定开销占比过高**（60%）限制了并行加速比

本路线图通过三阶段优化，实现 256 核 65% 效率的目标。

---

## 当前瓶颈分析

### 单文件处理时间拆解

```
当前 54 条规则 (2.5ms/文件):
┌────────────────────────────────────────────────────────┐
│  文件读取 (标准 I/O)          0.5ms    20%    [固定]   │
│  tree-sitter 解析             1.0ms    40%    [固定]   │
│  54 条规则执行                1.0ms    40%    [可并行] │
└────────────────────────────────────────────────────────┘
                           总计 2.5ms
                           可并行占比: 40%
                           256核理论效率: 29%
```

### 目标状态

```
目标 400+ 条规则 (50ms/文件):
┌────────────────────────────────────────────────────────┐
│  内存映射 I/O                 0.1ms     0.2%   [固定]  │
│  增量 AST 解析                0.2ms     0.4%   [固定]  │
│  基础规则 (200条)             4.0ms     8.0%   [可并行]│
│  SSA 分析 (100条)             10.0ms   20.0%   [可并行]│
│  程序间分析 (100条)           35.7ms   71.4%   [可并行]│
└────────────────────────────────────────────────────────┘
                           总计 50ms
                           可并行占比: 97%
                           256核理论效率: 65%
```

---

## 阶段一：基础优化（1-2 个月）

**目标**：减少固定开销，将 256 核效率从 29% 提升到 45%

### 1.1 内存映射 I/O 优化

**当前问题**
```rust
// src/io/mod.rs - 当前实现
pub fn read_file(path: &Path) -> Result<String> {
    std::fs::read_to_string(path)?  // ~0.5ms, 涉及内核拷贝
}
```

**优化方案**
```rust
// 已完成部分实现，需完善以下功能：
pub enum ZeroCopySource {
    Mmap(Arc<Mmap>),           // >1MB 文件
    Cached(Arc<String>),       // 小文件缓存
    Static(&'static str),
}

impl ZeroCopySource {
    pub fn as_str(&self) -> Result<&str> {
        match self {
            Self::Mmap(m) => std::str::from_utf8(m),
            Self::Cached(s) => Ok(s.as_str()),
            Self::Static(s) => Ok(s),
        }
    }
}
```

**预期收益**
- 文件读取: 0.5ms → 0.1ms (5x)
- 大文件内存占用减少 50%

### 1.2 AST 缓存系统

**实现方案**
```rust
// src/parser/cache.rs
pub struct AstCache {
    // LRU 缓存，按 (inode, mtime, size) 索引
    cache: RwLock<LruCache<FileId, Arc<ParseTree>>>,
    max_entries: usize,
}

impl AstCache {
    pub fn get_or_parse(&self, path: &Path, source: &str) -> Arc<ParseTree> {
        let file_id = FileId::from_path(path);
        
        // 快速路径：缓存命中
        if let Some(cached) = self.cache.read().get(&file_id) {
            return cached.clone();
        }
        
        // 慢速路径：解析并缓存
        let tree = self.parse(source);
        self.cache.write().put(file_id, tree.clone());
        tree
    }
}
```

**关键优化点**
- 缓存键使用文件元数据（inode + mtime），而非路径
- 支持增量解析：只重新解析变更的函数
- 内存限制：最多 1000 个文件（约 200MB）

**预期收益**
- 热运行解析: 1.0ms → 0.2ms (5x)
- CI 场景缓存命中率: >80%

### 1.3 阶段一成果

| 指标 | 优化前 | 优化后 | 提升 |
|------|--------|--------|------|
| 单文件时间 | 2.5ms | 1.0ms | 2.5x |
| 固定开销占比 | 60% | 30% | 2x |
| **256核效率** | **29%** | **45%** | +55% |
| 100K 文件处理 | ~12s | ~6s | 2x |

---

## 阶段二：规则扩展（3-6 个月）

**目标**：增加规则到 200+ 条，提升绝对检测能力

### 2.1 SSA 基础分析框架

**为什么需要 SSA**
```go
// 示例：无效赋值检测需要 SSA
func example() {
    x := 1
    x = 2      // 此行之前 x 的赋值无效
    println(x) // 只使用最后的值
}
```

**实现方案**
```rust
// src/ssa/mod.rs
pub struct SsaBuilder {
    variables: HashMap<NodeId, Variable>,
    assignments: Vec<Assignment>,
    uses: HashMap<Variable, Vec<Use>>,
}

impl SsaBuilder {
    pub fn from_ast(tree: &ParseTree) -> SsaGraph {
        // 1. 构建控制流图
        // 2. 插入 φ 函数
        // 3. 重命名变量
        // 4. 构建使用-定义链
    }
}

// SSA 支持的规则
pub struct DeadStoreDetection;
pub struct UnusedVariableDetection;
pub struct RedundantAssignmentDetection;
```

**新增规则清单（30-40 条）**

| 规则代码 | 名称 | 描述 | 复杂度 |
|----------|------|------|--------|
| F020 | dead-store | 无效赋值检测 | SSA |
| F021 | unused-variable | 未使用变量 | SSA |
| F022 | unread-variable | 只写不读变量 | SSA |
| F023 | shadow-variable | 变量遮蔽 | SSA |
| ... | ... | ... | ... |

**预期收益**
- 新增规则: 30-40 条
- 单文件耗时增加: +2-3ms
- 检测能力提升: 覆盖 80% staticcheck 功能

### 2.2 安全规则（gosec 级）

**实现方案**
```rust
// src/rules/security.rs
pub struct SqlInjectionCheck;
pub struct PathTraversalCheck;
pub struct InsecureDeserializeCheck;
pub struct WeakCryptoCheck;

impl Rule for SqlInjectionCheck {
    fn check(&self, node: Node, source: &str, file: &str) -> Vec<Diagnostic> {
        // 检测 SQL 查询字符串拼接
        // 模式: db.Query("SELECT * FROM " + userInput)
    }
}
```

**新增规则清单（20-30 条）**

| 类别 | 规则数 | 示例 |
|------|--------|------|
| SQL 注入 | 5 | G201-G202 |
| 命令注入 | 3 | G204 |
| 路径遍历 | 3 | G304-G305 |
| 不安全反序列化 | 2 | G405-G406 |
| 弱加密 | 4 | G401-G403 |
| 硬编码凭证 | 3 | G101-G102 |
| 其他 | 10+ | ... |

**预期收益**
- 新增规则: 20-30 条
- 单文件耗时增加: +1-2ms
- 安全检测: 对齐 gosec

### 2.3 性能规则

**新增规则清单（15-20 条）**

| 规则代码 | 名称 | 描述 |
|----------|------|------|
| P001 | preallocate-slice | 建议预分配切片容量 |
| P002 | string-builder | 循环内字符串拼接优化 |
| P003 | unnecessary-allocation | 避免不必要的内存分配 |
| P004 | inefficient-map-clear | map 清空方式优化 |
| ... | ... | ... |

### 2.4 阶段二成果

| 指标 | 阶段一后 | 阶段二后 | 提升 |
|------|----------|----------|------|
| 规则总数 | 54 | **200+** | 3.7x |
| 单文件时间 | 1.0ms | **4.0ms** | 4x |
| 可并行占比 | 70% | **80%** | +14% |
| **256核效率** | **45%** | **48%** | +7% |
| 检测能力 | 基础 | 全面 | - |

---

## 阶段三：程序间分析（6-12 个月）

**目标**：实现跨文件分析，将 256 核效率提升到 65%

### 3.1 全局符号索引

**架构设计**
```rust
// src/interprocedural/symbols.rs
pub struct GlobalSymbolIndex {
    // 包级别的类型、函数、接口索引
    packages: HashMap<PackageId, PackageSymbols>,
    
    // 接口实现关系
    interface_impls: HashMap<InterfaceId, Vec<TypeId>>,
    
    // 函数调用图
    call_graph: DiGraph<FunctionId, ()>,
}

impl GlobalSymbolIndex {
    pub fn build(project: &Project) -> Self {
        // 1. 扫描所有包的导出符号
        // 2. 构建接口实现表
        // 3. 构建调用图
    }
    
    pub fn find_interface_impls(&self, iface: &InterfaceId) -> &[TypeId] {
        &self.interface_impls[iface]
    }
}
```

**应用场景**
```go
// 跨文件接口一致性检查
// 如果 InterfaceA 增加了方法 MethodB，
// 需要确保所有实现 InterfaceA 的类型都有 MethodB
```

### 3.2 全局死代码检测

**算法**
```rust
// src/interprocedural/deadcode.rs
pub fn find_dead_code(index: &GlobalSymbolIndex) -> Vec<Diagnostic> {
    // 1. 从 main() 和 init() 开始标记可达函数
    // 2. 遍历调用图，标记所有可达函数
    // 3. 未被标记的导出函数报告为死代码
    
    let mut reachable = HashSet::new();
    let mut queue = VecDeque::new();
    
    // 入口点
    for entry in index.entry_points() {
        queue.push_back(entry);
        reachable.insert(entry);
    }
    
    // BFS 遍历
    while let Some(func) = queue.pop_front() {
        for callee in index.call_graph.callees(func) {
            if reachable.insert(callee) {
                queue.push_back(callee);
            }
        }
    }
    
    // 收集死代码
    index.all_functions()
        .filter(|f| !reachable.contains(f) && is_exported(f))
        .map(|f| Diagnostic::dead_code(f))
        .collect()
}
```

### 3.3 包依赖分析

**循环依赖检测**
```rust
pub fn detect_circular_deps(index: &GlobalSymbolIndex) -> Vec<Diagnostic> {
    // 使用 Tarjan 算法找强连通分量
    let sccs = tarjan_scc(&index.import_graph);
    
    sccs.iter()
        .filter(|scc| scc.len() > 1)
        .map(|scc| Diagnostic::circular_dependency(scc))
        .collect()
}
```

### 3.4 阶段三成果

| 指标 | 阶段二后 | 阶段三后 | 提升 |
|------|----------|----------|------|
| 分析类型 | 单文件 | **跨文件** | - |
| 单文件时间 | 4.0ms | **50ms** | 12.5x |
| 可并行占比 | 80% | **97%** | +21% |
| **256核效率** | **48%** | **65%** | +35% |
| 检测能力 | 语法/语义 | **全局优化** | - |

---

## 实施时间表

### 里程碑规划

```
Month 1-2:   [阶段一] 基础优化
             ├── mmap I/O 完善
             ├── AST 缓存实现
             └── 性能测试验证
             
Month 3-4:   [阶段二早期] SSA 框架
             ├── SSA IR 设计
             ├── 基础数据流分析
             └── 10-15 条 SSA 规则
             
Month 5-6:   [阶段二中期] 规则扩展
             ├── 剩余 SSA 规则
             ├── 安全规则 (gosec 级)
             └── 性能规则
             
Month 7-9:   [阶段二后期] 完善
             ├── 代码风格规则
             ├── 规则配置系统
             └── 回归测试
             
Month 10-12: [阶段三] 程序间分析
             ├── 全局符号索引
             ├── 全局死代码检测
             └── 包依赖分析
```

### 资源需求

| 阶段 | 人力 | 关键技能 | 风险 |
|------|------|----------|------|
| 阶段一 | 1 人 | 系统编程、I/O 优化 | 低 |
| 阶段二 | 2 人 | 编译器原理、SSA | 中 |
| 阶段三 | 2-3 人 | 静态分析、图算法 | 高 |

---

## 性能预测汇总

### 各阶段 256 核性能对比

| 阶段 | 规则数 | 单文件时间 | 可并行占比 | 256核效率 | 100K文件时间 |
|------|--------|-----------|-----------|-----------|-------------|
| 当前 | 54 | 2.5ms | 40% | **29%** | ~12s |
| 阶段一 | 54 | 1.0ms | 70% | **45%** | ~6s |
| 阶段二 | 200+ | 4.0ms | 80% | **48%** | ~18s |
| 阶段三 | 400+ | 50ms | 97% | **65%** | ~140s |

**说明**: 
- 阶段二虽然绝对时间增加，但检测能力大幅提升
- 阶段三适用于深度分析场景（如 IDE、代码审查）

### 与 golangci-lint 对比

| 工具 | 规则数 | 100K文件时间 | 256核效率 | 相对速度 |
|------|--------|-------------|-----------|----------|
| golangci-lint | 400+ | ~15 分钟 | 70% | 1x |
| woof (当前) | 54 | ~12 秒 | 29% | **75x** |
| woof (阶段一) | 54 | ~6 秒 | 45% | **150x** |
| woof (阶段二) | 200+ | ~18 秒 | 48% | **50x** |
| woof (阶段三) | 400+ | ~2.5 分钟 | 65% | **6x** |

---

## 风险评估与缓解

### 主要风险

| 风险 | 影响 | 概率 | 缓解措施 |
|------|------|------|----------|
| SSA 实现复杂度过高 | 阶段二延期 | 中 | 先实现简化版本，逐步完善 |
| 内存占用过高 | OOM | 中 | 限制缓存大小，LRU 淘汰 |
| 误报率上升 | 用户体验下降 | 高 | 严格的测试集，可配置规则 |
| 单文件性能下降 | 小项目体验差 | 中 | 自适应模式（小项目用快速模式） |

### 回退方案

如果阶段三过于复杂：
1. 专注于阶段二，将规则扩展到 400+ 条
2. 使用更简单的程序间分析（如仅接口一致性检查）
3. 保持 50% 效率目标，而非 65%

---

## 监控指标

### 关键性能指标 (KPI)

```rust
// src/metrics.rs
pub struct ScalingMetrics {
    // 扩展性指标
    pub speedup_256: f64,           // 256核加速比
    pub efficiency_256: f64,        // 256核效率 (%)
    
    // 性能指标
    pub time_per_file_ms: f64,      // 单文件处理时间
    pub throughput_files_per_sec: f64, // 吞吐量
    
    // 资源指标
    pub memory_peak_mb: usize,      // 峰值内存
    pub cache_hit_rate: f64,        // 缓存命中率
}
```

### 每周监控报告

```bash
# 自动化性能测试
woof benchmark --cores 1,16,32,64,128,256 --files 100000

# 输出格式
# Week 12 Report:
# - 256核效率: 45% (↑ 5% from last week)
# - 单文件时间: 1.1ms (target: 1.0ms)
# - Cache 命中率: 78%
```

---

## 结论

通过三阶段优化，woof 将在 12 个月内实现：

1. **阶段一（1-2月）**：256核效率从 29% → 45%
2. **阶段二（3-6月）**：规则数 54 → 200+，检测能力全面
3. **阶段三（6-12月）**：256核效率从 48% → 65%，支持程序间分析

**最终状态**：
- 规则数量：400+ 条（覆盖 90% golangci-lint 功能）
- 256核效率：65%
- 100K 文件处理：2.5 分钟（仍比 golangci-lint 快 6x）
- 检测能力：支持全局死代码、接口一致性等深度分析

---

*文档版本: 1.0*  
*最后更新: 2026-03-21*  
*作者: woof 开发团队*
