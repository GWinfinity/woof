#!/usr/bin/env python3
"""Generate 30 test scenarios for benchmarking woof vs golangci-lint"""

import os

scenarios = [
    # 01-05: Basic unused/dead code issues
    ("01_unused_import.go", '''package scenarios

import (
	"fmt"
	"os"
	"strings"
)

func UnusedImportScenario() {
	fmt.Println("only fmt is used")
}
'''),

    ("02_unused_variable.go", '''package scenarios

func UnusedVariableScenario() {
	used := "hello"
	unused := "world"
	_ = used
}
'''),

    ("03_shadow_variable.go", '''package scenarios

import "fmt"

func ShadowVariableScenario() {
	x := 1
	if true {
		x := 2
		fmt.Println(x)
	}
	fmt.Println(x)
}
'''),

    ("04_unreachable_code.go", '''package scenarios

func UnreachableCodeScenario() bool {
	return true
	x := 1
	_ = x
}
'''),

    ("05_empty_body.go", '''package scenarios

func EmptyBodyScenario(x int) {
	if x > 0 {
	}
	for i := 0; i < x; i++ {
	}
}
'''),

    # 06-10: Error handling issues
    ("06_unchecked_error.go", '''package scenarios

import (
	"os"
	"io/ioutil"
)

func UncheckedErrorScenario() {
	os.Open("test.txt")
	ioutil.ReadFile("test.txt")
}
'''),

    ("07_ignored_error.go", '''package scenarios

import "os"

func IgnoredErrorScenario() {
	f, _ := os.Open("test.txt")
	_ = f
}
'''),

    ("08_error_not_last_return.go", '''package scenarios

import "errors"

func ErrorNotLastReturnScenario() (error, int) {
	return errors.New("test"), 42
}
'''),

    ("09_nil_error_return.go", '''package scenarios

func NilErrorReturnScenario() error {
	var err error
	return err
}
'''),

    ("10_panic_error.go", '''package scenarios

func PanicErrorScenario(x int) int {
	if x < 0 {
		panic("negative")
	}
	return x
}
'''),

    # 11-15: Style and formatting
    ("11_line_too_long.go", '''package scenarios

// ThisIsAVeryLongCommentThatExceedsTheMaximumLineLengthAllowedByMostLintersAndShouldDefinitelyTriggerAnError is a long comment
func LineTooLongScenario() {}
'''),

    ("12_mixed_tabs_spaces.go", '''package scenarios

func MixedTabsSpacesScenario() {
   var x int
	y := x
	_ = y
}
'''),

    ("13_trailing_whitespace.go", '''package scenarios

func TrailingWhitespaceScenario() {   
	x := 1
	_ = x  
}
'''),

    ("14_snake_case.go", '''package scenarios

func snake_case_function() {}

var snake_case_variable = 1
'''),

    ("15_stuttering.go", '''package scenarios

type ScenariosConfig struct {
	Name string
}

func (s *ScenariosConfig) ScenariosMethod() {}
'''),

    # 16-20: Complexity issues
    ("16_high_cyclomatic.go", '''package scenarios

func HighCyclomaticScenario(x, y, z int) int {
	if x > 0 {
		if y > 0 {
			if z > 0 {
				return 1
			} else if z < 0 {
				return 2
			} else {
				return 3
			}
		} else if y < 0 {
			if z > 0 {
				return 4
			} else if z < 0 {
				return 5
			} else {
				return 6
			}
		} else {
			return 7
		}
	} else if x < 0 {
		return 8
	} else {
		return 9
	}
}
'''),

    ("17_deep_nesting.go", '''package scenarios

func DeepNestingScenario(x int) {
	if x > 0 {
		if x > 1 {
			if x > 2 {
				if x > 3 {
					if x > 4 {
						if x > 5 {
							_ = x
						}
					}
				}
			}
		}
	}
}
'''),

    ("18_long_function.go", '''package scenarios

func LongFunctionScenario() {
	x1 := 1
	x2 := x1 + 1
	x3 := x2 + 1
	x4 := x3 + 1
	x5 := x4 + 1
	x6 := x5 + 1
	x7 := x6 + 1
	x8 := x7 + 1
	x9 := x8 + 1
	x10 := x9 + 1
	x11 := x10 + 1
	x12 := x11 + 1
	x13 := x12 + 1
	x14 := x13 + 1
	x15 := x14 + 1
	x16 := x15 + 1
	x17 := x16 + 1
	x18 := x17 + 1
	x19 := x18 + 1
	x20 := x19 + 1
	x21 := x20 + 1
	x22 := x21 + 1
	x23 := x22 + 1
	x24 := x23 + 1
	x25 := x24 + 1
	_ = x25
}
'''),

    ("19_many_parameters.go", '''package scenarios

func ManyParametersScenario(a, b, c, d, e, f, g, h, i, j, k, l int) int {
	return a + b + c + d + e + f + g + h + i + j + k + l
}
'''),

    ("20_long_parameter_list.go", '''package scenarios

func LongParameterListScenario(param1 string, param2 int, param3 bool, param4 float64, param5 []byte, param6 map[string]interface{}, param7 chan int, param8 func(), param9 interface{}, param10 error) {}
'''),

    # 21-25: Concurrency issues
    ("21_race_condition.go", '''package scenarios

func RaceConditionScenario() int {
	var counter int
	go func() {
		counter++
	}()
	go func() {
		counter++
	}()
	return counter
}
'''),

    ("22_channel_close.go", '''package scenarios

func ChannelCloseScenario() chan int {
	ch := make(chan int)
	close(ch)
	return ch
}
'''),

    ("23_goroutine_leak.go", '''package scenarios

import "time"

func GoroutineLeakScenario() {
	go func() {
		time.Sleep(1 * time.Second)
	}()
}
'''),

    ("24_mutex_copy.go", '''package scenarios

import "sync"

type MutexHolder struct {
	mu sync.Mutex
	x  int
}

func MutexCopyScenario() MutexHolder {
	m := MutexHolder{x: 1}
	return m
}
'''),

    ("25_defer_in_loop.go", '''package scenarios

import "os"

func DeferInLoopScenario(files []string) {
	for _, f := range files {
		file, _ := os.Open(f)
		defer file.Close()
	}
}
'''),

    # 26-30: Other common issues
    ("26_redundant_type.go", '''package scenarios

var redundantTypeScenario map[string]int = map[string]int{}
'''),

    ("27_simplify_slice.go", '''package scenarios

func SimplifySliceScenario(s []int) []int {
	return s[:]
}
'''),

    ("28_naked_return.go", '''package scenarios

func NakedReturnScenario() (result int) {
	result = 42
	return
}
'''),

    ("29_deprecated_ioutil.go", '''package scenarios

import "io/ioutil"

func DeprecatedIOUtilScenario() ([]byte, error) {
	return ioutil.ReadFile("test.txt")
}
'''),

    ("30_exported_missing_doc.go", '''package scenarios

type ExportedType struct {
	Field int
}

func ExportedFunction() {}

var ExportedVariable = 42

const ExportedConstant = 100
'''),
]

def main():
    output_dir = os.path.dirname(os.path.abspath(__file__))
    scenarios_dir = os.path.join(output_dir, "scenarios")
    
    os.makedirs(scenarios_dir, exist_ok=True)
    
    for filename, content in scenarios:
        filepath = os.path.join(scenarios_dir, filename)
        with open(filepath, 'w') as f:
            f.write(content)
        print(f"Created: {filename}")
    
    print(f"\nTotal: {len(scenarios)} scenarios created in {scenarios_dir}")

if __name__ == "__main__":
    main()
