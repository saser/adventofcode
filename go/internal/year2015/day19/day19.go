package day19

import (
	"bufio"
	"errors"
	"fmt"
	"io"
	"regexp"
	"unicode"
)

func Part1(r io.Reader) (string, error) {
	replacements, molecule, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 19, part 1: %w", err)
	}
	fmt.Println(replacements)
	fmt.Println(molecule)
	return "", errors.New("not implemented yet")
}

func parse(r io.Reader) (map[string][][]string, string, error) {
	replacements, molecule, err := parseMappings(r)
	if err != nil {
		return nil, "", fmt.Errorf("parse: %w", err)
	}
	refined := refineReplacements(replacements)
	return refined, molecule, nil
}

func parseMappings(r io.Reader) (map[string][]string, string, error) {
	re, err := regexp.Compile(`^(\w+) => (\w+)$`)
	if err != nil {
		return nil, "", fmt.Errorf("parse mappings: %w", err)
	}
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	replacements := make(map[string][]string)
	var molecule string
	for sc.Scan() {
		line := sc.Text()
		matches := re.FindStringSubmatch(line)
		if matches == nil {
			if line == "" {
				continue
			}
			molecule = line
			break
		}
		sequences, ok := replacements[matches[1]]
		if !ok {
			sequences = []string{}
		}
		replacements[matches[1]] = append(sequences, matches[2])
	}
	return replacements, molecule, nil
}

func refineReplacements(replacements map[string][]string) map[string][][]string {
	refined := make(map[string][][]string)
	for key, molecules := range replacements {
		for _, molecule := range molecules {
			r, ok := refined[key]
			if !ok {
				r = [][]string{}
			}
			refined[key] = append(r, splitMolecule(molecule))
		}
	}
	return refined
}

func splitMolecule(molecule string) []string {
	upperIndices := make([]int, 0)
	for i, r := range molecule {
		if unicode.IsUpper(r) {
			upperIndices = append(upperIndices, i)
		}
	}
	upperIndices = append(upperIndices, len(molecule))
	nSubstances := len(upperIndices) - 1
	substances := make([]string, 0, nSubstances)
	for i := 0; i < nSubstances; i++ {
		substances = append(substances, molecule[upperIndices[i]:upperIndices[i+1]])
	}
	return substances
}
