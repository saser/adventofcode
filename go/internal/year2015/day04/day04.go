package day04

import (
	"crypto/md5"
	"encoding/binary"
	"fmt"
)

func Part1(input string) (string, error) {
	return solve(input, 0x00000fff)
}

func Part2(input string) (string, error) {
	return solve(input, 0x000000ff)
}

func solve(input string, limit uint32) (string, error) {
	end := len(input) - 1
	if input[end] == '\n' {
		input = input[:len(input)-1]
	}
	i := 1
	for {
		s := fmt.Sprintf("%s%d", input, i)
		sum := md5.Sum([]byte(s))
		v := binary.BigEndian.Uint32(sum[:4])
		if v <= limit {
			break
		}
		i++
	}
	return fmt.Sprint(i), nil
}
