package year2015

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func Day02One(r io.Reader) (string, error) {
	boxes, err := parseDay02(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 02, part 1: %w", err)
	}
	requiredPaper := 0
	for _, box := range boxes {
		smallestSide := -1
		for _, side := range []int{
			box.l * box.w,
			box.w * box.h,
			box.h * box.l,
		} {
			if smallestSide == -1 || side < smallestSide {
				smallestSide = side
			}
			requiredPaper += side * 2
		}
		requiredPaper += smallestSide
	}
	return fmt.Sprint(requiredPaper), nil
}

func Day02Two(r io.Reader) (string, error) {
	return "", errors.New("not yet implemented")
}

type day02Box struct {
	l, w, h int
}

func parseDay02(r io.Reader) ([]day02Box, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	boxes := make([]day02Box, 0)
	for sc.Scan() {
		line := sc.Text()
		parts := strings.Split(line, "x")
		l, err := strconv.Atoi(parts[0])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		w, err := strconv.Atoi(parts[1])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		h, err := strconv.Atoi(parts[2])
		if err != nil {
			return nil, fmt.Errorf("parse: %w", err)
		}
		boxes = append(boxes, day02Box{l: l, w: w, h: h})
	}
	return boxes, nil
}
