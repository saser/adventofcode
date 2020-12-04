package day04

import (
	"errors"
	"fmt"
	"io"
	"io/ioutil"
	"regexp"
	"strconv"
	"strings"
)

var (
	yearRE  = regexp.MustCompile(`^\d{4}$`)
	hgtRE   = regexp.MustCompile(`^(?P<n>\d+)(?P<unit>(in|cm))$`)
	hgtN    = hgtRE.SubexpIndex("n")
	hgtUnit = hgtRE.SubexpIndex("unit")
	hclRE   = regexp.MustCompile(`^#[0-9a-f]{6}$`)
	eclRE   = regexp.MustCompile(`^(amb|blu|brn|gry|grn|hzl|oth)$`)
	pidRE   = regexp.MustCompile(`^\d{9}$`)
)

func init() {
	if hgtN < 0 {
		panic("invalid hgt regex: group `n` not found")
	}
	if hgtUnit < 0 {
		panic("invalid hgt regex: group `unit` not found")
	}
}

type passport struct {
	Byr, Iyr, Eyr string
	Hgt           string
	Hcl, Ecl      string
	Pid, Cid      string
}

func readYear(s string) (int, error) {
	if !yearRE.MatchString(s) {
		return 0, fmt.Errorf("invalid year: %q", s)
	}
	return strconv.Atoi(s)
}

func validateYear(s string, low, high int) error {
	year, err := readYear(s)
	if err != nil {
		return err
	}
	if year < low || year > high {
		return fmt.Errorf("invalid year %v: must be in the range [%v, %v]", year, low, high)
	}
	return nil
}

func readHgt(s string) (int, string, error) {
	matches := hgtRE.FindStringSubmatch(s)
	if matches == nil {
		return 0, "", fmt.Errorf("invalid hgt: %q", s)
	}
	n, err := strconv.Atoi(matches[hgtN])
	if err != nil {
		return 0, "", fmt.Errorf("invalid hgt: %q: %w", s, err)
	}
	return n, matches[hgtUnit], nil
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
	if err := validateYear(p.Byr, 1920, 2002); err != nil {
		return fmt.Errorf("invalid byr %q: %w", p.Byr, err)
	}
	// iyr
	if err := validateYear(p.Iyr, 2010, 2020); err != nil {
		return fmt.Errorf("invalid iyr %q: %w", p.Iyr, err)
	}
	// eyr
	if err := validateYear(p.Eyr, 2020, 2030); err != nil {
		return fmt.Errorf("invalid eyr %q: %w", p.Eyr, err)
	}
	// hgt
	n, unit, err := readHgt(p.Hgt)
	if err != nil {
		return fmt.Errorf("invalid hgt %q: %w", p.Hgt, err)
	}
	var low, high int
	switch unit {
	case "cm":
		low, high = 150, 193
	case "in":
		low, high = 59, 76
	}
	if n < low || n > high {
		return fmt.Errorf("invalid hgt %q: %s %v must be in range [%v, %v]", p.Hgt, unit, n, low, high)
	}
	// hcl
	if !hclRE.MatchString(p.Hcl) {
		return fmt.Errorf("invalid hcl: %q", p.Hcl)
	}
	// ecl
	if !eclRE.MatchString(p.Ecl) {
		return fmt.Errorf("invalid ecl: %q", p.Ecl)
	}
	// pid
	if !pidRE.MatchString(p.Pid) {
		return fmt.Errorf("invalid pid: %q", p.Pid)
	}
	return nil
}

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
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
