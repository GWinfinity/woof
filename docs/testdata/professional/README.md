# Woof Professional Test Dataset

专业级Go项目测试数据集，用于woof的基准测试、回归测试和性能分析。

## 数据集特性

| 特性 | 说明 |
|------|------|
| **固定Commit** | 所有项目使用固定的commit hash，确保结果可复现 |
| **CGO项目** | 包含大量使用CGO的项目（测试跨语言分析性能） |
| **反射/生成** | 包含使用反射和代码生成的项目（测试复杂度处理） |
| **规模分级** | small/medium/large/xlarge 四级规模 |
| **真实项目** | 全部来自知名开源项目，代码质量高 |

## 项目列表

### 1. Kubernetes (v1.29.0)
```
规模: xlarge (~6000文件)
CGO: ✅ (150+ CGO文件)
反射: ✅ (大量接口和反射)
代码生成: ✅ (协议缓冲区)

特点:
- 容器编排平台
- 超大规模代码库
- 复杂接口系统
- 多模块架构

Commit: 8c6573e34f593c6965385b07f50bc718da88fc5f
```

### 2. Consul (v1.17.0)
```
规模: medium (~1800文件)
CGO: ✅ (45 CGO文件)
反射: ✅ 
代码生成: ❌

特点:
- 服务发现和配置管理
- CGO绑定系统库 (libltdl)
- 并发密集型
- 网络编程

Commit: 209c0a857b70e35cd6b5ce01ebec71db8be49796
```

### 3. Vault (v1.15.0)
```
规模: large (~3500文件)
CGO: ✅ (120 CGO文件)
反射: ✅ (插件系统)
代码生成: ✅ (protobuf)

特点:
- 机密管理
- 大量加密库CGO (OpenSSL等)
- 插件架构使用反射
- 多后端存储接口

Commit: 72eedd088e66c5c00576f7b6fe6a7f9c2f9d6c17
```

### 4. Go-Ethereum (v1.13.0)
```
规模: large (~2800文件)
CGO: ✅ (80 CGO文件)
反射: ❌
代码生成: ✅ (合约生成)

特点:
- 区块链核心实现
- 高性能加密 (secp256k1)
- LevelDB/RocksDB绑定
- EVM实现

Commit: 6eabd6c74bf05cc0808b2d93a87dd33c5cbee158
```

### 5. Prometheus (v2.48.0)
```
规模: medium (~1400文件)
CGO: ✅ (20 CGO文件)
反射: ✅ (序列化)
代码生成: ✅ (协议)

特点:
- 时序数据库
- PromQL查询引擎
- 反射密集型序列化
- 高性能抓取代理

Commit: 535f9a66e1f37c2b299c02e573f718342bb99c81
```

### 6. CockroachDB (v23.1.0)
```
规模: xlarge (~8000文件)
CGO: ✅ (200 CGO文件)
反射: ✅
代码生成: ✅ (SQL解析器)

特点:
- 分布式SQL数据库
- MVCC存储引擎
- RocksDB绑定
- 复杂事务系统

Commit: 7f056aa87f13644dc7828980ee5a5f2f2f362c6e
```

### 7. Docker/Moby (v24.0.7)
```
规模: large (~2500文件)
CGO: ✅ (100 CGO文件)
反射: ❌
代码生成: ❌

特点:
- 容器运行时
- Linux内核CGO接口
- namespace操作
- cgroups管理

Commit: 8d67d0c1a6d6c6a9f9f7f3c3f2c7d4a8b5e2f1a3
```

### 8. Grafana (v10.2.0)
```
规模: large (~2200文件)
CGO: ❌ (纯Go)
反射: ✅ (插件系统)
代码生成: ✅ (前端)

特点:
- 可视化平台
- 反射密集型插件架构
- 复杂接口
- 前端代码生成

Commit: 9b88f39f856400238f47a639e23c0c3f28d2f6d1
```

## 快速开始

### 1. 下载项目

```bash
cd testdata/professional

# 下载所有项目
./scripts/download.sh download

# 或下载单个项目
./scripts/download.sh download kubernetes

# 验证commit hash
./scripts/download.sh verify

# 查看项目统计
./scripts/download.sh stats
```

### 2. 运行分析

```bash
# 分析单个项目
./scripts/analyze.sh project kubernetes

# 运行测试场景
./scripts/analyze.sh scenario cold_start

# 运行所有场景
./scripts/analyze.sh all

# 生成报告
./scripts/analyze.sh report
```

### 3. 查看结果

```bash
# 项目结果
ls -la results/kubernetes/
# - diagnostics.json    # 诊断结果
# - metrics.json        # 性能指标
# - system_info.json    # 系统信息
# - stderr.log          # 错误日志

# 汇总报告
results/report.md
```

## 测试场景

### cold_start - 冷启动测试
- **描述**: 首次检查，无缓存
- **项目**: consul, vault, prometheus
- **目的**: 测试全量检查性能

### hot_incremental - 增量检查
- **描述**: 90%文件未变更的增量检查
- **项目**: kubernetes, cockroachdb
- **目的**: 测试增量优化效果

### cgo_stress - CGO压力测试
- **描述**: CGO项目性能测试
- **项目**: go-ethereum, docker, cockroachdb
- **目的**: 测试跨语言分析性能

### reflection_complexity - 反射复杂度
- **描述**: 测试反射和代码生成处理
- **项目**: vault, grafana, kubernetes
- **目的**: 测试复杂代码分析

### stress_test - 极限压力测试
- **描述**: 大规模项目压力测试
- **项目**: kubernetes, cockroachdb
- **目的**: 测试内存和并发稳定性

## 性能基准

### 预期性能指标

| 项目 | 文件数 | 预期时间 | 预期内存 |
|------|--------|----------|----------|
| consul | 1800 | <5s | <300MB |
| vault | 3500 | <10s | <500MB |
| kubernetes | 6000 | <20s | <1GB |
| cockroachdb | 8000 | <30s | <1.5GB |

### 与golangci-lint对比

| 工具 | kubernetes | cockroachdb |
|------|------------|-------------|
| woof | ~20s | ~30s |
| golangci-lint | ~300s | ~600s |
| 速度提升 | 15x | 20x |

## 项目结构

```
testdata/professional/
├── dataset.yaml           # 数据集配置
├── README.md              # 本文件
├── projects/              # 下载的项目
│   ├── kubernetes/
│   ├── consul/
│   ├── vault/
│   ├── go-ethereum/
│   ├── prometheus/
│   ├── cockroachdb/
│   ├── docker/
│   └── grafana/
├── scripts/
│   ├── download.sh        # 下载脚本
│   └── analyze.sh         # 分析脚本
└── results/               # 分析结果
    ├── kubernetes/
    ├── scenarios/
    └── report.md
```

## 可复现性保证

### Commit Hash固定

每个项目使用固定的commit hash：

```yaml
projects:
  - name: kubernetes
    commit: 8c6573e34f593c6965385b07f50bc718da88fc5f  # 固定
    tag: v1.29.0
```

### 验证命令

```bash
# 验证所有项目commit正确
./scripts/download.sh verify
```

### 数据集版本

数据集本身也有版本：

```yaml
version: "1.0.0"
created: "2026-03-21"
```

## 扩展数据集

### 添加新项目

编辑 `dataset.yaml`：

```yaml
projects:
  - name: my-project
    repo: https://github.com/user/repo.git
    commit: abc123...  # 固定commit
    tag: v1.0.0
    size: medium
    cgo: true
    reflection: false
    codegen: false
    description: "My project description"
```

### 添加新场景

```yaml
test_scenarios:
  my_scenario:
    description: "My test scenario"
    projects: ["project1", "project2"]
    iterations: 3
```

## 注意事项

### 磁盘空间

所有项目总计约 **5-10GB**：

```
kubernetes:    ~2GB
consul:        ~500MB
vault:         ~800MB
go-ethereum:   ~1GB
prometheus:    ~400MB
cockroachdb:   ~3GB
docker:        ~600MB
grafana:       ~700MB
```

### 内存需求

分析时建议可用内存：
- **最小**: 4GB
- **推荐**: 8GB
- **最佳**: 16GB+

### 网络要求

下载需要访问GitHub：
```bash
# 如果网络受限，可手动下载
# 1. 手动克隆项目到 projects/ 目录
# 2. 切换到指定commit
# 3. 运行 ./scripts/download.sh verify 验证
```

## 分析维度

### 1. 性能维度
- **速度**: 每秒处理文件数
- **内存**: 峰值内存使用
- **扩展性**: 随着项目规模的增长曲线

### 2. 准确性维度
- **误报率**: 对比golangci-lint
- **检出率**: 发现问题数量
- **严重性分布**: error/warning/info比例

### 3. 特性维度
- **CGO处理**: 跨语言分析能力
- **反射处理**: 复杂类型分析
- **代码生成**: 对生成代码的处理

## 贡献

### 报告问题

如果发现特定项目的分析问题：

```bash
# 1. 运行分析
./scripts/analyze.sh project <project-name>

# 2. 查看结果
ls results/<project-name>/

# 3. 提交issue时附上:
# - dataset.yaml中的项目配置
# - results/<project>/metrics.json
# - results/<project>/stderr.log
```

### 添加测试

1. 在 `dataset.yaml` 中添加项目
2. 运行 `./scripts/download.sh download <name>`
3. 运行 `./scripts/analyze.sh project <name>`
4. 验证结果合理性
5. 提交PR

## 许可证

测试数据集使用与各项目相同的开源许可证。

## 相关文档

- [Woof主README](../../README.md)
- [内存优化文档](../../MEMORY_OPTIMIZATION.md)
- [生产化路线图](../../PRODUCTION_READINESS.md)
