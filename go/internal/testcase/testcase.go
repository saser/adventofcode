package testcase

import (
	"fmt"
	"io"
	"os"
	"strings"
	"testing"

	"github.com/Saser/adventofcode/internal/solution"
	"github.com/stretchr/testify/require"
)

var resultString string
var resultErr error

type TestCase interface {
	Name() string
	Input() io.ReadSeeker
	Output() string
}

func Run(t *testing.T, tc TestCase, sol solution.Solution) {
	t.Run(tc.Name(), func(t *testing.T) {
		answer, err := sol(tc.Input())
		require.NoError(t, err)
		require.Equal(t, tc.Output(), answer)
	})
}

func Bench(b *testing.B, tc TestCase, sol solution.Solution) {
	rs := tc.Input()
	var r string
	var e error
	b.Run(tc.Name(), func(b *testing.B) {
		for i := 0; i < b.N; i++ {
			r, e = sol(rs)
			_, err := rs.Seek(0, io.SeekStart)
			if err != nil {
				b.Fatal(err)
			}
		}
	})
	resultString = r
	resultErr = e
}

type stringCase struct {
	name   string
	input  string
	output string
}

func FromString(name, input, output string) TestCase {
	return &stringCase{
		name:   name,
		input:  input,
		output: output,
	}
}

func (s *stringCase) Name() string {
	return s.name
}

func (s *stringCase) Input() io.ReadSeeker {
	return strings.NewReader(s.input)
}

func (s *stringCase) Output() string {
	return s.output
}

type fileCase struct {
	name   string
	file   *os.File
	output string
}

func FromFile(t testing.TB, path string, output string) TestCase {
	file, err := os.Open(path)
	require.NoError(t, err)
	return &fileCase{
		name:   fmt.Sprintf("file=%s", path),
		file:   file,
		output: output,
	}
}

func (f *fileCase) Name() string {
	return f.name
}

func (f *fileCase) Input() io.ReadSeeker {
	return f.file
}

func (f *fileCase) Output() string {
	return f.output
}
