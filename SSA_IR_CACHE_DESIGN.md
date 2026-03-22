# Woof SSA/IR 缓存架构设计

## 当前架构 vs SSA/IR 缓存架构

### 当前架构（仅AST缓存）

```
Source Code
    │
    ▼
[Parser] ──► AST (tree-sitter::Tree)
    │
    ▼
[Rules] ──► AST遍历检查
    │
    ▼
Diagnostics

问题:
- 每个规则独立遍历AST
- 重复的类型推导
- 无控制流分析
- 复杂规则性能差
```

### SSA/IR 缓存架构

```
Source Code
    │
    ▼
[Parser] ──► AST (cached)
    │
    ▼
[SSA Builder] ──► SSA Form (cached) ⭐
    │
    ▼
[Analyzer] ──► IR with analysis results (cached) ⭐
    │
    ▼
[Rules] ──► 基于SSA/IR检查
    │
    ▼
Diagnostics

优势:
- 一次构建，多次使用
- 复用类型推导结果
- 控制流/数据流分析结果缓存
- 复杂规则性能大幅提升
```

## SSA/IR 缓存层次

### L1: AST Cache (已有)
```rust
struct AstCache {
    tree: Tree,
    source: Arc<String>,
}
// 命中: 避免重复解析
// 节省: ~5-10ms
```

### L2: SSA Cache (新增)
```rust
struct SsaCache {
    ssa: SsaForm,           // 静态单赋值形式
    control_flow: Cfg,      // 控制流图
    dominance: DomTree,     // 支配树
}
// 命中: 避免重复SSA构建
// 节省: ~10-20ms
```

### L3: Analysis Cache (新增)
```rust
struct AnalysisCache {
    types: TypeMap,         // 类型推导结果
    constants: ConstMap,    // 常量传播
    def_use: DefUseChains,  // 定义-使用链
    escape: EscapeAnalysis, // 逃逸分析
}
// 命中: 避免重复分析
// 节省: ~20-50ms
```

## 实现架构

```rust
// src/ssa/mod.rs

/// SSA缓存管理器
pub struct SsaCacheManager {
    cache: RwLock<LruCache<String, Arc<SsaProgram>>>,
}

impl SsaCacheManager {
    pub fn get_or_build(&self, ast: &Ast) -> Arc<SsaProgram> {
        let key = compute_hash(ast);
        
        // Try cache
        if let Some(ssa) = self.cache.read().get(&key) {
            return ssa.clone();
        }
        
        // Build SSA
        let ssa = Arc::new(SsaBuilder::new(ast).build());
        
        // Cache it
        self.cache.write().put(key, ssa.clone());
        
        ssa
    }
}

/// SSA程序表示
pub struct SsaProgram {
    pub functions: Vec<SsaFunction>,
    pub globals: Vec<Global>,
}

/// SSA函数
pub struct SsaFunction {
    pub name: String,
    pub blocks: Vec<BasicBlock>,
    pub params: Vec<Param>,
    pub return_ty: Type,
}

/// 基本块
pub struct BasicBlock {
    pub id: BlockId,
    pub instructions: Vec<Instruction>,
    pub terminator: Terminator,
}

/// SSA指令
pub enum Instruction {
    // 内存操作
    Load { ptr: Value, ty: Type },
    Store { ptr: Value, val: Value },
    Alloc { ty: Type },
    
    // 算术运算
    Add { lhs: Value, rhs: Value },
    Sub { lhs: Value, rhs: Value },
    Mul { lhs: Value, rhs: Value },
    Div { lhs: Value, rhs: Value },
    
    // 比较
    Eq { lhs: Value, rhs: Value },
    Lt { lhs: Value, rhs: Value },
    
    // 类型转换
    Cast { val: Value, ty: Type },
    
    // 调用
    Call { func: FuncId, args: Vec<Value> },
    
    // Phi节点（SSA关键）
    Phi { incoming: Vec<(BlockId, Value)> },
}

/// 值（SSA形式）
pub enum Value {
    Immediate(Immediate),
    Instruction(BlockId, InstId),
    Param(BlockId, ParamId),
    Global(GlobalId),
}
```

## SSA构建器

```rust
// src/ssa/builder.rs

pub struct SsaBuilder<'a> {
    ast: &'a Ast,
    program: SsaProgram,
    current_func: Option<FuncId>,
    current_block: Option<BlockId>,
    variables: HashMap<String, Value>, // 变量到SSA值的映射
    block_stack: Vec<BlockId>,         // 控制流块栈
}

impl<'a> SsaBuilder<'a> {
    pub fn build(mut self) -> SsaProgram {
        // 遍历AST构建SSA
        self.visit_ast();
        self.program
    }
    
    fn visit_function(&mut self, func: &ast::Function) {
        let func_id = self.program.add_function(&func.name);
        self.current_func = Some(func_id);
        
        // 创建入口块
        let entry_block = self.program[func_id].add_block();
        self.current_block = Some(entry_block);
        
        // 处理参数
        for param in &func.params {
            let param_val = Value::Param(entry_block, param.id);
            self.variables.insert(param.name.clone(), param_val);
        }
        
        // 处理函数体
        self.visit_block(&func.body);
        
        // 插入Phi节点
        self.insert_phi_nodes();
    }
    
    fn visit_if(&mut self, cond: &ast::Expr, then_block: &ast::Block, else_block: Option<&ast::Block>) {
        let cond_val = self.visit_expr(cond);
        
        // 创建then/else块
        let then_id = self.new_block();
        let else_id = self.new_block();
        let merge_id = self.new_block();
        
        // 条件跳转
        self.terminate(Terminator::Branch(cond_val, then_id, else_id));
        
        // 处理then分支
        self.current_block = Some(then_id);
        self.visit_block(then_block);
        self.terminate(Terminator::Jump(merge_id));
        
        // 处理else分支
        self.current_block = Some(else_id);
        if let Some(else_blk) = else_block {
            self.visit_block(else_blk);
        }
        self.terminate(Terminator::Jump(merge_id));
        
        // 合并块
        self.current_block = Some(merge_id);
    }
    
    fn insert_phi_nodes(&mut self) {
        // 使用经典算法插入Phi节点
        // 1. 计算支配边界
        // 2. 在支配边界插入Phi节点
        // 3. 重命名变量
        
        let dominance = self.compute_dominance();
        let dominance_frontier = self.compute_dominance_frontier(&dominance);
        
        // 为每个变量插入Phi节点
        for (var_name, def_blocks) in self.variable_definitions() {
            for block_id in &def_blocks {
                for df_block in &dominance_frontier[block_id] {
                    self.insert_phi(var_name, *df_block);
                }
            }
        }
    }
}
```

## 分析缓存

```rust
// src/ssa/analysis.rs

/// 分析结果缓存
pub struct AnalysisResults {
    // 类型推导
    pub type_map: HashMap<Value, Type>,
    
    // 常量传播
    pub constants: HashMap<Value, Constant>,
    
    // 到达定义
    pub reaching_defs: HashMap<BlockId, HashSet<DefId>>,
    
    // 活跃变量
    pub live_vars: HashMap<BlockId, HashSet<String>>,
    
    // 逃逸分析
    pub escape_info: HashMap<Value, EscapeKind>,
}

impl AnalysisResults {
    /// 执行所有分析（一次构建）
    pub fn analyze(ssa: &SsaProgram) -> Self {
        let mut results = Self::default();
        
        // 类型推导
        results.type_inference(ssa);
        
        // 常量传播
        results.constant_propagation(ssa);
        
        // 到达定义
        results.reaching_definitions(ssa);
        
        // 活跃变量
        results.liveness_analysis(ssa);
        
        // 逃逸分析
        results.escape_analysis(ssa);
        
        results
    }
    
    /// 类型推导
    fn type_inference(&mut self, ssa: &SsaProgram) {
        // 基于Go类型系统推导每个值的类型
        for func in &ssa.functions {
            for block in &func.blocks {
                for inst in &block.instructions {
                    let ty = self.infer_inst_type(inst);
                    self.type_map.insert(inst.result(), ty);
                }
            }
        }
    }
    
    /// 常量传播
    fn constant_propagation(&mut self, ssa: &SsaProgram) {
        // 稀疏条件常量传播 (SCCP)
        let mut worklist = VecDeque::new();
        let mut executable = HashSet::new();
        
        // 初始状态
        executable.insert((FuncId(0), BlockId(0)));
        
        while let Some((func_id, block_id)) = worklist.pop_front() {
            if !executable.insert((func_id, block_id)) {
                continue;
            }
            
            let func = &ssa.functions[func_id.0];
            let block = &func.blocks[block_id.0];
            
            for inst in &block.instructions {
                if let Some(const_val) = self.eval_constant(inst) {
                    self.constants.insert(inst.result(), const_val);
                }
            }
            
            // 处理终止符
            match &block.terminator {
                Terminator::Branch(cond, then_blk, else_blk) => {
                    if let Some(const_cond) = self.constants.get(cond) {
                        // 常量条件，只走一个分支
                        let target = if const_cond.is_true() { then_blk } else { else_blk };
                        worklist.push_back((func_id, *target));
                    } else {
                        // 非常量，两个分支都可能
                        worklist.push_back((func_id, *then_blk));
                        worklist.push_back((func_id, *else_blk));
                    }
                }
                Terminator::Jump(target) => {
                    worklist.push_back((func_id, *target));
                }
                _ => {}
            }
        }
    }
}
```

## 基于SSA的规则检查

```rust
// src/rules/ssa_based.rs

/// 基于SSA的未使用变量检测
pub struct SsaUnusedVariable;

impl SsaRule for SsaUnusedVariable {
    fn code(&self) -> &'static str { "E002" }
    
    fn check(&self, ssa: &SsaProgram, analysis: &AnalysisResults) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        
        for func in &ssa.functions {
            // 使用def-use链检查
            for (value, defs) in &analysis.reaching_defs {
                if defs.len() > 1 {
                    continue; // 有多个定义，可能是活跃的
                }
                
                let def_id = defs.iter().next().unwrap();
                let uses = self.find_uses(ssa, *def_id);
                
                if uses.is_empty() {
                    // 无使用，报告未使用变量
                    diags.push(Diagnostic {
                        code: "E002".to_string(),
                        message: format!("Variable is unused"),
                        // ...
                    });
                }
            }
        }
        
        diags
    }
}

/// 基于SSA的nil指针检测
pub struct SsaNilPointer;

impl SsaRule for SsaNilPointer {
    fn code(&self) -> &'static str { "E006" }
    
    fn check(&self, ssa: &SsaProgram, analysis: &AnalysisResults) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        
        for func in &ssa.functions {
            for block in &func.blocks {
                for inst in &block.instructions {
                    if let Instruction::Load { ptr, .. } = inst {
                        // 检查ptr是否可能为nil
                        let nil_status = self.analyze_nil_status(ssa, analysis, ptr);
                        
                        if nil_status == NilStatus::AlwaysNil {
                            diags.push(Diagnostic {
                                code: "E006".to_string(),
                                message: "Nil pointer dereference".to_string(),
                                // ...
                            });
                        } else if nil_status == NilStatus::MaybeNil {
                            // 警告级别
                            diags.push(Diagnostic {
                                code: "E006".to_string(),
                                message: "Possible nil pointer dereference".to_string(),
                                severity: Severity::Warning,
                                // ...
                            });
                        }
                    }
                }
            }
        }
        
        diags
    }
}

/// 基于SSA的逃逸分析
pub struct SsaEscapeAnalysis;

impl SsaRule for SsaEscapeAnalysis {
    fn code(&self) -> &'static str { "P003" }
    
    fn check(&self, ssa: &SsaProgram, analysis: &AnalysisResults) -> Vec<Diagnostic> {
        let mut diags = Vec::new();
        
        for func in &ssa.functions {
            for block in &func.blocks {
                for inst in &block.instructions {
                    if let Instruction::Alloc { ty } = inst {
                        let escape = &analysis.escape_info[&inst.result()];
                        
                        if *escape == EscapeKind::Heap {
                            // 检查是否可以栈分配
                            if self.can_stack_allocate(ssa, analysis, inst) {
                                diags.push(Diagnostic {
                                    code: "P003".to_string(),
                                    message: format!(
                                        "Heap allocation can be moved to stack"
                                    ),
                                    severity: Severity::Info,
                                    // ...
                                });
                            }
                        }
                    }
                }
            }
        }
        
        diags
    }
}
```

## 性能对比预测

### 当前（无SSA/IR缓存）

| 规则类型 | 当前性能 | 瓶颈 |
|----------|----------|------|
| 简单AST规则 | 2ms | AST遍历 |
| 未使用变量 | 5ms | 重复符号查找 |
| nil检查 | 8ms | 无类型信息 |
| 逃逸分析 | 20ms | 无数据流分析 |
| 复杂规则 | 50ms+ | 多次遍历 |

### 有SSA/IR缓存后

| 规则类型 | 预期性能 | 加速比 | 原因 |
|----------|----------|--------|------|
| 简单AST规则 | 2ms | 1x | 仍使用AST |
| 未使用变量 | 1ms | 5x | def-use链直接可用 |
| nil检查 | 2ms | 4x | 类型信息缓存 |
| 逃逸分析 | 3ms | 6.5x | 逃逸分析结果缓存 |
| 复杂规则 | 5ms | 10x | SSA形式统一处理 |

### 热运行时间对比

```
场景: 检查1000行Go代码

无SSA缓存:
├─> AST构建: 5ms (缓存命中)
├─> 规则1 (unused): 8ms (遍历+符号表)
├─> 规则2 (nil): 12ms (类型推导)
├─> 规则3 (escape): 25ms (数据流分析)
└─> 总计: ~50ms

有SSA缓存:
├─> AST构建: 5ms (缓存命中)
├─> SSA构建: 10ms (缓存命中) ⭐
├─> 分析结果: 0ms (缓存命中) ⭐⭐
├─> 规则1 (unused): 1ms (直接查def-use)
├─> 规则2 (nil): 2ms (类型已推导)
├─> 规则3 (escape): 3ms (逃逸分析已做)
└─> 总计: ~11ms

加速比: 4.5x
```

## 缓存策略

### 缓存键设计

```rust
fn compute_cache_key(ast: &Ast, config: &Config) -> u64 {
    let mut hasher = DefaultHasher::new();
    
    // AST结构hash
    ast.hash(&mut hasher);
    
    // 配置hash（影响分析结果的部分）
    config.target_go_version.hash(&mut hasher);
    config.build_tags.hash(&mut hasher);
    
    hasher.finish()
}
```

### 缓存失效策略

```rust
enum CacheInvalidation {
    // 源码变更 - 必须重建
    SourceChanged,
    
    // 配置变更 - 部分重建
    ConfigChanged {
        affected_analysis: Vec<AnalysisKind>,
    },
    
    // Go版本变更 - 全部重建
    GoVersionChanged,
    
    // 依赖变更 - 可能重建
    DependencyChanged,
}
```

### 内存管理

```rust
const MAX_SSA_CACHE_SIZE: usize = 100 * 1024 * 1024; // 100MB
const MAX_ANALYSIS_CACHE_SIZE: usize = 50 * 1024 * 1024; // 50MB

pub struct CacheManager {
    ssa_cache: LruCache<String, Arc<SsaProgram>>, // 最近使用
    analysis_cache: LruCache<String, Arc<AnalysisResults>>,
    
    // 磁盘缓存（大项目）
    disk_cache: Option<PathBuf>,
}
```

## 实施路线图

### Phase 1: SSA基础 (2周)
- [ ] SSA IR定义
- [ ] 基本SSA构建器
- [ ] 简单函数处理

### Phase 2: 控制流 (2周)
- [ ] CFG构建
- [ ] Phi节点插入
- [ ] 支配树计算

### Phase 3: 分析框架 (3周)
- [ ] 类型推导
- [ ] 常量传播
- [ ] def-use链

### Phase 4: 规则迁移 (2周)
- [ ] 未使用变量检测
- [ ] nil检查
- [ ] 逃逸分析

### Phase 5: 缓存优化 (1周)
- [ ] SSA缓存
- [ ] 分析结果缓存
- [ ] 磁盘持久化

**总计: 10周**

## 预期收益

### 性能收益

| 指标 | 当前 | SSA缓存后 | 提升 |
|------|------|-----------|------|
| 热运行(单文件) | 2ms | 0.5ms | 4x |
| 复杂规则 | 50ms | 5ms | 10x |
| 大项目(1000文件) | 1200ms | 300ms | 4x |

### 功能收益

- ✅ 更精确的nil检查
- ✅ 跨过程分析
- ✅ 常量传播优化建议
- ✅ 逃逸分析优化建议
- ✅ 死代码检测

### 资源开销

| 资源 | 开销 | 备注 |
|------|------|------|
| 内存 | +50MB | SSA缓存 |
| 构建时间 | +15ms | 首次构建SSA |
| 磁盘 | +100MB | 持久化缓存 |

## 风险与挑战

### 技术挑战

1. **Go类型系统复杂**
   - 接口、chan、map等需要特殊处理
   - 反射代码难以静态分析

2. **构建时间增加**
   - SSA构建需要时间
   - 需要平衡构建vs查询性能

3. **内存占用**
   - SSA表示比AST大
   - 需要有效的缓存淘汰

### 缓解措施

1. **渐进式迁移**
   - 先对简单函数构建SSA
   - 复杂函数回退到AST

2. **并行构建**
   - 函数级并行构建SSA
   - 后台线程预构建

3. **压缩缓存**
   - 序列化压缩存储
   - 内存紧张时换出

## 结论

SSA/IR缓存可以将woof的热运行性能提升 **4-10倍**，同时支持更复杂的静态分析。

**推荐实施优先级**: P2 (重要但不紧急)
- 当前性能已足够好
- SSA是长期技术债
- 建议6个月后开始实施
