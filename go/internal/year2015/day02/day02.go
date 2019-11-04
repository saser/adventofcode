package day02

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func Part1(r io.Reader) (string, error) {
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

func Part2(r io.Reader) (string, error) {
	boxes, err := parseDay02(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 02, part 2: %w", err)
	}
	requiredRibbon := 0
	for _, box := range boxes {
		smallestPerimeter := -1
		for _, perimeter := range []int{
			2 * (box.l + box.w),
			2 * (box.w + box.h),
			2 * (box.h + box.l),
		} {
			if smallestPerimeter == -1 || perimeter < smallestPerimeter {
				smallestPerimeter = perimeter
			}
		}
		requiredRibbon += smallestPerimeter
		requiredRibbon += box.l * box.w * box.h
	}
	return fmt.Sprint(requiredRibbon), nil
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
