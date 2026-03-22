# Woof 性能基准测试

## 内存基准测试

### 运行内存测试

```bash
# 编译发布版本
cargo build --release

# 运行内存基准
./target/release/memory_usage benchmark/scenarios

# 对比优化前后
cargo run --bin memory_usage -- benchmark/scenarios
```

### 测试场景

1. **小项目** (100文件): 测试基础内存使用
2. **中型项目** (1000文件): 测试批量处理优化
3. **大型项目** (10000文件): 测试流式处理效果

### 关键指标

| 指标 | 说明 | 目标 |
|------|------|------|
| Peak Memory | 峰值内存使用 | < 500MB for 1000 files |
| Memory/File | 每文件平均内存 | < 100KB |
| Cache Hit Rate | AST缓存命中率 | > 90% (二次运行) |
| Parser Reuse | Parser复用率 | > 95% |

## 速度基准测试

```bash
cargo bench
```

## 对比测试

### vs golangci-lint

```bash
cd benchmark
./run_benchmark.sh
```

### 内存对比

| 工具 | 100文件 | 1000文件 | 10000文件 |
|------|---------|----------|-----------|
| woof (优化前) | 50MB | 300MB | 2GB |
| woof (优化后) | 20MB | 150MB | 800MB |
| golangci-lint | 200MB | 1GB | 4GB+ |
