package day21

import (
	"errors"
	"io"
)

func Part1(r io.Reader) (string, error) {
	return "", errors.New("not implemented yet")
}

type character struct {
	hitpoints int
	damage    int
	armor     int
}

func playerWins(player, boss character) bool {
	return false
}
