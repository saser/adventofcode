package striter

import (
	"io/ioutil"
	"os"
	"path/filepath"
	"reflect"
	"sort"
	"strings"
	"testing"
)

var (
	doNotOptimizeInt    int
	doNotOptimizeString string

	inputs = map[string]string{} // path -> contents
	paths  []string              // keys into inputs, sorted
)

func init() {
	walkFn := func(path string, info os.FileInfo, err error) error {
		if err != nil {
			return err
		}
		if info.IsDir() {
			return nil
		}
		data, err := ioutil.ReadFile(path)
		if err != nil {
			return err
		}
		paths = append(paths, path)
		inputs[path] = string(data)
		return nil
	}
	if err := filepath.Walk("../../../inputs", walkFn); err != nil {
		panic(err)
	}
	sort.Strings(paths)
}

// TestSplit tests the correctness of Split by walking through all puzzle
// inputs, splitting using some common separators, and comparing the results to
// that of the standard library strings.Split function.
func TestSplit(t *testing.T) {
	for _, path := range paths {
		input := inputs[path]
		for _, sep := range []string{
			" ",
			"\n",
			"\n\n",
		} {
			var (
				wantInts    []int
				wantStrings []string
			)
			for i, s := range strings.Split(input, sep) {
				wantInts = append(wantInts, i)
				wantStrings = append(wantStrings, s)
			}
			var (
				gotInts    []int
				gotStrings []string
			)
			Split(input, sep, func(i int, s string) bool {
				gotInts = append(gotInts, i)
				gotStrings = append(gotStrings, s)
				return true
			})
			if !reflect.DeepEqual(gotInts, wantInts) {
				t.Errorf("[path=%q sep=%q] len(gotInts) = %v; len(wantInts) = %v", path, sep, len(gotInts), len(wantInts))
				if testing.Verbose() {
					t.Logf("[path=%q sep=%q] gotInts = %v; wantInts = %v", path, sep, gotInts, wantInts)
				}
			}
			if !reflect.DeepEqual(gotStrings, wantStrings) {
				t.Errorf("[path=%q sep=%q] len(gotStrings) = %v; len(wantStrings) = %v", path, sep, len(gotStrings), len(wantStrings))
				if testing.Verbose() {
					t.Logf("[path=%q sep=%q] gotStrings = %v; wantStrings = %v", path, sep, gotStrings, wantStrings)
				}
			}
		}
	}
}

func BenchmarkSplit(b *testing.B) {
	for _, path := range paths {
		input := inputs[path]
		b.Run(path, func(b *testing.B) {
			for name, sep := range map[string]string{
				"spc":  " ",
				"nl":   "\n",
				"nlnl": "\n\n",
			} {
				b.Run(name, func(b *testing.B) {
					for n := 0; n < b.N; n++ {
						Split(input, sep, func(i int, s string) bool {
							doNotOptimizeInt = i
							doNotOptimizeString = s
							return true
						})
					}
				})
			}
		})
	}
}
