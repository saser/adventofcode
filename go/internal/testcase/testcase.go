package testcase

import (
	"io/ioutil"
	"testing"
)

var (
	doNotOptimizeAnswer string
	doNotOptimizeError  error
)

type Solution func(string) (string, error)

type TestCase2 struct {
	name  string
	input string
	want  string
}

func (tc TestCase2) Test(t *testing.T, sln Solution) {
	t.Helper()
	t.Run(tc.name, func(t *testing.T) {
		got, err := sln(tc.input)
		if err != nil {
			t.Fatalf("error: %v", err)
		}
		if got != tc.want {
			t.Errorf("answer = %q; want %q", got, tc.want)
		}
	})
}

func (tc TestCase2) Benchmark(b *testing.B, sln Solution) {
	b.Helper()
	b.Run(tc.name, func(b *testing.B) {
		var (
			got string
			err error
		)
		for i := 0; i < b.N; i++ {
			got, err = sln(tc.input)
			if err != nil {
				b.Fatalf("error: %v", err)
			}
			if got != tc.want {
				b.Fatalf("answer = %q; want %q", got, tc.want)
			}
		}
		doNotOptimizeAnswer = got
		doNotOptimizeError = err
	})
}

func New(name, input, want string) TestCase2 {
	return TestCase2{
		name:  name,
		input: input,
		want:  want,
	}
}

func NewFile(name, filename, want string) TestCase2 {
	data, err := ioutil.ReadFile(filename)
	if err != nil {
		panic(err)
	}
	input := string(data)
	return TestCase2{
		name:  name,
		input: input,
		want:  want,
	}
}
