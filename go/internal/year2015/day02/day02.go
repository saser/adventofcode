package day02

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	boxes, err := parse(input)
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

func Part2(input string) (string, error) {
	boxes, err := parse(input)
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

type box struct {
	l, w, h int
}

func parse(input string) ([]box, error) {
	lines := strings.Split(input, "\n")
	boxes := make([]box, len(lines))
	for i, line := range lines {
		if line == "" {
			continue
		}
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
		boxes[i] = box{l: l, w: w, h: h}
	}
	return boxes, nil
}
