# Woof 新特性规则集

> 自动生成于 2026-03-22T10:07:16.248203

本规则集专门针对 Go 语言的新特性，包括泛型、Fuzzing、Workspace 等。

## 规则概览

总计 **71** 条规则

## Fuzzing 规则

针对 Go 1.18+ Fuzzing 测试的检查规则

共 16 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `FUZZ001` | fuzz-test-signature | Fuzzing 测试函数签名错误 | 🔴 error |
| `FUZZ002` | fuzz-target-name | Fuzzing 目标命名不规范 | 🟡 warning |
| `FUZZ003` | fuzz-missing-corpus | 缺少 Fuzzing 语料库目录 | 🔵 info |
| `FUZZ004` | fuzz-seed-corpus | 建议使用种子语料库 | 🔵 info |
| `FUZZ005` | fuzz-parallel | Fuzzing 测试应考虑并行执行 | 🔵 info |
| `FUZZ006` | fuzz-cleanup | Fuzzing 测试后缺少清理 | 🟡 warning |
| `FUZZ007` | fuzz-add-duplicate | Fuzzing 中添加重复语料 | 🟡 warning |
| `FUZZ008` | fuzz-skipped | Fuzzing 测试被跳过 | 🔵 info |
| `FUZZ009` | fuzz-timeout | Fuzzing 测试应设置超时 | 🟡 warning |
| `FUZZ010` | fuzz-panic-recover | Fuzzing 中不建议使用 panic/recover | 🟡 warning |
| `FUZZ011` | fuzz-global-state | Fuzzing 测试使用了全局状态 | 🔴 error |
| `FUZZ012` | fuzz-non-deterministic | Fuzzing 测试非确定性行为 | 🔴 error |
| `FUZZ101` | fuzz-input-validation | Fuzzing 输入应充分验证 | 🟡 warning |
| `FUZZ102` | fuzz-expensive-op | Fuzzing 循环中有昂贵操作 | 🟡 warning |
| `FUZZ103` | fuzz-file-operation | Fuzzing 测试中避免文件操作 | 🔵 info |
| `FUZZ104` | fuzz-network-access | Fuzzing 测试中避免网络访问 | 🟡 warning |

## 泛型规则 (Generics)

针对 Go 1.18+ 泛型特性的检查规则

共 17 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `GEN001` | unused-type-parameter | 未使用的类型参数 | 🟡 warning |
| `GEN002` | type-parameter-shadow | 类型参数遮蔽外部类型 | 🔴 error |
| `GEN003` | any-constraint | 使用 any 约束而非 interface{} | 🔵 info |
| `GEN004` | comparable-misuse | comparable 约束的误用 | 🔴 error |
| `GEN005` | type-set-redundant | 冗余的类型集合 | 🟡 warning |
| `GEN006` | generic-interface-method | 泛型接口中的方法限制 | 🟡 warning |
| `GEN007` | type-inference-fail | 无法推断类型参数 | 🔴 error |
| `GEN008` | type-param-too-many | 过多的类型参数 | 🟡 warning |
| `GEN009` | constraint-unnecessary | 不必要的类型约束 | 🔵 info |
| `GEN010` | generic-naming | 类型参数命名不规范 (应为大写字母) | 🟡 warning |
| `GEN011` | type-assertion-in-generic | 泛型函数中不必要的类型断言 | 🟡 warning |
| `GEN012` | reflect-in-generic | 泛型函数中使用 reflect 而非类型参数 | 🔵 info |
| `GEN013` | pointer-receiver-generic | 泛型类型的指针接收器检查 | 🟡 warning |
| `GEN014` | type-param-bounds | 类型参数边界检查 | 🔴 error |
| `GEN015` | instantiation-error | 泛型实例化错误 | 🔴 error |
| `GEN101` | generic-boxing | 泛型装箱操作可能影响性能 | 🔵 info |
| `GEN102` | generic-monomorphization | 建议泛型单态化优化 | 🔵 info |

## Go 1.20+ 规则

Go 1.20 新特性相关规则

共 6 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `UP1201` | use-context-with-cancel-cause | 使用 context.WithCancelCause 替代 WithCancel | 🔵 info |
| `UP1202` | use-errors-join | 使用 errors.Join 替代自定义错误合并 | 🔵 info |
| `UP1203` | use-slices-binary-search | 使用 slices.BinarySearch 替代手动实现 | 🔵 info |
| `UP1204` | use-maps-clone | 使用 maps.Clone 替代手动拷贝 | 🔵 info |
| `UP1205` | use-maps-delete-func | 使用 maps.DeleteFunc 替代循环删除 | 🔵 info |
| `UP1206` | use-atomic-types | 使用 atomic.Int64 等类型替代 atomic.AddInt64 | 🔵 info |

## Go 1.21+ 规则

Go 1.21 新特性相关规则

共 8 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `UP1211` | use-slog-context | 使用 slog 的 Context 方法 | 🔵 info |
| `UP1212` | slog-attr-optimization | slog 属性优化建议 | 🔵 info |
| `UP1213` | slog-group-nesting | slog group 嵌套过深 | 🟡 warning |
| `UP1214` | slog-duplicate-attrs | slog 重复属性 | 🟡 warning |
| `UP1215` | use-slices-sort-func | 使用 slices.SortFunc 替代 sort.Slice | 🔵 info |
| `UP1216` | use-slices-reverse | 使用 slices.Reverse | 🔵 info |
| `UP1217` | use-testing-option | 使用 testing 包的新 Option 类型 | 🔵 info |
| `UP1218` | test-skip-f | 使用 t.Skipf 而非 fmt.Sprintf + t.Skip | 🔵 info |

## Go 1.22+ 规则

Go 1.22 新特性相关规则

共 5 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `UP1221` | loopvar-capture-fixed | Go 1.22+ 循环变量捕获已修复，无需 workarounds | 🔵 info |
| `UP1222` | range-over-int | 使用整数 range (Go 1.22+) | 🔵 info |
| `UP1223` | range-over-func | 使用函数 range (Go 1.23+) | 🔵 info |
| `UP1224` | use-http-servemux-patterns | 使用 http.ServeMux 的新模式语法 | 🔵 info |
| `UP1225` | http-route-conflict | HTTP 路由模式冲突 | 🔴 error |

## Workspace 规则

针对 Go 1.18+ Workspace 多模块开发的检查规则

共 19 条规则

| 代码 | 名称 | 描述 | 级别 |
|------|------|------|------|
| `WS001` | workspace-go-version | Workspace 中 Go 版本不一致 | 🔴 error |
| `WS002` | workspace-module-path | Workspace 模块路径冲突 | 🔴 error |
| `WS003` | workspace-missing-module | Workspace 缺少必要模块 | 🟡 warning |
| `WS004` | workspace-orphan-module | Workspace 中孤立的模块 | 🔵 info |
| `WS005` | workspace-replace-directive | Workspace 中不当的 replace 指令 | 🟡 warning |
| `WS006` | workspace-exclude-directive | Workspace 中不当的 exclude 指令 | 🟡 warning |
| `WS007` | workspace-use-directive | Workspace 中 use 指令问题 | 🔴 error |
| `WS008` | workspace-nested | 嵌套的 Workspace 配置 | 🔴 error |
| `WS009` | workspace-vendor-conflict | Workspace 与 vendor 冲突 | 🟡 warning |
| `WS010` | workspace-local-path | Workspace 使用相对路径可能的问题 | 🔵 info |
| `WS101` | workspace-dep-version-mismatch | Workspace 依赖版本不一致 | 🟡 warning |
| `WS102` | workspace-dep-override | Workspace 依赖覆盖 | 🔵 info |
| `WS103` | workspace-dep-cycle | Workspace 模块间循环依赖 | 🔴 error |
| `WS104` | workspace-dep-deprecated | Workspace 使用已弃用依赖 | 🟡 warning |
| `WS105` | workspace-dep-security | Workspace 依赖存在安全漏洞 | 🔴 error |
| `WS201` | gowork-format | go.work 文件格式错误 | 🔴 error |
| `WS202` | gowork-syntax | go.work 文件语法错误 | 🔴 error |
| `WS203` | gowork-duplicate-use | go.work 中重复的 use 指令 | 🟡 warning |
| `WS204` | gowork-missing-directive | go.work 缺少必要指令 | 🔴 error |

## 实现建议

### 优先级

1. **P0 (必须)** 🔴: 错误级别规则，可能导致运行时问题
2. **P1 (推荐)** 🟡: 警告级别规则，建议遵循的最佳实践
3. **P2 (可选)** 🔵: 信息级别规则，代码改进建议

### 实现顺序

建议按以下顺序实现：

1. **泛型规则 (GENxxx)**
   - 优先实现 GEN001-GEN010 (核心泛型检查)
   - 然后实现 GEN101-GEN102 (性能相关)

2. **Fuzzing 规则 (FUZZxxx)**
   - 优先实现 FUZZ001, FUZZ011 (基础规范)
   - 然后实现其他规则

3. **Workspace 规则 (WSxxx)**
   - 优先实现 WS001-WS010 (Workspace 结构)
   - 然后实现依赖相关规则

4. **Go 版本升级规则 (UP12xx)**
   - 按 Go 版本顺序实现
   - 优先实现高影响规则

## 参考

- [Go 1.18 Release Notes - Generics](https://go.dev/doc/go1.18)
- [Go 1.18 Release Notes - Fuzzing](https://go.dev/doc/go1.18)
- [Go 1.18 Release Notes - Workspace](https://go.dev/doc/go1.18)
- [Go 1.20 Release Notes](https://go.dev/doc/go1.20)
- [Go 1.21 Release Notes](https://go.dev/doc/go1.21)
- [Go 1.22 Release Notes](https://go.dev/doc/go1.22)
