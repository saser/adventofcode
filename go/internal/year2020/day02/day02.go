package day02

import (
	"bufio"
	"fmt"
	"io"
	"regexp"
	"strconv"
)

var (
	re            = regexp.MustCompile(`(?P<low>\d+)-(?P<high>\d+) (?P<letter>\w): (?P<password>\w+)`)
	lowIndex      = re.SubexpIndex("low")
	highIndex     = re.SubexpIndex("high")
	letterIndex   = re.SubexpIndex("letter")
	passwordIndex = re.SubexpIndex("password")
)

func init() {
	if lowIndex < 0 {
		panic("invalid regex: group `low` not found")
	}
	if highIndex < 0 {
		panic("invalid regex: group `high` not found")
	}
	if letterIndex < 0 {
		panic("invalid regex: group `letter` not found")
	}
	if passwordIndex < 0 {
		panic("invalid regex: group `password` not found")
	}
}

type entry struct {
	Low, High int
	Letter    rune
	Password  string
}

func parse(s string) (entry, error) {
	wrap := func(err error) error {
		return fmt.Errorf("parse: invalid entry %q: %w", s, err)
	}
	matches := re.FindStringSubmatch(s)
	lowS := matches[lowIndex]
	low, err := strconv.Atoi(lowS)
	if err != nil {
		return entry{}, wrap(err)
	}
	highS := matches[highIndex]
	high, err := strconv.Atoi(highS)
	if err != nil {
		return entry{}, wrap(err)
	}
	letterS := matches[letterIndex]
	letter := rune(letterS[0])
	password := matches[passwordIndex]
	return entry{
		Low:      low,
		High:     high,
		Letter:   letter,
		Password: password,
	}, nil
}

func (e entry) ValidCount() bool {
	letterCount := 0
	for _, r := range e.Password {
		if r == e.Letter {
			letterCount++
		}
	}
	return letterCount >= e.Low && letterCount <= e.High
}

func (e entry) ValidPosition() bool {
	return (rune(e.Password[e.Low-1]) == e.Letter) != (rune(e.Password[e.High-1]) == e.Letter)
}

func (e *entry) String() string {
	return fmt.Sprintf("%d-%d %s: %s", e.Low, e.High, string(e.Letter), e.Password)
}

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	validCount := 0
	for sc.Scan() {
		e, err := parse(sc.Text())
		if err != nil {
			return "", fmt.Errorf("part %v: %w", part, err)
		}
		switch {
		case part == 1 && e.ValidCount():
			validCount++
		case part == 2 && e.ValidPosition():
			validCount++
		}
	}
	return fmt.Sprint(validCount), nil
}
