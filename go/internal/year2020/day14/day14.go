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
	clear, set, floating int64
}

func parseMask(line string) mask {
	n := len(line)
	var m mask
	for bit := 0; bit < 36; bit++ {
		var i *int64
		switch line[n-bit-1] {
		case '0':
			i = &m.clear
		case '1':
			i = &m.set
		case 'X':
			i = &m.floating
		}
		*i |= 1 << bit
	}
	return m
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

func (m mask) ApplyToAddr(addr int64, f func(maskedAddr int64)) {
	var rec func(bit int, acc mask)
	rec = func(bit int, acc mask) {
		if bit < 0 {
			f(acc.ApplyTo(addr))
			return
		}
		// bit is not a floating bit.
		if m.floating&(1<<bit) == 0 {
			rec(bit-1, acc)
			return
		}
		accCleared := acc
		accCleared.clear |= 1 << bit
		rec(bit-1, accCleared)
		accSet := acc
		accSet.set |= 1 << bit
		rec(bit-1, accSet)
	}
	rec(35, mask{set: m.set})
}

func solve(input string, part int) (string, error) {
	var m mask
	mem := make(map[int64]int64)
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
		switch {
		case strings.HasPrefix(line, "mask"):
			m = parseMask(line)
		case strings.HasPrefix(line, "mem"):
			addr, val := parseMem(line)
			switch part {
			case 1:
				mem[addr] = m.ApplyTo(val)
			case 2:
				m.ApplyToAddr(addr, func(maskedAddr int64) {
					mem[maskedAddr] = val
				})
			}
		}
	}
	var sum int64 = 0
	for _, val := range mem {
		sum += val
	}
	return fmt.Sprint(sum), nil
}
