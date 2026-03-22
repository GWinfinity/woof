# Go Linter Crawler 报告

## 概述

本报告汇总了从 Go 社区抓取的各种 linter 规则，用于充实 woof 的规则体系。

## 抓取的 Linter 源

- **staticcheck**: 270 条规则
- **go-critic**: 202 条规则
- **revive**: 154 条规则

## 规则分类


### 逻辑错误 (F-series) (190条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| SA1000 | sa1000 | Invalid regular expression... | staticcheck |
| SA1001 | sa1001 | Invalid template... | staticcheck |
| SA1002 | sa1002 | Invalid format in time.Parse... | staticcheck |
| SA1003 | sa1003 | Unsupported argument to functions in encoding/bina... | staticcheck |
| SA1004 | sa1004 | Suspiciously small untyped constant in time.Sleep... | staticcheck |
| SA1005 | sa1005 | Invalid first argument to exec.Command... | staticcheck |
| SA1006 | sa1006 | Printf with dynamic first argument and no further ... | staticcheck |
| SA1007 | sa1007 | Invalid URL in net/url.Parse... | staticcheck |
| SA1008 | sa1008 | Non-canonical key in http.Header map... | staticcheck |
| SA1010 | sa1010 | (*regexp.Regexp).FindAll called with n == 0, which... | staticcheck |
| SA1011 | sa1011 | Various methods in the 'strings' package with inva... | staticcheck |
| SA1012 | sa1012 | A nil context is being passed to a function... | staticcheck |
| SA1013 | sa1013 | io.Seeker.Seek is being called with the wrong cons... | staticcheck |
| SA1014 | sa1014 | Non-pointer value passed to Unmarshal or Decode... | staticcheck |
| SA1015 | sa1015 | Using time.Tick in a way that leaks memory... | staticcheck |
| SA1016 | sa1016 | Trapping a signal that cannot be trapped... | staticcheck |
| SA1017 | sa1017 | Channels used with os/signal.Notify should be buff... | staticcheck |
| SA1018 | sa1018 | strings.Replace called with n == 0, which does not... | staticcheck |
| SA1019 | sa1019 | Using a deprecated function, variable, constant or... | staticcheck |
| SA1020 | sa1020 | Using an invalid host:port pair with a net.Listen-... | staticcheck |
| ... | ... | 还有 170 条规则 | ... |

### 代码质量 (B-series) (16条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| REVIVE_ARGUMENT_LIMIT | argument-limit | Limits the number of arguments... | revive |
| REVIVE_COGNITIVE_COMPLEXITY | cognitive-complexity | Checks cognitive complexity of functions... | revive |
| REVIVE_CYCLOMATIC | cyclomatic | Checks cyclomatic complexity... | revive |
| REVIVE_FUNCTION_LENGTH | function-length | Checks function length... | revive |
| REVIVE_FUNCTION_RESULT_LIMIT | function-result-limit | Limits function results... | revive |
| REVIVE_MAX_CONTROL_NESTING | max-control-nesting | Limits control nesting... | revive |
| REVIVE_MAX_PUBLIC_STRUCTS | max-public-structs | Limits public structs... | revive |
| REVIVE_NESTED_STRUCTS | nested-structs | Checks for nested structs... | revive |
| REVIVE_ARGUMENT_LIMIT_1 | argument-limit | Limits the number of arguments... | revive |
| REVIVE_COGNITIVE_COMPLEXITY_1 | cognitive-complexity | Checks cognitive complexity of functions... | revive |
| REVIVE_CYCLOMATIC_1 | cyclomatic | Checks cyclomatic complexity... | revive |
| REVIVE_FUNCTION_LENGTH_1 | function-length | Checks function length... | revive |
| REVIVE_FUNCTION_RESULT_LIMIT_1 | function-result-limit | Limits function results... | revive |
| REVIVE_MAX_CONTROL_NESTING_1 | max-control-nesting | Limits control nesting... | revive |
| REVIVE_MAX_PUBLIC_STRUCTS_1 | max-public-structs | Limits public structs... | revive |
| REVIVE_NESTED_STRUCTS_1 | nested-structs | Checks for nested structs... | revive |

### 简化建议 (SIM-series) (130条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| SA4000 | sa4000 | Binary operator has identical expressions on both ... | staticcheck |
| SA4001 | sa4001 | &*x gets simplified to x, it does not copy x... | staticcheck |
| SA4003 | sa4003 | Comparing unsigned values against negative values ... | staticcheck |
| SA4004 | sa4004 | The loop exits unconditionally after one iteration... | staticcheck |
| SA4005 | sa4005 | Field assignment that will never be observed... | staticcheck |
| SA4006 | sa4006 | A value assigned to a variable is never read befor... | staticcheck |
| SA4008 | sa4008 | The variable in the loop condition never changes... | staticcheck |
| SA4009 | sa4009 | A function argument is overwritten before its firs... | staticcheck |
| SA4010 | sa4010 | The result of append will never be observed... | staticcheck |
| SA4011 | sa4011 | Break statement with no effect... | staticcheck |
| SA4012 | sa4012 | Comparing a value against NaN even though no value... | staticcheck |
| SA4013 | sa4013 | Negating a boolean twice (!!b) is the same as writ... | staticcheck |
| SA4014 | sa4014 | An if/else if chain has repeated conditions and no... | staticcheck |
| SA4015 | sa4015 | Calling functions like math.Ceil on floats convert... | staticcheck |
| SA4016 | sa4016 | Certain bitwise operations, such as x ^ 0, do not ... | staticcheck |
| SA4017 | sa4017 | Discarding the return values of a function without... | staticcheck |
| SA4018 | sa4018 | Self-assignment of variables... | staticcheck |
| SA4019 | sa4019 | Multiple, identical build constraints in the same ... | staticcheck |
| SA4020 | sa4020 | Unreachable case clause in a type switch... | staticcheck |
| SA4021 | sa4021 | x = append(y) is equivalent to x = y... | staticcheck |
| ... | ... | 还有 110 条规则 | ... |

### 风格规范 (S-series) (242条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| ST1000 | st1000 | Incorrectly or inconsistently package comment... | staticcheck |
| ST1003 | st1003 | Poorly chosen identifier... | staticcheck |
| ST1005 | st1005 | Incorrectly formatted error string... | staticcheck |
| ST1006 | st1006 | Poorly chosen receiver name... | staticcheck |
| ST1008 | st1008 | A function's error value should be its last return... | staticcheck |
| ST1011 | st1011 | Manually assigning to individual struct fields... | staticcheck |
| ST1012 | st1012 | Poorly chosen variable name for error value... | staticcheck |
| ST1013 | st1013 | Should use constants for HTTP status code... | staticcheck |
| ST1015 | st1015 | A switch's default case should be first or last... | staticcheck |
| ST1016 | st1016 | Use consistent method receiver names... | staticcheck |
| ST1017 | st1017 | Don't use Yoda conditions... | staticcheck |
| ST1018 | st1018 | Avoid zero-width and control characters in string ... | staticcheck |
| ST1019 | st1019 | Importing the same package multiple times... | staticcheck |
| ST1020 | st1020 | Documentation of an exported function should start... | staticcheck |
| ST1021 | st1021 | Documentation of an exported type should start wit... | staticcheck |
| ST1022 | st1022 | Documentation of an exported variable or constant ... | staticcheck |
| ST1023 | st1023 | Redundant type in variable declaration... | staticcheck |
| ST1000_1 | st1000 | Incorrectly or inconsistently package comment... | staticcheck |
| ST1003_1 | st1003 | Poorly chosen identifier... | staticcheck |
| ST1005_1 | st1005 | Incorrectly formatted error string... | staticcheck |
| ... | ... | 还有 222 条规则 | ... |

### 文档规范 (D-series) (4条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| SA3000 | sa3000 | TestMain doesn't call os.Exit, hiding test failure... | staticcheck |
| SA3001 | sa3001 | Assigning to b.N in benchmarks distorts the result... | staticcheck |
| SA3000_1 | sa3000 | TestMain doesn't call os.Exit, hiding test failure... | staticcheck |
| SA3001_1 | sa3001 | Assigning to b.N in benchmarks distorts the result... | staticcheck |

### 性能优化 (P-series) (32条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| SA6000 | sa6000 | Using regexp.Match or related in a loop, should us... | staticcheck |
| SA6001 | sa6001 | Missing an optimization opportunity when indexing ... | staticcheck |
| SA6002 | sa6002 | Storing non-pointer values in sync.Pool allocates ... | staticcheck |
| SA6003 | sa6003 | Converting a string to a slice of runes before ran... | staticcheck |
| SA6005 | sa6005 | Inefficient string comparison with strings.ToLower... | staticcheck |
| SA6006 | sa6006 | Using io.WriteString to write a byte slice... | staticcheck |
| SA6000_1 | sa6000 | Using regexp.Match or related in a loop, should us... | staticcheck |
| SA6001_1 | sa6001 | Missing an optimization opportunity when indexing ... | staticcheck |
| SA6002_1 | sa6002 | Storing non-pointer values in sync.Pool allocates ... | staticcheck |
| SA6003_1 | sa6003 | Converting a string to a slice of runes before ran... | staticcheck |
| SA6005_1 | sa6005 | Inefficient string comparison with strings.ToLower... | staticcheck |
| SA6006_1 | sa6006 | Using io.WriteString to write a byte slice... | staticcheck |
| CRITIC_APPENDCOMBINE | appendCombine | Detect append chains that can be combined... | go-critic |
| CRITIC_EQUALFOLD | equalFold | Detects string equality checks that can be replace... | go-critic |
| CRITIC_HUGEPARAM | hugeParam | Detects params that are too large... | go-critic |
| CRITIC_INDEXALLOC | indexAlloc | Detects strings.Index calls that may cause allocat... | go-critic |
| CRITIC_PREFERWRITEBYTE | preferWriteByte | Suggests using WriteByte... | go-critic |
| CRITIC_RANGEEXPRCOPY | rangeExprCopy | Detects expensive copies in range expressions... | go-critic |
| CRITIC_RANGEVALCOPY | rangeValCopy | Detects expensive copies in range loops... | go-critic |
| CRITIC_SQLQUERY | sqlQuery | Detects issues with sql queries... | go-critic |
| ... | ... | 还有 12 条规则 | ... |

### 并发问题 (C-series) (12条)

| 代码 | 名称 | 描述 | 来源 |
|------|------|------|------|
| SA2000 | sa2000 | sync.WaitGroup.Add called inside the goroutine, le... | staticcheck |
| SA2001 | sa2001 | Empty critical section, did you mean to defer the ... | staticcheck |
| SA2002 | sa2002 | Called testing.T.FailNow or SkipNow in a goroutine... | staticcheck |
| SA2003 | sa2003 | Deferred Lock right after locking, likely meant to... | staticcheck |
| SA2000_1 | sa2000 | sync.WaitGroup.Add called inside the goroutine, le... | staticcheck |
| SA2001_1 | sa2001 | Empty critical section, did you mean to defer the ... | staticcheck |
| SA2002_1 | sa2002 | Called testing.T.FailNow or SkipNow in a goroutine... | staticcheck |
| SA2003_1 | sa2003 | Deferred Lock right after locking, likely meant to... | staticcheck |
| CRITIC_BADLOCK | badLock | Detect common mistakes with locks... | go-critic |
| CRITIC_BADLOCK_1 | badLock | Detect common mistakes with locks... | go-critic |
| REVIVE_DATARACE | datarace | Checks for potential data races... | revive |
| REVIVE_DATARACE_1 | datarace | Checks for potential data races... | revive |

## 建议实现优先级

### P0 - 必须实现 (核心规则)

共 278 条核心规则，包括:

- `SA1000`: sa1000 (staticcheck)
- `SA1001`: sa1001 (staticcheck)
- `SA1002`: sa1002 (staticcheck)
- `SA1003`: sa1003 (staticcheck)
- `SA1004`: sa1004 (staticcheck)
- `SA1005`: sa1005 (staticcheck)
- `SA1006`: sa1006 (staticcheck)
- `SA1007`: sa1007 (staticcheck)
- `SA1008`: sa1008 (staticcheck)
- `SA1010`: sa1010 (staticcheck)
- `SA1011`: sa1011 (staticcheck)
- `SA1012`: sa1012 (staticcheck)
- `SA1013`: sa1013 (staticcheck)
- `SA1014`: sa1014 (staticcheck)
- `SA1015`: sa1015 (staticcheck)

### P1 - 推荐实现

包括性能优化、代码质量、风格规范等规则。

### P2 - 可选实现

包括文档规范、简化建议等规则。

## 新特性覆盖

### 泛型 (Generics) 相关规则

找到 0 条泛型相关规则

### Fuzzing 相关规则

找到 0 条 Fuzzing 相关规则

### Workspace 相关规则

找到 0 条 Workspace 相关规则

## 使用建议

1. **逐步集成**: 优先实现 P0 级别规则，确保基础功能完善
2. **参考实现**: 可以参考原始 linter 的实现逻辑
3. **测试覆盖**: 为每条规则添加测试用例
4. **文档同步**: 更新 RULES_CATALOG.md 文档

## 参考链接

- [golangci-lint](https://golangci-lint.run/)
- [staticcheck](https://staticcheck.io/)
- [go-critic](https://go-critic.com/)
- [revive](https://github.com/mgechev/revive)
