package day20

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"math"
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
	target /= 10
	for i := 1; i <= target; i++ {
		s := sumOfDivisors(i)
		if s >= target {
			return fmt.Sprint(i), nil
		}
	}
	return "", errors.New("not implemented yet")
}

func sumOfDivisors(n int) int {
	if n == 1 {
		return 1
	}
	sum := 1 + n
	limit := int(math.Sqrt(float64(n)))
	for i := 2; i < limit; i++ {
		if n%i == 0 {
			sum += i
			sum += n / i
		}
	}
	if n%limit == 0 {
		sum += limit
	}
	return sum
}
