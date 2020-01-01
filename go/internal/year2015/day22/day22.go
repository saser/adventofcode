package day22

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	boss, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 22, part %d: %w", part, err)
	}
	fmt.Printf("%#v\n", boss)
	player := playerStats{
		hitpoints: 50,
		armor:     0,
		mana:      500,
	}
	fmt.Printf("%#v\n", player)
	return "", errors.New("not implemented yet")
}

type bossStats struct {
	hitpoints int
	damage    int
}

type playerStats struct {
	hitpoints int
	armor     int
	mana      int
}

func parse(r io.Reader) (bossStats, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var values []int
	for sc.Scan() {
		parts := strings.Split(sc.Text(), ": ")
		v, err := strconv.Atoi(parts[1])
		if err != nil {
			return bossStats{}, fmt.Errorf("parse: %w", err)
		}
		values = append(values, v)
	}
	return bossStats{
		hitpoints: values[0],
		damage:    values[1],
	}, nil
}
