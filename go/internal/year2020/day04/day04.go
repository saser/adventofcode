package day04

import (
	"errors"
	"fmt"
	"io/ioutil"
	"strconv"
	"strings"
)

var ecls = map[string]struct{}{
	"amb": {},
	"blu": {},
	"brn": {},
	"gry": {},
	"grn": {},
	"hzl": {},
	"oth": {},
}

type passport struct {
	Byr, Iyr, Eyr string
	Hgt           string
	Hcl, Ecl      string
	Pid, Cid      string
}

func validYear(s string, low, high int) bool {
	year, err := strconv.Atoi(s)
	if err != nil {
		return false
	}
	return year >= low && year <= high
}

func validHgt(s string) bool {
	cm := strings.Contains(s, "cm")
	in := strings.Contains(s, "in")
	if !cm && !in {
		return false
	}
	var (
		n, low, high int
		unit         string
	)
	switch {
	case cm:
		unit = "cm"
		low, high = 150, 193
	case in:
		unit = "in"
		low, high = 59, 76
	}
	n, err := strconv.Atoi(strings.ReplaceAll(s, unit, ""))
	if err != nil {
		return false
	}
	return n >= low && n <= high
}

func validHcl(s string) bool {
	if len(s) != 7 {
		return false
	}
	if s[0] != '#' {
		return false
	}
	for _, r := range s[1:] {
		if (r < '0' || r > '9') && (r < 'a' || r > 'f') {
			return false
		}
	}
	return true
}

func validEcl(s string) bool {
	_, ok := ecls[s]
	return ok
}

func validPid(s string) bool {
	if len(s) != 9 {
		return false
	}
	for _, r := range s {
		if r < '0' || r > '9' {
			return false
		}
	}
	return true
}

func (p *passport) ReadLine(line string) error {
	for _, pair := range strings.Split(line, " ") {
		parts := strings.Split(pair, ":")
		key, value := parts[0], parts[1]
		switch key {
		case "byr":
			p.Byr = value
		case "iyr":
			p.Iyr = value
		case "eyr":
			p.Eyr = value
		case "hgt":
			p.Hgt = value
		case "hcl":
			p.Hcl = value
		case "ecl":
			p.Ecl = value
		case "pid":
			p.Pid = value
		case "cid":
			p.Cid = value
		}
	}
	return nil
}

func (p *passport) Validate(strict bool) error {
	switch {
	case p.Byr == "", p.Iyr == "", p.Eyr == "",
		p.Hgt == "",
		p.Hcl == "", p.Ecl == "",
		p.Pid == "":
		return errors.New("passport is invalid")
	}
	if !strict {
		return nil
	}
	// byr
	if !validYear(p.Byr, 1920, 2002) {
		return fmt.Errorf("invalid byr %q", p.Byr)
	}
	// iyr
	if !validYear(p.Iyr, 2010, 2020) {
		return fmt.Errorf("invalid iyr %q", p.Iyr)
	}
	// eyr
	if !validYear(p.Eyr, 2020, 2030) {
		return fmt.Errorf("invalid eyr %q", p.Eyr)
	}
	// hgt
	if !validHgt(p.Hgt) {
		return fmt.Errorf("invalid hgt: %q", p.Hgt)
	}
	// hcl
	if !validHcl(p.Hcl) {
		return fmt.Errorf("invalid hcl: %q", p.Hcl)
	}
	// ecl
	if !validEcl(p.Ecl) {
		return fmt.Errorf("invalid ecl: %q", p.Ecl)
	}
	// pid
	if !validPid(p.Pid) {
		return fmt.Errorf("invalid pid: %q", p.Pid)
	}
	return nil
}

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	data, err := ioutil.ReadAll(r)
	if err != nil {
		return "", fmt.Errorf("part %v: %w", part, err)
	}
	input := string(data)
	rawPassports := strings.Split(input, "\n\n")
	validCount := 0
	for _, raw := range rawPassports {
		var p passport
		for _, line := range strings.Split(raw, "\n") {
			if line == "" {
				continue
			}
			if err := p.ReadLine(line); err != nil {
				continue
			}
		}
		if err := p.Validate(part == 2); err != nil {
			continue
		}
		validCount++
	}
	return fmt.Sprint(validCount), nil
}
