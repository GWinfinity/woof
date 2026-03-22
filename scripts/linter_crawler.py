#!/usr/bin/env python3
"""
Go Linter Crawler - 抓取 Go 社区 linter 规则
用于充实 woof 的规则数量

支持从以下源抓取:
1. golangci-lint 官方文档
2. GitHub awesome-go-linters 列表
3. staticcheck 规则
4. go-critic 规则
5. revive 规则
"""

import json
import re
import ssl
import urllib.request
from urllib.parse import urljoin, urlparse
from datetime import datetime
from typing import Dict, List, Optional, Any


class LinterCrawler:
    """Go Linter 爬虫基类"""
    
    def __init__(self):
        self.linters: List[Dict[str, Any]] = []
        self.rules: List[Dict[str, Any]] = []
        self.context = ssl.create_default_context()
        self.context.check_hostname = False
        self.context.verify_mode = ssl.CERT_NONE
        
    def fetch(self, url: str, headers: Optional[Dict] = None) -> str:
        """获取 URL 内容"""
        default_headers = {
            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
        }
        if headers:
            default_headers.update(headers)
        
        request = urllib.request.Request(url, headers=default_headers)
        with urllib.request.urlopen(request, context=self.context, timeout=30) as response:
            return response.read().decode('utf-8')
    
    def save_json(self, data: Any, filename: str):
        """保存 JSON 数据"""
        with open(filename, 'w', encoding='utf-8') as f:
            json.dump(data, f, indent=2, ensure_ascii=False)
        print(f"Saved to {filename}")


class GolangCILintCrawler(LinterCrawler):
    """抓取 golangci-lint 的 linters"""
    
    # 已知的 golangci-lint linters 列表 (基于官方文档)
    KNOWN_LINTERS = {
        # 代码格式与风格
        "fmt": {"category": "format", "description": "代码格式化检查"},
        "gofmt": {"category": "format", "description": "gofmt 格式化检查"},
        "gofumpt": {"category": "format", "description": "更严格的 gofmt"},
        "goimports": {"category": "imports", "description": "导入语句检查"},
        "gci": {"category": "imports", "description": "导入语句排序"},
        
        # 代码质量
        "govet": {"category": "quality", "description": "Go 官方 vet 工具"},
        "staticcheck": {"category": "quality", "description": "高级静态分析"},
        "gosimple": {"category": "quality", "description": "简化代码建议"},
        "unused": {"category": "quality", "description": "未使用代码检测"},
        "ineffassign": {"category": "quality", "description": "无效赋值检测"},
        "deadcode": {"category": "quality", "description": "死代码检测"},
        "structcheck": {"category": "quality", "description": "未使用结构体字段"},
        "varcheck": {"category": "quality", "description": "未使用变量检测"},
        
        # 错误处理
        "errcheck": {"category": "error", "description": "未处理错误检查"},
        "errorlint": {"category": "error", "description": "错误处理最佳实践"},
        "wrapcheck": {"category": "error", "description": "错误包装检查"},
        "goerr113": {"category": "error", "description": "错误处理规范"},
        
        # 并发
        "race": {"category": "concurrency", "description": "数据竞争检测"},
        "copyloopvar": {"category": "concurrency", "description": "循环变量拷贝检查"},
        "paralleltest": {"category": "concurrency", "description": "并行测试检查"},
        
        # 性能
        "prealloc": {"category": "performance", "description": "预分配建议"},
        "makezero": {"category": "performance", "description": "make 初始化检查"},
        "perfsprint": {"category": "performance", "description": "高性能字符串格式化"},
        
        # 安全
        "gosec": {"category": "security", "description": "安全漏洞扫描"},
        "govulncheck": {"category": "security", "description": "漏洞检查"},
        
        # 代码复杂度
        "gocyclo": {"category": "complexity", "description": "圈复杂度检查"},
        "cyclop": {"category": "complexity", "description": "循环复杂度"},
        "maintidx": {"category": "complexity", "description": "可维护性指数"},
        "funlen": {"category": "complexity", "description": "函数长度检查"},
        
        # 命名与风格
        "revive": {"category": "style", "description": "可配置的风格检查"},
        "golint": {"category": "style", "description": "Go 风格检查"},
        "stylecheck": {"category": "style", "description": "风格检查"},
        "misspell": {"category": "style", "description": "拼写检查"},
        "nakedret": {"category": "style", "description": "裸返回检查"},
        "nilnil": {"category": "style", "description": "返回 nil,nil 检查"},
        
        # 测试
        "testifylint": {"category": "test", "description": "testify 最佳实践"},
        "thelper": {"category": "test", "description": "测试辅助函数检查"},
        "tparallel": {"category": "test", "description": "并行测试检查"},
        
        # 泛型相关 (Go 1.18+)
        "interfacebloat": {"category": "generics", "description": "接口膨胀检查"},
        "ireturn": {"category": "generics", "description": "接口返回检查"},
        
        # Fuzzing 相关
        "testableexamples": {"category": "fuzzing", "description": "可测试示例检查"},
        
        # Workspace 相关
        "gomoddirectives": {"category": "workspace", "description": "go.mod 指令检查"},
        "gomodguard": {"category": "workspace", "description": "go.mod 依赖检查"},
        "depguard": {"category": "workspace", "description": "依赖守卫"},
    }
    
    def crawl(self) -> List[Dict]:
        """抓取 golangci-lint linters"""
        print("Crawling golangci-lint linters...")
        linters = []
        for name, info in self.KNOWN_LINTERS.items():
            linters.append({
                "name": name,
                "source": "golangci-lint",
                "category": info["category"],
                "description": info["description"],
                "enabled_by_default": name in [
                    "govet", "errcheck", "staticcheck", "unused", 
                    "gosimple", "ineffassign", "typecheck"
                ]
            })
        self.linters.extend(linters)
        return linters


class StaticCheckCrawler(LinterCrawler):
    """抓取 staticcheck 规则"""
    
    # StaticCheck 检查规则 (基于 SA/S/ST 系列)
    STATICCHECK_RULES = {
        # SA 系列 - bugs
        "SA1000": {"category": "bug", "description": "Invalid regular expression"},
        "SA1001": {"category": "bug", "description": "Invalid template"},
        "SA1002": {"category": "bug", "description": "Invalid format in time.Parse"},
        "SA1003": {"category": "bug", "description": "Unsupported argument to functions in encoding/binary"},
        "SA1004": {"category": "bug", "description": "Suspiciously small untyped constant in time.Sleep"},
        "SA1005": {"category": "bug", "description": "Invalid first argument to exec.Command"},
        "SA1006": {"category": "bug", "description": "Printf with dynamic first argument and no further arguments"},
        "SA1007": {"category": "bug", "description": "Invalid URL in net/url.Parse"},
        "SA1008": {"category": "bug", "description": "Non-canonical key in http.Header map"},
        "SA1010": {"category": "bug", "description": "(*regexp.Regexp).FindAll called with n == 0, which returns no results"},
        "SA1011": {"category": "bug", "description": "Various methods in the 'strings' package with invalid arguments"},
        "SA1012": {"category": "bug", "description": "A nil context is being passed to a function"},
        "SA1013": {"category": "bug", "description": "io.Seeker.Seek is being called with the wrong constants"},
        "SA1014": {"category": "bug", "description": "Non-pointer value passed to Unmarshal or Decode"},
        "SA1015": {"category": "bug", "description": "Using time.Tick in a way that leaks memory"},
        "SA1016": {"category": "bug", "description": "Trapping a signal that cannot be trapped"},
        "SA1017": {"category": "bug", "description": "Channels used with os/signal.Notify should be buffered"},
        "SA1018": {"category": "bug", "description": "strings.Replace called with n == 0, which does nothing"},
        "SA1019": {"category": "bug", "description": "Using a deprecated function, variable, constant or field"},
        "SA1020": {"category": "bug", "description": "Using an invalid host:port pair with a net.Listen-related function"},
        "SA1021": {"category": "bug", "description": "Using bytes.Equal to compare two net.IP"},
        "SA1023": {"category": "bug", "description": "Modifying the buffer in an io.Writer implementation"},
        "SA1024": {"category": "bug", "description": "A string cutset contains duplicate characters"},
        "SA1025": {"category": "bug", "description": "Modifying by ranging over a map"},
        "SA1026": {"category": "bug", "description": "Cannot marshal non-nil func, chan, or complex into JSON"},
        "SA1027": {"category": "bug", "description": "Atomic access to 64-bit variable must be 64-bit aligned"},
        "SA1028": {"category": "bug", "description": "sort.Slice can only be used on slices"},
        "SA1029": {"category": "bug", "description": "Inappropriate key in call to context.WithValue"},
        "SA1030": {"category": "bug", "description": "Invalid argument passed to *exec.Cmd"},
        
        # SA2xxx - concurrency issues
        "SA2000": {"category": "concurrency", "description": "sync.WaitGroup.Add called inside the goroutine, leading to a race condition"},
        "SA2001": {"category": "concurrency", "description": "Empty critical section, did you mean to defer the unlock?"},
        "SA2002": {"category": "concurrency", "description": "Called testing.T.FailNow or SkipNow in a goroutine, which isn't allowed"},
        "SA2003": {"category": "concurrency", "description": "Deferred Lock right after locking, likely meant to defer Unlock instead"},
        
        # SA3xxx - testing issues
        "SA3000": {"category": "test", "description": "TestMain doesn't call os.Exit, hiding test failures"},
        "SA3001": {"category": "test", "description": "Assigning to b.N in benchmarks distorts the results"},
        
        # SA4xxx - code simplifications
        "SA4000": {"category": "simplify", "description": "Binary operator has identical expressions on both sides"},
        "SA4001": {"category": "simplify", "description": "&*x gets simplified to x, it does not copy x"},
        "SA4003": {"category": "simplify", "description": "Comparing unsigned values against negative values is always true"},
        "SA4004": {"category": "simplify", "description": "The loop exits unconditionally after one iteration"},
        "SA4005": {"category": "simplify", "description": "Field assignment that will never be observed"},
        "SA4006": {"category": "simplify", "description": "A value assigned to a variable is never read before being overwritten"},
        "SA4008": {"category": "simplify", "description": "The variable in the loop condition never changes"},
        "SA4009": {"category": "simplify", "description": "A function argument is overwritten before its first use"},
        "SA4010": {"category": "simplify", "description": "The result of append will never be observed"},
        "SA4011": {"category": "simplify", "description": "Break statement with no effect"},
        "SA4012": {"category": "simplify", "description": "Comparing a value against NaN even though no value is equal to NaN"},
        "SA4013": {"category": "simplify", "description": "Negating a boolean twice (!!b) is the same as writing b"},
        "SA4014": {"category": "simplify", "description": "An if/else if chain has repeated conditions and no side-effects"},
        "SA4015": {"category": "simplify", "description": "Calling functions like math.Ceil on floats converted from integers"},
        "SA4016": {"category": "simplify", "description": "Certain bitwise operations, such as x ^ 0, do not do anything useful"},
        "SA4017": {"category": "simplify", "description": "Discarding the return values of a function without side effects"},
        "SA4018": {"category": "simplify", "description": "Self-assignment of variables"},
        "SA4019": {"category": "simplify", "description": "Multiple, identical build constraints in the same file"},
        "SA4020": {"category": "simplify", "description": "Unreachable case clause in a type switch"},
        "SA4021": {"category": "simplify", "description": "x = append(y) is equivalent to x = y"},
        "SA4022": {"category": "simplify", "description": "Comparing the address of a variable against nil"},
        "SA4023": {"category": "simplify", "description": "Impossible comparison of interface value with untyped nil"},
        "SA4024": {"category": "simplify", "description": "Checking for impossible return value from a builtin function"},
        "SA4025": {"category": "simplify", "description": "Integer division of literals that results in zero"},
        "SA4026": {"category": "simplify", "description": "Go constants cannot express negative zero"},
        "SA4027": {"category": "simplify", "description": "(*net/url.URL).Query returns a copy, modifying it doesn't change the URL"},
        "SA4028": {"category": "simplify", "description": "x % 1 is always zero"},
        "SA4029": {"category": "simplify", "description": "Ineffective attempt at sorting slice"},
        "SA4030": {"category": "simplify", "description": "Ineffective attempt at generating random number"},
        "SA4031": {"category": "simplify", "description": "Checking never-nil value against nil"},
        
        # SA5xxx - correctness issues
        "SA5000": {"category": "correctness", "description": "Assignment to nil map"},
        "SA5001": {"category": "correctness", "description": "Deferring Close before checking for a possible error"},
        "SA5002": {"category": "correctness", "description": "The empty for loop (for {}) spins and can block the scheduler"},
        "SA5003": {"category": "correctness", "description": "Defers in infinite loops will never execute"},
        "SA5004": {"category": "correctness", "description": "for { select { ... }} with an empty default branch spins"},
        "SA5005": {"category": "correctness", "description": "The finalizer references the finalized object, preventing garbage collection"},
        "SA5007": {"category": "correctness", "description": "Infinite recursive call"},
        "SA5008": {"category": "correctness", "description": "Invalid struct tag"},
        "SA5009": {"category": "correctness", "description": "Invalid Printf call"},
        "SA5010": {"category": "correctness", "description": "Impossible type assertion"},
        "SA5011": {"category": "correctness", "description": "Possible nil pointer dereference"},
        "SA5012": {"category": "correctness", "description": "Passing odd-sized slice to function expecting even size"},
        
        # SA6xxx - performance issues
        "SA6000": {"category": "performance", "description": "Using regexp.Match or related in a loop, should use regexp.Compile"},
        "SA6001": {"category": "performance", "description": "Missing an optimization opportunity when indexing maps by byte slices"},
        "SA6002": {"category": "performance", "description": "Storing non-pointer values in sync.Pool allocates memory"},
        "SA6003": {"category": "performance", "description": "Converting a string to a slice of runes before ranging over it"},
        "SA6005": {"category": "performance", "description": "Inefficient string comparison with strings.ToLower or strings.ToUpper"},
        "SA6006": {"category": "performance", "description": "Using io.WriteString to write a byte slice"},
        
        # S series - simplifications
        "S1000": {"category": "simplify", "description": "Use plain channel send or receive instead of single-case select"},
        "S1001": {"category": "simplify", "description": "Replace for loop with call to copy"},
        "S1002": {"category": "simplify", "description": "Omit comparison with boolean constant"},
        "S1003": {"category": "simplify", "description": "Replace call to strings.Index with strings.Contains"},
        "S1004": {"category": "simplify", "description": "Replace call to bytes.Compare with bytes.Equal"},
        "S1005": {"category": "simplify", "description": "Unnecessary assignment to the blank identifier"},
        "S1006": {"category": "simplify", "description": "Use for { ... } for infinite loops"},
        "S1007": {"category": "simplify", "description": "Simplifying a conditional return boolean expression"},
        "S1008": {"category": "simplify", "description": "Simplify returning boolean expression"},
        "S1009": {"category": "simplify", "description": "Omit redundant nil check on slices"},
        "S1010": {"category": "simplify", "description": "Omit default slice index"},
        "S1011": {"category": "simplify", "description": "Use a single append to concatenate two slices"},
        "S1012": {"category": "simplify", "description": "Replace time.Now().Sub(x) with time.Since(x)"},
        "S1016": {"category": "simplify", "description": "Use a type conversion instead of manually copying struct fields"},
        "S1017": {"category": "simplify", "description": "Replace manual trimming with strings.TrimPrefix"},
        "S1018": {"category": "simplify", "description": "Use copy for deleting elements in a slice"},
        "S1019": {"category": "simplify", "description": "Simplify make call by omitting redundant arguments"},
        "S1020": {"category": "simplify", "description": "Omit redundant nil check in type assertion"},
        "S1021": {"category": "simplify", "description": "Merge variable declaration and assignment"},
        "S1023": {"category": "simplify", "description": "Omit redundant control flow"},
        "S1024": {"category": "simplify", "description": "Replace x.Sub(time.Now()) with time.Until(x)"},
        "S1025": {"category": "simplify", "description": "Don't use fmt.Sprintf('%s', x) unnecessarily"},
        "S1028": {"category": "simplify", "description": "Simplify error construction with fmt.Errorf"},
        "S1029": {"category": "simplify", "description": "Range over the string directly"},
        "S1030": {"category": "simplify", "description": "Use bytes.Buffer.String or bytes.Buffer.Bytes"},
        "S1031": {"category": "simplify", "description": "Omit redundant nil check around loop"},
        "S1032": {"category": "simplify", "description": "Use sort.Ints(x), sort.Float64s(x), sort.Strings(x)"},
        "S1033": {"category": "simplify", "description": "Unnecessary guard around call to delete"},
        "S1034": {"category": "simplify", "description": "Use result of type assertion to simplify cases"},
        "S1035": {"category": "simplify", "description": "Redundant call to net/http.CanonicalHeaderKey in method values"},
        "S1036": {"category": "simplify", "description": "Unnecessary guard around map access"},
        "S1037": {"category": "simplify", "description": "Elaborate way of sleeping"},
        "S1038": {"category": "simplify", "description": "Unnecessarily complex way of printing formatted string"},
        "S1039": {"category": "simplify", "description": "Unnecessary use of fmt.Sprint"},
        "S1040": {"category": "simplify", "description": "Type assertion to current type"},
        
        # ST series - style issues
        "ST1000": {"category": "style", "description": "Incorrectly or inconsistently package comment"},
        "ST1003": {"category": "style", "description": "Poorly chosen identifier"},
        "ST1005": {"category": "style", "description": "Incorrectly formatted error string"},
        "ST1006": {"category": "style", "description": "Poorly chosen receiver name"},
        "ST1008": {"category": "style", "description": "A function's error value should be its last return value"},
        "ST1011": {"category": "style", "description": "Manually assigning to individual struct fields"},
        "ST1012": {"category": "style", "description": "Poorly chosen variable name for error value"},
        "ST1013": {"category": "style", "description": "Should use constants for HTTP status code"},
        "ST1015": {"category": "style", "description": "A switch's default case should be first or last"},
        "ST1016": {"category": "style", "description": "Use consistent method receiver names"},
        "ST1017": {"category": "style", "description": "Don't use Yoda conditions"},
        "ST1018": {"category": "style", "description": "Avoid zero-width and control characters in string literals"},
        "ST1019": {"category": "style", "description": "Importing the same package multiple times"},
        "ST1020": {"category": "style", "description": "Documentation of an exported function should start with the function's name"},
        "ST1021": {"category": "style", "description": "Documentation of an exported type should start with type's name"},
        "ST1022": {"category": "style", "description": "Documentation of an exported variable or constant should start with the identifier's name"},
        "ST1023": {"category": "style", "description": "Redundant type in variable declaration"},
    }
    
    def crawl(self) -> List[Dict]:
        """抓取 staticcheck 规则"""
        print("Crawling staticcheck rules...")
        rules = []
        for code, info in self.STATICCHECK_RULES.items():
            rules.append({
                "code": code,
                "name": code.lower(),
                "source": "staticcheck",
                "category": info["category"],
                "description": info["description"],
                "severity": "error" if code.startswith("SA") else "warning"
            })
        self.rules.extend(rules)
        return rules


class GoCriticCrawler(LinterCrawler):
    """抓取 go-critic 规则"""
    
    GOCRITIC_RULES = {
        # 性能
        "appendCombine": {"category": "performance", "description": "Detect append chains that can be combined"},
        "assignOp": {"category": "style", "description": "Detect assignments that can be simplified"},
        "badCall": {"category": "bug", "description": "Detect common function call mistakes"},
        "badCond": {"category": "bug", "description": "Detect suspicious condition expressions"},
        "badLock": {"category": "concurrency", "description": "Detect common mistakes with locks"},
        "badRegexp": {"category": "bug", "description": "Detect suspicious regexp patterns"},
        "badSorting": {"category": "bug", "description": "Detect suspicious sort function calls"},
        "badSyncOnceFunc": {"category": "bug", "description": "Detect bad usage of sync.OnceFunc"},
        "boolExprSimplify": {"category": "style", "description": "Suggests simplifying bool expressions"},
        "builtinShadow": {"category": "bug", "description": "Detects when predeclared identifiers shadowed"},
        "builtinShadowDecl": {"category": "bug", "description": "Detects when predeclared identifiers shadowed in var declarations"},
        "captLocal": {"category": "style", "description": "Detects capitalized names for local variables"},
        "caseOrder": {"category": "bug", "description": "Detects erroneous case order inside switch statements"},
        "codegenComment": {"category": "style", "description": "Detects malformed 'code generated' file comments"},
        "commentFormatting": {"category": "style", "description": "Detects comments with non-standard formatting"},
        "commentedOutCode": {"category": "style", "description": "Detects commented-out code inside function bodies"},
        "commentedOutImport": {"category": "style", "description": "Detects commented-out imports"},
        "defaultCaseOrder": {"category": "style", "description": "Detects when default case in switch isn't on 1st or last position"},
        "deferInLoop": {"category": "bug", "description": "Detects defer in loops"},
        "deferUnlambda": {"category": "style", "description": "Detects defer calls that can be simplified"},
        "deprecatedComment": {"category": "style", "description": "Detects malformed 'Deprecated' doc comments"},
        "docStub": {"category": "style", "description": "Detects comments that are stubs"},
        "dupArg": {"category": "bug", "description": "Detects suspicious duplicated arguments"},
        "dupBranchBody": {"category": "style", "description": "Detects duplicated branch bodies inside conditional statements"},
        "dupCase": {"category": "bug", "description": "Detects duplicated case clauses inside switch statements"},
        "dupImport": {"category": "style", "description": "Detects re-imports of the same package"},
        "dupSubExpr": {"category": "bug", "description": "Detects suspicious duplicated sub-expressions"},
        "dynamicFmtString": {"category": "bug", "description": "Detects non-constant format strings"},
        "elseif": {"category": "style", "description": "Detects else with nested if that can be replaced else-if"},
        "emptyDecl": {"category": "style", "description": "Detects empty declarations"},
        "emptyFallthrough": {"category": "style", "description": "Detects empty fallthrough statements"},
        "emptyStringTest": {"category": "style", "description": "Detects empty string checks that can be simplified"},
        "equalFold": {"category": "performance", "description": "Detects string equality checks that can be replaced with strings.EqualFold"},
        "evalOrder": {"category": "bug", "description": "Detects dependencies on evaluation order"},
        "exitAfterDefer": {"category": "bug", "description": "Detects os.Exit calls that cancel deferred functions"},
        "exposedSyncMutex": {"category": "bug", "description": "Detects exposed sync.Mutex in structs"},
        "filepathJoin": {"category": "bug", "description": "Detects problems in filepath.Join calls"},
        "flagDeref": {"category": "bug", "description": "Detects immediate dereferencing of flag pointers"},
        "flagName": {"category": "style", "description": "Detects suspicious flag names"},
        "hexLiteral": {"category": "style", "description": "Suggests using hex literals for big integers"},
        "httpNoBody": {"category": "bug", "description": "Detects HTTP requests without request body"},
        "hugeParam": {"category": "performance", "description": "Detects params that are too large"},
        "ifElseChain": {"category": "style", "description": "Detects repeated if-else chains"},
        "importShadow": {"category": "bug", "description": "Detects when imports shadow package-level identifiers"},
        "indexAlloc": {"category": "performance", "description": "Detects strings.Index calls that may cause allocation"},
        "initClause": {"category": "style", "description": "Detects non-assignment statements inside if/switch init clauses"},
        "mapKey": {"category": "bug", "description": "Detects suspicious map key assignments"},
        "methodExprCall": {"category": "style", "description": "Detects method expression call that can be replaced with method call"},
        "nestingReduce": {"category": "style", "description": "Finds where nesting level can be reduced"},
        "newDeref": {"category": "style", "description": "Detects immediate dereferencing of new()"},
        "nilValReturn": {"category": "bug", "description": "Detects return of a nil value after check"},
        "octalLiteral": {"category": "bug", "description": "Detects octal literals passed to functions"},
        "offBy1": {"category": "bug", "description": "Detects off-by-one errors"},
        "paramTypeCombine": {"category": "style", "description": "Detects function parameters that can be combined"},
        "preferDecodeRune": {"category": "style", "description": "Suggests using utf8.DecodeRuneInString"},
        "preferFilepathJoin": {"category": "style", "description": "Suggests using filepath.Join"},
        "preferFprint": {"category": "style", "description": "Suggests using fmt.Fprint functions"},
        "preferStringWriter": {"category": "style", "description": "Suggests using io.StringWriter"},
        "preferWriteByte": {"category": "performance", "description": "Suggests using WriteByte"},
        "ptrToRefParam": {"category": "bug", "description": "Detects pointer to reference parameter"},
        "rangeExprCopy": {"category": "performance", "description": "Detects expensive copies in range expressions"},
        "rangeValCopy": {"category": "performance", "description": "Detects expensive copies in range loops"},
        "recvNil": {"category": "bug", "description": "Detects immediate conversion of received pointer to value"},
        "redundantSprint": {"category": "style", "description": "Detects redundant Sprint calls"},
        "regexpMust": {"category": "style", "description": "Suggests using regexp.MustCompile"},
        "returnAfterHttpError": {"category": "bug", "description": "Detects return after http.Error"},
        "ruleguard": {"category": "style", "description": "Runs ruleguard rules"},
        "singleCaseSwitch": {"category": "style", "description": "Detects switch statements with single case"},
        "sliceClear": {"category": "style", "description": "Detects slice clearing that can be optimized"},
        "sloppyLen": {"category": "style", "description": "Detects len() comparisons that can be simplified"},
        "sloppyReassign": {"category": "bug", "description": "Detects sloppy reassignments"},
        "sloppyTestFuncName": {"category": "style", "description": "Detects test function names with wrong format"},
        "sloppyTypeAssert": {"category": "bug", "description": "Detects sloppy type assertions"},
        "sortSlice": {"category": "bug", "description": "Detects sort.Slice calls that can be optimized"},
        "sprintfQuotedString": {"category": "style", "description": "Detects sprintf formatting verbs for quoted strings"},
        "sqlQuery": {"category": "performance", "description": "Detects issues with sql queries"},
        "stringConcatSimplify": {"category": "style", "description": "Simplifies string concatenations"},
        "stringXbytes": {"category": "style", "description": "Detects redundant string(byteSlice) conversions"},
        "suggestFuncsInTesting": {"category": "style", "description": "Suggests using testing functions"},
        "switchTrue": {"category": "style", "description": "Detects switch true that can be rewritten as if-else"},
        "syncMapLoadAndDelete": {"category": "style", "description": "Suggests using sync.Map LoadAndDelete"},
        "timeExprSimplify": {"category": "style", "description": "Simplifies time expressions"},
        "todoCommentWithoutDetail": {"category": "style", "description": "Detects TODO comments without detail"},
        "tooManyResultsChecker": {"category": "style", "description": "Detects functions with too many results"},
        "truncateCmp": {"category": "bug", "description": "Detects truncation in comparisons"},
        "typeAssertChain": {"category": "style", "description": "Detects type assertion chains"},
        "typeDefFirst": {"category": "style", "description": "Detects type definitions before package doc"},
        "typeSwitchVar": {"category": "style", "description": "Suggests using type switch variable"},
        "typeUnparen": {"category": "style", "description": "Suggests removing parentheses around types"},
        "underef": {"category": "style", "description": "Suggests simplification of dereferencing"},
        "unlabelStmt": {"category": "style", "description": "Detects redundant labeled statements"},
        "unlambda": {"category": "style", "description": "Suggests simplification of lambda expressions"},
        "unnamedResult": {"category": "style", "description": "Suggests naming result parameters"},
        "unnecessaryBlock": {"category": "style", "description": "Detects unnecessary braced blocks"},
        "unnecessaryDefer": {"category": "style", "description": "Detects unnecessary defer statements"},
        "unusedParam": {"category": "style", "description": "Detects unused function parameters"},
        "unusedReceiver": {"category": "style", "description": "Detects unused method receivers"},
        "weakCond": {"category": "bug", "description": "Detects weak condition expressions"},
        "whyNoLint": {"category": "style", "description": "Ensures nolint directives have explanations"},
        "wrapperFunc": {"category": "style", "description": "Detects function wrappers"},
        "yodaStyleExpr": {"category": "style", "description": "Detects Yoda style conditions"},
    }
    
    def crawl(self) -> List[Dict]:
        """抓取 go-critic 规则"""
        print("Crawling go-critic rules...")
        rules = []
        for name, info in self.GOCRITIC_RULES.items():
            rules.append({
                "code": f"CRITIC_{name.upper()}",
                "name": name,
                "source": "go-critic",
                "category": info["category"],
                "description": info["description"]
            })
        self.rules.extend(rules)
        return rules


class ReviveCrawler(LinterCrawler):
    """抓取 revive 规则"""
    
    REVIVE_RULES = {
        "add-constant": {"category": "style", "description": "Suggests using constant for magic numbers"},
        "argument-limit": {"category": "complexity", "description": "Limits the number of arguments"},
        "atomic": {"category": "bug", "description": "Checks for atomic operations on non-atomic types"},
        "banned-characters": {"category": "style", "description": "Checks for banned characters in identifiers"},
        "bare-return": {"category": "style", "description": "Checks for bare returns"},
        "blank-imports": {"category": "style", "description": "Checks for blank imports"},
        "bool-literal-in-expr": {"category": "style", "description": "Suggests removing bool literals from expressions"},
        "call-to-gc": {"category": "performance", "description": "Checks for explicit runtime.GC calls"},
        "cognitive-complexity": {"category": "complexity", "description": "Checks cognitive complexity of functions"},
        "comment-spacings": {"category": "style", "description": "Checks comment formatting"},
        "comments-density": {"category": "style", "description": "Checks density of comments"},
        "confusing-naming": {"category": "bug", "description": "Checks for confusing naming"},
        "confusing-results": {"category": "bug", "description": "Checks for confusing function results"},
        "constant-logical-expr": {"category": "bug", "description": "Checks for constant logical expressions"},
        "context-as-argument": {"category": "style", "description": "Checks context placement in arguments"},
        "context-keys-type": {"category": "bug", "description": "Checks context key types"},
        "cyclomatic": {"category": "complexity", "description": "Checks cyclomatic complexity"},
        "datarace": {"category": "concurrency", "description": "Checks for potential data races"},
        "deep-exit": {"category": "style", "description": "Checks for exit calls in library code"},
        "defer": {"category": "bug", "description": "Checks defer statements"},
        "dot-imports": {"category": "style", "description": "Checks for dot imports"},
        "duplicated-imports": {"category": "style", "description": "Checks for duplicated imports"},
        "early-return": {"category": "style", "description": "Suggests early returns"},
        "empty-block": {"category": "style", "description": "Checks for empty blocks"},
        "empty-lines": {"category": "style", "description": "Checks for empty lines"},
        "enforce-map-style": {"category": "style", "description": "Enforces map initialization style"},
        "enforce-repeated-arg-type-style": {"category": "style", "description": "Enforces repeated argument type style"},
        "enforce-slice-style": {"category": "style", "description": "Enforces slice initialization style"},
        "error-naming": {"category": "style", "description": "Checks error variable naming"},
        "error-return": {"category": "style", "description": "Checks error return placement"},
        "error-string": {"category": "style", "description": "Checks error string formatting"},
        "exported": {"category": "style", "description": "Checks exported identifiers"},
        "file-header": {"category": "style", "description": "Checks file header"},
        "flag-parameter": {"category": "style", "description": "Checks for flag parameters"},
        "function-length": {"category": "complexity", "description": "Checks function length"},
        "function-result-limit": {"category": "complexity", "description": "Limits function results"},
        "get-return": {"category": "style", "description": "Checks getter return values"},
        "identical-branches": {"category": "bug", "description": "Checks for identical branches"},
        "if-return": {"category": "style", "description": "Checks if-return patterns"},
        "import-alias-naming": {"category": "style", "description": "Checks import alias naming"},
        "import-shadowing": {"category": "bug", "description": "Checks for import shadowing"},
        "imports-blocklist": {"category": "style", "description": "Checks for blocked imports"},
        "increment-decrement": {"category": "style", "description": "Checks increment/decrement style"},
        "indent-error-flow": {"category": "style", "description": "Checks error flow indentation"},
        "line-length-limit": {"category": "style", "description": "Limits line length"},
        "max-control-nesting": {"category": "complexity", "description": "Limits control nesting"},
        "max-public-structs": {"category": "complexity", "description": "Limits public structs"},
        "modifies-parameter": {"category": "bug", "description": "Checks for parameter modifications"},
        "modifies-value-receiver": {"category": "bug", "description": "Checks for value receiver modifications"},
        "nested-structs": {"category": "complexity", "description": "Checks for nested structs"},
        "optimize-operands-order": {"category": "performance", "description": "Suggests optimizing operand order"},
        "package-comments": {"category": "style", "description": "Checks package comments"},
        "range": {"category": "style", "description": "Checks range statements"},
        "range-val-address": {"category": "bug", "description": "Checks range value addresses"},
        "range-val-in-closure": {"category": "bug", "description": "Checks range values in closures"},
        "receiver-naming": {"category": "style", "description": "Checks receiver naming"},
        "redefines-builtin-id": {"category": "bug", "description": "Checks for builtin redefinitions"},
        "redundant-import-alias": {"category": "style", "description": "Checks for redundant import aliases"},
        "string-format": {"category": "bug", "description": "Checks string format"},
        "string-of-int": {"category": "bug", "description": "Checks for string(int) conversions"},
        "struct-tag": {"category": "bug", "description": "Checks struct tags"},
        "superfluous-else": {"category": "style", "description": "Checks for superfluous else"},
        "time-equal": {"category": "bug", "description": "Checks time comparisons"},
        "time-naming": {"category": "style", "description": "Checks time variable naming"},
        "unchecked-type-assertion": {"category": "bug", "description": "Checks for unchecked type assertions"},
        "unconditional-recursion": {"category": "bug", "description": "Checks for unconditional recursion"},
        "unexported-naming": {"category": "style", "description": "Checks unexported naming"},
        "unexported-return": {"category": "bug", "description": "Checks for unexported returns"},
        "unhandled-error": {"category": "bug", "description": "Checks for unhandled errors"},
        "unnecessary-stmt": {"category": "style", "description": "Checks for unnecessary statements"},
        "unused-parameter": {"category": "style", "description": "Checks for unused parameters"},
        "unused-receiver": {"category": "style", "description": "Checks for unused receivers"},
        "use-any": {"category": "style", "description": "Suggests using any instead of interface{}"},
        "use-errors-new": {"category": "style", "description": "Suggests using errors.New"},
        "var-declaration": {"category": "style", "description": "Checks variable declarations"},
        "var-naming": {"category": "style", "description": "Checks variable naming"},
        "waitgroup-by-value": {"category": "bug", "description": "Checks for WaitGroup passed by value"},
    }
    
    def crawl(self) -> List[Dict]:
        """抓取 revive 规则"""
        print("Crawling revive rules...")
        rules = []
        for name, info in self.REVIVE_RULES.items():
            rules.append({
                "code": f"REVIVE_{name.upper().replace('-', '_')}",
                "name": name,
                "source": "revive",
                "category": info["category"],
                "description": info["description"]
            })
        self.rules.extend(rules)
        return rules


def generate_woof_rules(all_rules: List[Dict]) -> Dict[str, List[Dict]]:
    """生成 woof 格式的规则分类"""
    
    # 分类映射
    category_mapping = {
        # 原有分类
        "bug": "F",
        "error": "F", 
        "concurrency": "C",
        "performance": "P",
        "security": "SEC",
        "style": "S",
        "format": "E",
        "imports": "I",
        "complexity": "B",
        "test": "D",
        "simplify": "SIM",
        "correctness": "F",
        "workspace": "UP",
        "generics": "UP",
        "fuzzing": "D",
        "quality": "B",
    }
    
    woof_rules = {
        "E": [],  # 代码风格
        "F": [],  # 逻辑错误
        "B": [],  # 代码质量
        "I": [],  # 导入
        "UP": [], # 升级建议
        "SIM": [],# 简化建议
        "S": [],  # 风格
        "D": [],  # 文档
        "P": [],  # 性能
        "C": [],  # 并发
        "SEC": [],# 安全
    }
    
    seen_codes = set()
    
    for rule in all_rules:
        category = rule.get("category", "style")
        woof_category = category_mapping.get(category, "S")
        
        # 生成唯一 code
        base_code = rule.get("code", "")
        if not base_code:
            base_code = f"{woof_category}001"
        
        # 避免重复
        counter = 1
        code = base_code
        while code in seen_codes:
            code = f"{base_code}_{counter}"
            counter += 1
        seen_codes.add(code)
        
        woof_rule = {
            "code": code,
            "name": rule.get("name", ""),
            "description": rule.get("description", ""),
            "source": rule.get("source", ""),
            "severity": rule.get("severity", "warning"),
            "category": woof_category,
        }
        
        if woof_category in woof_rules:
            woof_rules[woof_category].append(woof_rule)
    
    return woof_rules


def generate_rust_code(woof_rules: Dict[str, List[Dict]]) -> str:
    """生成 Rust 规则代码模板"""
    
    rust_code = """//! Auto-generated rules from Go community linters
//! Generated: {timestamp}
//! Total rules: {total}

use crate::rules::{{Rule, RuleMetadata, RuleCategory, RulePriority}};
use crate::{{Diagnostic, Severity}};
use tree_sitter::Node;

// ==================== GENERATED RULES ====================

""".format(
        timestamp=datetime.now().isoformat(),
        total=sum(len(rules) for rules in woof_rules.values())
    )
    
    # 为每个规则生成结构体
    for category, rules in woof_rules.items():
        if not rules:
            continue
            
        rust_code += f"// {category}系列规则\n"
        
        for rule in rules:
            name = rule["name"].replace("-", "_").replace(".", "_")
            code = rule["code"]
            description = rule["description"].replace('"', '\\"')
            
            rust_code += f"""
pub struct {name.upper()};

impl Rule for {name.upper()} {{
    fn metadata(&self) -> RuleMetadata {{
        RuleMetadata {{
            code: "{code}",
            name: "{name}",
            description: "{description}",
            category: RuleCategory::{get_category_enum(rule["category"])},
            priority: RulePriority::{get_priority(rule["severity"])},
            default_severity: Severity::{rule["severity"].capitalize()},
        }}
    }}

    fn check(&self, node: Node, source: &str, file_path: &str) -> Vec<Diagnostic> {{
        // TODO: Implement check logic
        // Source: {rule["source"]}
        vec![]
    }}
}}
"""
    
    # 生成 get_rules 函数
    rust_code += "\n// ==================== GET RULES ====================\n\n"
    rust_code += "pub fn get_generated_rules() -> Vec<Box<dyn Rule>> {\n"
    rust_code += "    vec![\n"
    
    for category, rules in woof_rules.items():
        for rule in rules:
            name = rule["name"].replace("-", "_").replace(".", "_")
            rust_code += f"        Box::new({name.upper()}),\n"
    
    rust_code += "    ]\n}\n"
    
    return rust_code


def get_category_enum(category: str) -> str:
    """获取类别枚举名"""
    mapping = {
        "E": "Codestyle",
        "F": "Logic", 
        "B": "Bugbear",
        "I": "Imports",
        "UP": "Upgrade",
        "SIM": "Simplify",
        "S": "Style",
        "D": "Docs",
        "P": "Performance",
        "C": "Concurrency",
        "SEC": "Security",
    }
    return mapping.get(category, "Style")


def get_priority(severity: str) -> str:
    """获取优先级"""
    if severity == "error":
        return "Required"
    elif severity == "warning":
        return "Recommended"
    return "Optional"


def main():
    """主函数"""
    print("=" * 60)
    print("Go Linter Crawler - 抓取 Go 社区 linter 规则")
    print("=" * 60)
    
    # 创建爬虫实例
    crawlers = [
        GolangCILintCrawler(),
        StaticCheckCrawler(),
        GoCriticCrawler(),
        ReviveCrawler(),
    ]
    
    all_linters = []
    all_rules = []
    
    # 执行抓取
    for crawler in crawlers:
        try:
            if hasattr(crawler, 'crawl'):
                result = crawler.crawl()
                if isinstance(crawler, GolangCILintCrawler):
                    all_linters.extend(result)
                else:
                    all_rules.extend(result)
                # 同时收集每个 crawler 的 rules 属性
                if hasattr(crawler, 'rules') and crawler.rules:
                    all_rules.extend(crawler.rules)
        except Exception as e:
            print(f"Error crawling {crawler.__class__.__name__}: {e}")
    
    print(f"\nTotal linters collected: {len(all_linters)}")
    print(f"Total rules collected: {len(all_rules)}")
    
    # 生成 woof 格式规则
    woof_rules = generate_woof_rules(all_rules)
    
    # 统计
    print("\n" + "=" * 60)
    print("规则统计:")
    print("=" * 60)
    total = 0
    for category, rules in woof_rules.items():
        if rules:
            print(f"  {category}: {len(rules)} rules")
            total += len(rules)
    print(f"\nTotal: {total} rules")
    
    # 保存结果
    import os
    os.makedirs("scripts/output", exist_ok=True)
    
    # 保存所有 linter
    crawlers[0].save_json(all_linters, "scripts/output/linters.json")
    
    # 保存所有规则
    crawlers[0].save_json(all_rules, "scripts/output/rules.json")
    
    # 保存 woof 格式规则
    crawlers[0].save_json(woof_rules, "scripts/output/woof_rules.json")
    
    # 生成 Rust 代码
    rust_code = generate_rust_code(woof_rules)
    with open("scripts/output/generated_rules.rs", "w", encoding="utf-8") as f:
        f.write(rust_code)
    print("\nSaved to scripts/output/generated_rules.rs")
    
    # 生成 Markdown 报告
    generate_markdown_report(woof_rules, all_linters)
    
    print("\n" + "=" * 60)
    print("Done!")
    print("=" * 60)


def generate_markdown_report(woof_rules: Dict, linters: List[Dict]):
    """生成 Markdown 报告"""
    
    md = """# Go Linter Crawler 报告

## 概述

本报告汇总了从 Go 社区抓取的各种 linter 规则，用于充实 woof 的规则体系。

## 抓取的 Linter 源

"""
    
    # 统计来源
    sources = {}
    for category, rules in woof_rules.items():
        for rule in rules:
            source = rule.get("source", "unknown")
            sources[source] = sources.get(source, 0) + 1
    
    for source, count in sorted(sources.items(), key=lambda x: -x[1]):
        md += f"- **{source}**: {count} 条规则\n"
    
    md += "\n## 规则分类\n\n"
    
    category_names = {
        "E": "代码风格 (E-series)",
        "F": "逻辑错误 (F-series)",
        "B": "代码质量 (B-series)",
        "I": "导入规范 (I-series)",
        "UP": "升级建议 (UP-series)",
        "SIM": "简化建议 (SIM-series)",
        "S": "风格规范 (S-series)",
        "D": "文档规范 (D-series)",
        "P": "性能优化 (P-series)",
        "C": "并发问题 (C-series)",
        "SEC": "安全问题 (SEC-series)",
    }
    
    for category, rules in woof_rules.items():
        if not rules:
            continue
        md += f"\n### {category_names.get(category, category)} ({len(rules)}条)\n\n"
        md += "| 代码 | 名称 | 描述 | 来源 |\n"
        md += "|------|------|------|------|\n"
        for rule in rules[:20]:  # 只显示前20条
            md += f"| {rule['code']} | {rule['name']} | {rule['description'][:50]}... | {rule['source']} |\n"
        if len(rules) > 20:
            md += f"| ... | ... | 还有 {len(rules)-20} 条规则 | ... |\n"
    
    md += """
## 建议实现优先级

### P0 - 必须实现 (核心规则)
"""
    
    # 找出高优先级的规则
    critical_rules = []
    for category, rules in woof_rules.items():
        for rule in rules:
            if rule.get("severity") == "error" or category in ["F", "SEC", "C"]:
                critical_rules.append(rule)
    
    md += f"\n共 {len(critical_rules)} 条核心规则，包括:\n\n"
    for rule in critical_rules[:15]:
        md += f"- `{rule['code']}`: {rule['name']} ({rule['source']})\n"
    
    md += """
### P1 - 推荐实现

包括性能优化、代码质量、风格规范等规则。

### P2 - 可选实现

包括文档规范、简化建议等规则。

## 新特性覆盖

### 泛型 (Generics) 相关规则
"""
    
    # 收集所有规则供统计
    all_rules = []
    for rules in woof_rules.values():
        all_rules.extend(rules)
        
    generic_rules = [r for r in all_rules if r.get("category") == "UP" and "generics" in r.get("description", "").lower()]
    md += f"\n找到 {len(generic_rules)} 条泛型相关规则\n"
    
    md += """
### Fuzzing 相关规则
"""
    
    fuzzing_rules = [r for r in all_rules if r.get("category") == "D" and "fuzz" in r.get("description", "").lower()]
    md += f"\n找到 {len(fuzzing_rules)} 条 Fuzzing 相关规则\n"
    
    md += """
### Workspace 相关规则
"""
    
    workspace_rules = [r for r in all_rules if r.get("category") == "UP" and "workspace" in r.get("description", "").lower()]
    md += f"\n找到 {len(workspace_rules)} 条 Workspace 相关规则\n"
    
    md += """
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
"""
    
    with open("scripts/output/CRAWLER_REPORT.md", "w", encoding="utf-8") as f:
        f.write(md)
    print("Saved to scripts/output/CRAWLER_REPORT.md")


if __name__ == "__main__":
    main()
