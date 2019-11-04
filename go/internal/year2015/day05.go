package year2015

import (
	"fmt"
	"io"
	"io/ioutil"
	"strings"
)

func Day05One(r io.Reader) (string, error) {
	lines, err := parseDay05(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 05, part 1: %w", err)
	}
	conditions := []day05Condition{
		threeVowels,
		letterTwice,
		noBadPairs,
	}
	return solveDay05(lines, conditions), nil
}

func Day05Two(r io.Reader) (string, error) {
	lines, err := parseDay05(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 05, part 2: %w", err)
	}
	conditions := []day05Condition{
		twicePair,
		letterTwiceSpaced,
	}
	return solveDay05(lines, conditions), nil
}

func parseDay05(r io.Reader) ([]string, error) {
	bytes, err := ioutil.ReadAll(r)
	if err != nil {
		return nil, fmt.Errorf("parse: %w", err)
	}
	lines := strings.Split(string(bytes), "\n")
	return lines, nil
}

func solveDay05(lines []string, conditions []day05Condition) string {
	count := 0
outer:
	for _, line := range lines {
		for _, cond := range conditions {
			if !cond(line) {
				continue outer
			}
		}
		count++
	}
	return fmt.Sprint(count)
}

type day05Condition func(string) bool

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
