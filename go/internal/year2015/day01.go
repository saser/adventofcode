package year2015

import (
	"bufio"
	"fmt"
	"io"
)

func One(r io.Reader) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanRunes)
	floor := 0
	for sc.Scan() {
		tok := sc.Text()
		switch tok {
		case "(":
			floor++
		case ")":
			floor--
		default:
			return "", fmt.Errorf("year 2015, day 01, part 1: invalid token: %s", tok)
		}
	}
	return fmt.Sprint(floor), nil
}
