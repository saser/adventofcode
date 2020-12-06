package day05

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1), nil
}

func Part2(input string) (string, error) {
	return solve(input, 2), nil
}

func solve(input string, part int) string {
	var conditions []condition
	switch part {
	case 1:
		conditions = []condition{
			threeVowels,
			letterTwice,
			noBadPairs,
		}
	case 2:
		conditions = []condition{
			twicePair,
			letterTwiceSpaced,
		}
	}
	count := 0
outer:
	for _, line := range strings.Split(input, "\n") {
		for _, cond := range conditions {
			if !cond(line) {
				continue outer
			}
		}
		count++
	}
	return fmt.Sprint(count)
}

type condition func(string) bool

func threeVowels(s string) (b bool) {
	count := 0
	for _, r := range s {
		switch r {
		case 'a', 'e', 'i', 'o', 'u':
			count++
			if count >= 3 {
				return true
			}
		}
	}
	return false
}

func letterTwice(s string) bool {
	for i := 0; i < len(s)-1; i++ {
		if s[i] == s[i+1] {
			return true
		}
	}
	return false
}

func noBadPairs(s string) bool {
	for _, pair := range []string{"ab", "cd", "pq", "xy"} {
		if strings.Contains(s, pair) {
			return false
		}
	}
	return true
}

func twicePair(s string) bool {
	for i := 0; i < len(s)-2; i++ {
		if strings.Contains(s[i+2:], s[i:i+2]) {
			return true
		}
	}
	return false
}

func letterTwiceSpaced(s string) bool {
	for i := 0; i < len(s)-2; i++ {
		if s[i] == s[i+2] {
			return true
		}
	}
	return false
}
