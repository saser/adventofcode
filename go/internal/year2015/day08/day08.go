package day08

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	s := 0
	var diff func(string) int
	switch part {
	case 1:
		diff = diffMemory
	case 2:
		diff = diffEncoded
	}
	for _, line := range strings.Split(input, "\n") {
		if line == "" {
			continue
		}
		s += diff(line)
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
