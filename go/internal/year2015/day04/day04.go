package day04

import (
	"bufio"
	"crypto/md5"
	"encoding/binary"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 0x00000fff)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 0x000000ff)
}

func solve(r io.Reader, limit uint32) (string, error) {
	br := bufio.NewReader(r)
	prefix, _, err := br.ReadLine()
	if err != nil {
		return "", fmt.Errorf("year 2015, day 04, part 1: %w", err)
	}
	i := 1
	for {
		s := fmt.Sprintf("%s%d", prefix, i)
		sum := md5.Sum([]byte(s))
		v := binary.BigEndian.Uint32(sum[:4])
		if v <= limit {
			break
		}
		i++
	}
	return fmt.Sprint(i), nil
}
