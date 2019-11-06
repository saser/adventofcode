package day08

import (
	"bufio"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	s := 0
	var diff func(string) int
	switch part {
	case 1:
		diff = diffMemory
	case 2:
		diff = diffEncoded
	}
	for sc.Scan() {
		s += diff(sc.Text())
	}
	return fmt.Sprint(s), nil
}

func diffMemory(s string) int {
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

func diffEncoded(s string) int {
	encoded := 2 // for the starting and ending quotation marks
	for _, r := range s {
		length := 1
		switch r {
		case '\\', '"':
			length = 2
		}
		encoded += length
	}
	return encoded - len(s)
}
