package day14

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type mask struct {
	clear, set int64
}

func parseMask(line string) mask {
	n := len(line)
	var clear, set int64
	for bit := 0; bit < 36; bit++ {
		switch line[n-bit-1] {
		case '0':
			clear |= 1 << bit
		case '1':
			set |= 1 << bit
		}
	}
	return mask{
		clear: clear,
		set:   set,
	}
}

func parseMem(line string) (int64, int64) {
	parts := strings.Split(line, " = ")
	mem, err := strconv.ParseInt(strings.Trim(parts[0], "mem[]"), 10, 64)
	if err != nil {
		panic(err)
	}
	val, err := strconv.ParseInt(parts[1], 10, 64)
	if err != nil {
		panic(err)
	}
	return mem, val
}

func (m mask) ApplyTo(n int64) int64 {
	return (n &^ m.clear) | m.set
}

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	var m mask
	mem := make(map[int64]int64)
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		switch {
		case strings.HasPrefix(line, "mask"):
			m = parseMask(line)
		case strings.HasPrefix(line, "mem"):
			addr, val := parseMem(line)
			mem[addr] = m.ApplyTo(val)
		}
	}
	var sum int64 = 0
	for _, val := range mem {
		sum += val
	}
	return fmt.Sprint(sum), nil
}
