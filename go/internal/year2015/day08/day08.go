package day08

import (
	"bufio"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	s := 0
	for sc.Scan() {
		s += diff(sc.Text())
	}
	return fmt.Sprint(s), nil
}

func diff(s string) int {
	memory := 0
	skip := 0
	for i := 1; i < len(s)-1; i++ {
		if skip > 0 {
			skip--
			continue
		}
		memory++
		switch s[i] {
		case '\\':
			switch s[i+1] {
			case '\\', '"':
				skip = 1
			case 'x':
				skip = 3
			}
		}
	}
	return len(s) - memory
}
