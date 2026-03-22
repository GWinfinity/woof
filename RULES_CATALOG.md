# Woof Rules Catalog - Go 语言规则体系

参照 Python flake8 生态，为 Go 语言设计的完整规则分类体系。

---

## 规则前缀体系

| 前缀 | 来源 | 含义 | 规则数量 |
|------|------|------|----------|
| **E** | gocodestyle | 代码风格与语法错误 | 20 |
| **F** | goflakes | 逻辑错误与潜在运行时问题 | 18 |
| **B** | go-bugbear | 代码质量提升与反模式检测 | 15 |
| **I** | goimports | 导入排序与分组检查 | 8 |
| **UP** | goupgrade | 升级到现代 Go 语法 | 10 |
| **SIM** | go-simplify | 代码简化建议 | 12 |
| **S** | style | 风格与命名规范 | 10 |
| **D** | docs | 文档与注释规范 | 7 |
| **P** | performance | 性能优化建议 | 6 |
| **C** | concurrency | 并发问题检测 | 5 |
| **SEC** | security | 安全问题检测 | 5 |

**总计: 116 个规则**

---

## E 系列 - gocodestyle（代码风格与语法错误）

### 布局与格式 (E1xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| E101 | mixed-indentation | 混合使用空格和 Tab | `\t    var x` |
| E111 | over-indented | 过度缩进 | 缩进超过必要层级 |
| E112 | under-indented | 缩进不足 | 代码块缩进不够 |
| E113 | blank-line-after-docstring | 文档字符串后缺少空行 | `/** doc */\nfunc` |
| E114 | blank-line-before-comment | 注释前缺少空行 | `}\n// comment` |
| E115 | trailing-whitespace | 行尾空白字符 | `var x = 1    ` |
| E116 | multiple-trailing-newlines | 多个 trailing 换行 | 文件末尾多行空行 |
| E117 | no-newline-at-end | 文件末尾缺少换行 | EOF 非换行符 |
| E118 | line-too-long | 行过长 | 超过 120 字符 |

### 导入 (E2xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| E201 | blank-line-after-import | 导入后缺少空行 | `import "fmt"\nfunc` |
| E202 | import-not-at-top | 导入不在文件顶部 | 函数内有 import |
| E203 | duplicate-import | 重复导入 | 同一包导入两次 |
| E204 | unused-import | 未使用的导入 | import "os" 但未使用 |
| E205 | import-alias-unused | 别名未使用 | `import f "fmt"` 用 fmt.Println |
| E206 | import-shadow | 导入别名遮蔽内置 | `import str "strings"` |

### 空白与格式 (E3xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| E301 | multiple-imports | 多个导入在同一行 | `import ("a"; "b")` |
| E302 | missing-whitespace | 运算符两侧缺少空格 | `a+b` |
| E303 | unexpected-whitespace | 意外空格 | `f (x)` 调用时空格 |
| E304 | whitespace-around-colon | 冒号周围空格错误 | `map [ string ] int` |

### 代码结构 (E4xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| E401 | multiple-statements | 多语句在一行 | `a := 1; b := 2` |
| E402 | import-block-not-first | import 块不在文件首 | 被代码前置 |
| E403 | block-comment-not-aligned | 块注释未对齐 | `/* misaligned */` |
| E404 | docstring-quotes | 文档字符串引号不一致 | 混用 `"""` 和 `'''` |

### 比较与判断 (E7xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| E711 | comparison-to-nil | 与 nil 比较方式错误 | `err == nil` 应为 `err != nil` |
| E712 | comparison-to-true | 与 true 显式比较 | `x == true` 应为 `x` |
| E713 | comparison-to-false | 与 false 显式比较 | `x == false` 应为 `!x` |
| E714 | suspicious-comparison | 可疑比较 | 比较函数而非调用 |
| E721 | type-comparison | 类型比较方式 | 使用 `reflect.TypeOf` 而非 `==` |

---

## F 系列 - goflakes（逻辑错误与潜在运行时问题）

### 未使用 (F4xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| F401 | unused-import | 未使用的导入 | import "os" 未引用 |
| F402 | unused-variable | 未使用的变量 | `x := 1` 后未用 |
| F403 | unused-parameter | 未使用的参数 | 函数参数未引用 |
| F404 | unused-return-value | 未使用的返回值 | 忽略重要返回 |
| F405 | unused-label | 未使用的标签 | `Label:` 后无跳转 |
| F406 | unused-const | 未使用的常量 | const 定义未用 |
| F407 | unused-type | 未使用的类型 | type 定义未用 |

### 变量问题 (F8xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| F811 | redefined-variable | 变量重定义 | `x := 1; x := 2` |
| F812 | undefined-variable | 未定义的变量 | 使用未声明变量 |
| F821 | variable-shadows-import | 变量遮蔽导入 | `os := 1` 遮蔽 `import "os"` |
| F822 | variable-shadows-builtin | 变量遮蔽内置 | `len := 1` 遮蔽 len() |
| F823 | variable-shadows-parameter | 变量遮蔽参数 | `func f(x int) { x := 1 }` |
| F831 | loop-variable-capture | 循环变量捕获 | goroutine 中误用循环变量 |
| F841 | unused-assignment | 未使用的赋值 | `x = 1` 但 x 未再用 |

### 控制流 (F9xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| F901 | unreachable-code | 不可达代码 | return 后的代码 |
| F902 | duplicate-case | 重复的 case | switch 中重复 case |
| F903 | missing-return | 缺少 return | 非 void 函数无 return |
| F904 | return-in-finally | defer 中 return | defer func 中 return |

---

## B 系列 - go-bugbear（代码质量与反模式，含 Uber Go Style）

### Uber Go Style - 并发规范 (B0xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| B001 | mutex-zero-value | Mutex 零值使用 | `var mu sync.Mutex` 无需初始化 |
| B002 | channel-buffer-size | Channel 缓冲限制 | `make(chan T, 10)` 应 ≤1 |
| B003 | goroutine-leak | Goroutine 泄漏 | 禁止 Fire-and-Forget |
| B004 | atomic-operations | 原子操作封装 | 用 go.uber.org/atomic |

### Uber Go Style - 性能优化 (B1xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| B101 | strconv-over-fmt | 字符串转换优化 | `strconv.Itoa` 替代 `fmt.Sprintf("%d", n)` |

### Uber Go Style - 防御性编程 (B2xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| B201 | map-slice-copy | 防御性复制 | API 边界返回内部 map/slice |
| B202 | slice-index-protection | Slice 索引保护 | 访问前检查长度 |

### 资源管理 (B3xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| B301 | missing-defer-close | 资源未关闭 | `os.Open` 无 defer Close |
| B302 | context-cancel | Context 未取消 | `WithCancel` 返回的 cancel 未调用 |

---

## I 系列 - goimports（导入排序与分组）

### 导入排序 (I0xx)

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| I001 | unsorted-imports | 导入未排序 | 未按字母顺序 |
| I002 | import-groups-missing | 缺少导入分组 | 标准库/第三方未分组 |
| I003 | wrong-import-order | 导入顺序错误 | 标准库应在第三方前 |
| I004 | blank-import-unused | 空白导入未使用 | `import _ "fmt"` 无 init 依赖 |
| I005 | blank-import-missing | 应使用空白导入 | 仅需要 init 副作用 |
| I006 | import-alias-necessary | 应使用别名 | `import json "encoding/json"` |
| I007 | import-alias-unnecessary | 别名不必要 | 无冲突却用别名 |
| I008 | dot-import | 点导入 | `import . "fmt"` 不推荐 |

---

## UP 系列 - goupgrade（升级到现代 Go 语法）

### 语法现代化 (UP0xx)

| 代码 | 名称 | 描述 | 升级前 | 升级后 |
|------|------|------|--------|--------|
| UP001 | use-strings-contains | 使用 strings.Contains | `strings.Index(s, sub) >= 0` | `strings.Contains(s, sub)` |
| UP002 | use-strings-prefix | 使用 strings.HasPrefix | `strings.HasPrefix(s, prefix)` | 同上（提醒） |
| UP003 | use-io-read-all | 使用 io.ReadAll | `ioutil.ReadAll(r)` | `io.ReadAll(r)` |
| UP004 | use-os-read-file | 使用 os.ReadFile | `ioutil.ReadFile(path)` | `os.ReadFile(path)` |
| UP005 | use-os-write-file | 使用 os.WriteFile | `ioutil.WriteFile(path, data, 0644)` | `os.WriteFile(path, data, 0644)` |
| UP006 | use-context-with-cancel | 使用 context.WithCancel | `context.WithCancel(ctx)` | 提醒使用 |
| UP007 | use-errors-is | 使用 errors.Is | `err == ErrFoo` | `errors.Is(err, ErrFoo)` |
| UP008 | use-errors-as | 使用 errors.As | 类型断言错误 | `errors.As(err, &target)` |
| UP009 | use-slices-sort | 使用 slices.Sort | `sort.Ints(x)` | `slices.Sort(x)` |
| UP010 | use-maps-keys | 使用 maps.Keys | 循环取 keys | `maps.Keys(m)` |

---

## SIM 系列 - go-simplify（代码简化建议）

### 简化建议 (SIM1xx)

| 代码 | 名称 | 描述 | 简化前 | 简化后 |
|------|------|------|--------|--------|
| SIM101 | use-if-return | 提前返回 | `if x { ... } else { return }` | `if !x { return } ...` |
| SIM102 | use-if-continue | 继续循环 | 嵌套 if | 提前 continue |
| SIM103 | return-condition | 直接返回条件 | `if x { return true } return false` | `return x` |
| SIM104 | use-any | 使用 any | `interface{}` | `any` |
| SIM105 | use-min-max | 使用 min/max | 自定义比较 | `min(a, b)` |
| SIM106 | use-ternary | 简化条件赋值 | `var x; if c { x = a } else { x = b }` | `x := map[bool]int{true: a, false: b}[c]` |
| SIM107 | use-compound-assignment | 复合赋值 | `x = x + 1` | `x += 1` |
| SIM108 | use-increment | 自增 | `x = x + 1` | `x++` |
| SIM109 | merge-nested-if | 合并嵌套 if | `if a { if b { ... } }` | `if a && b { ... }` |
| SIM110 | simplify-len-check | 简化长度检查 | `len(x) > 0` | `len(x) != 0` |
| SIM111 | use-copy | 使用 copy | 循环复制 slice | `copy(dst, src)` |
| SIM112 | use-append | 使用 append | 循环添加元素 | `append(slice, elems...)` |

---

## S 系列 - style（风格与命名）

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| S001 | camel-case-function | 函数名应为驼峰 | `func foo_bar()` |
| S002 | pascal-case-exported | 导出符号应大写 | `func foo()` 应为 `Foo()` |
| S003 | no-underscore-in-name | 名称不应有下划线 | `var foo_bar` |
| S004 | receiver-name-short | 接收器名应简短 | `func (receiver *Type)` |
| S005 | receiver-name-consistent | 接收器名应一致 | 同类型不同方法不同名 |
| S006 | interface-name | 接口名应有 I 前缀 | 或后缀 -er |
| S007 | error-var-naming | error 变量命名 | 应为 ErrXxx |
| S008 | const-naming | 常量命名 | 应为驼峰或全大写 |
| S009 | type-acronym | 类型首字母大写 | `type httpClient` |
| S010 | package-name | 包名规范 | 不应含下划线 |

---

## D 系列 - docs（文档与注释）

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| D001 | missing-doc | 导出符号缺少文档 | `func Foo()` 无注释 |
| D002 | package-doc-missing | 包缺少文档 | 无 package 注释 |
| D003 | todo-format | TODO 格式错误 | `// todo: xxx` |
| D004 | deprecated-format | Deprecated 格式错误 | `// deprecated` |
| D005 | comment-spelling | 注释拼写错误 | 常见拼写错误 |
| D006 | comment-grammar | 注释语法问题 | 首字母未大写 |
| D007 | comment-period | 注释缺少句号 | 句子未以句号结尾 |

---

## P 系列 - performance（性能优化）

| 代码 | 名称 | 描述 | 优化前 | 优化后 |
|------|------|------|--------|--------|
| P001 | preallocate-slice | 预分配 slice | `var s []int; for { s = append(s, x) }` | `make([]int, 0, n)` |
| P002 | string-builder | 使用 strings.Builder | `s += x` 循环 | `var b strings.Builder` |
| P003 | avoid-unnecessary-copy | 避免不必要拷贝 | `for _, v := range slice { use(v) }` | `for i := range slice { use(&slice[i]) }` |
| P004 | buffer-pool | 使用 sync.Pool | 频繁创建 bytes.Buffer | pool.Get/ Put |
| P005 | map-capacity | 指定 map 容量 | `make(map[string]int)` | `make(map[string]int, n)` |
| P006 | closure-alloc | 闭包分配优化 | 循环内捕获变量 | 传参替代捕获 |

---

## C 系列 - concurrency（并发问题）

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| C001 | race-condition | 数据竞争 | 无锁访问共享变量 |
| C002 | mutex-copy | 互斥锁拷贝 | `m2 := m1` (sync.Mutex) |
| C003 | channel-close | channel 关闭问题 | 向已关闭 channel 发送 |
| C004 | goroutine-leak | goroutine 泄漏 | 无退出条件的 goroutine |
| C005 | select-default | select 中 default | 忙等待模式 |

---

## SEC 系列 - security（安全问题）

| 代码 | 名称 | 描述 | 示例 |
|------|------|------|------|
| SEC001 | sql-injection | SQL 注入 | `db.Query("SELECT * FROM t WHERE id = " + id)` |
| SEC002 | command-injection | 命令注入 | `exec.Command("sh", "-c", userInput)` |
| SEC003 | path-traversal | 路径遍历 | `os.Open(userPath)` |
| SEC004 | insecure-random | 不安全随机数 | `math/rand` 用于安全场景 |
| SEC005 | hardcoded-password | 硬编码密码 | `password := "admin123"` |

---

## 规则统计

| 系列 | 代码 | 数量 | 优先级 | 状态 |
|------|------|------|--------|------|
| E - gocodestyle | E1xx-E7xx | 14 | P0 (必须) | ✅ 已实现 |
| F - goflakes | F4xx-F9xx | 12 | P0 (必须) | ✅ 已实现 |
| I - goimports | I0xx | 7 | P0 (必须) | ✅ 已实现 |
| S - style | S001-S008 | 8 | P1 (推荐) | ✅ 已实现 |
| D - docs | D001-D004 | 4 | P2 (可选) | ✅ 已实现 |
| B - go-bugbear | B0xx-B4xx | 9 | P1 (推荐) | ✅ 已实现 |
| UP - goupgrade | UP0xx | 10 | P2 (可选) | 📝 计划中 |
| SIM - simplify | SIM1xx | 12 | P1 (推荐) | 📝 计划中 |
| P - performance | P0xx | 6 | P2 (可选) | 📝 计划中 |
| C - concurrency | C0xx | 5 | P1 (推荐) | 📝 计划中 |
| SEC - security | SECxxx | 5 | P1 (推荐) | 📝 计划中 |

**当前实现: 54 个规则**
**总计规划: 116 个规则**

---

## 实施路线图

### Phase 1: 核心规则 (P0) - ✅ 已完成 33个规则
- [x] E 系列: 布局、导入、比较 (14个)
  - E101, E115-E118 (格式)
  - E201-E203 (导入)
  - E301-E302, E401 (结构)
  - E712-E713, E721 (比较)
- [x] F 系列: 未使用、变量问题 (12个)
  - F401-F405 (未使用)
  - F811-F812, F831, F841 (变量问题)
  - F901-F903 (控制流)
- [x] I 系列: 导入排序 (7个)
  - I001-I004, I006-I008

### Phase 2: 进行中 - 54个规则 ✅ 已实现 21个
- [x] S 系列: 风格规范 (8个已实现)
  - S001-S008
- [x] D 系列: 文档规范 (4个已实现)
  - D001-D004
- [x] B 系列: Uber Go Style / Bugbear (9个已实现)
  - B001-B004: 并发规范
  - B101: 性能优化
  - B201-B202: 防御性编程
  - B301-B302: 资源管理
- [ ] SIM 系列: 简化建议 (12个待实现)
- [ ] C 系列: 并发问题 (5个待实现)
- [ ] SEC 系列: 安全问题 (5个待实现)

### Phase 3: 计划中 (P2) - 28个规则
- [ ] UP 系列: 语法升级 (10个)
- [ ] P 系列: 性能优化 (6个)
- [ ] B 系列剩余: 资源管理 (5个)
- [ ] D 系列扩展: (3个)

---

## 使用示例

```bash
# 启用所有规则
woof check . --all

# 仅启用特定系列
woof check . --select E,F,I

# 忽略某些规则
woof check . --ignore UP001,SIM106

# 性能分析模式
GOLOGGING=perf woof check .
```

---

## 参考

- Python pycodestyle: https://pycodestyle.pycqa.org/
- Python Pyflakes: https://github.com/PyCQA/pyflakes
- Python flake8-bugbear: https://github.com/PyCQA/flake8-bugbear
- Python isort: https://pycqa.github.io/isort/
- Python pyupgrade: https://github.com/asottile/pyupgrade
