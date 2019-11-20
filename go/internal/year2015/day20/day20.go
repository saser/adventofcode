package day20

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
)

func Part1(r io.Reader) (string, error) {
	br := bufio.NewReader(r)
	line, err := br.ReadString('\n')
	if err != nil && err != io.EOF {
		return "", fmt.Errorf("year 2015, day 20, part 1: %w", err)
	}
	target, err := strconv.Atoi(line)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 20, part 1: %w", err)
	}
	fmt.Println(target)
	return "", errors.New("not implemented yet")
}
