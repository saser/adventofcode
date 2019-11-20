package day20

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	if ok := sc.Scan(); !ok {
		return "", fmt.Errorf("year 2015, day 20, part 1: %w", sc.Err())
	}
	line := sc.Text()
	target, err := strconv.Atoi(line)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 20, part 1: %w", err)
	}
	answer, ok := firstHouse(target)
	if !ok {
		return "", errors.New("year 2015, day 20, part 1: no solution found")
	}
	return fmt.Sprint(answer), nil
}

func firstHouse(target int) (int, bool) {
	houses := make([]int, target+1)
	for i := 1; i <= target/10; i++ {
		for j := i; j <= target/10; j += i {
			houses[j] += i * 10
		}
	}
	for i, house := range houses {
		if house >= target {
			return i, true
		}
	}
	return 0, false
}
