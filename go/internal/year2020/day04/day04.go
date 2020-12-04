package day04

import (
	"fmt"
	"io"
	"io/ioutil"
	"strings"
)

type passport struct {
	Byr, Iyr, Eyr string
	Hgt           string
	Hcl, Ecl      string
	Pid, Cid      string
}

func (p *passport) ReadLine(line string) {
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
}

func (p *passport) Valid() bool {
	switch {
	case p.Byr == "", p.Iyr == "", p.Eyr == "",
		p.Hgt == "",
		p.Hcl == "", p.Ecl == "",
		p.Pid == "":
		return false
	}
	return true
}

func Part1(r io.Reader) (string, error) {
	return solve(r, 1)
}

func Part2(r io.Reader) (string, error) {
	return solve(r, 2)
}

func solve(r io.Reader, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
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
			p.ReadLine(line)
		}
		if p.Valid() {
			validCount++
		}
	}
	return fmt.Sprint(validCount), nil
}
