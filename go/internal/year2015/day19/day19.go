package day19

import (
	"fmt"
	"regexp"
	"strings"
	"unicode"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	replacements, molecule, err := parse(input)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 19, part %d: %w", part, err)
	}
	switch part {
	case 1:
		return fmt.Sprint(len(distinctReplacements(molecule, replacements))), nil
	case 2:
		return fmt.Sprint(shortestProduction(molecule)), nil
	default:
		return "", fmt.Errorf("year 2015, day 19: invalid part: %d", part)
	}
}

func parse(input string) (map[string][][]string, []string, error) {
	replacements, molecule, err := parseMappings(input)
	if err != nil {
		return nil, nil, fmt.Errorf("parse: %w", err)
	}
	refined := refineReplacements(replacements)
	return refined, splitMolecule(molecule), nil
}

func parseMappings(input string) (map[string][]string, string, error) {
	re, err := regexp.Compile(`^(\w+) => (\w+)$`)
	if err != nil {
		return nil, "", fmt.Errorf("parse mappings: %w", err)
	}
	replacements := make(map[string][]string)
	var molecule string
	for _, line := range strings.Split(strings.TrimSpace(input), "\n") {
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

func joinSubstances(substances []string) string {
	return strings.Join(substances, "")
}

func replaceSubstance(molecule []string, at int, replacement []string) []string {
	if len(molecule) == 1 {
		return replacement
	}
	return append(molecule[:at], append(replacement, molecule[at+1:]...)...)
}

func distinctReplacements(molecule []string, replacements map[string][][]string) [][]string {
	distinct := make(map[string]struct{})
	for i, substance := range molecule {
		for _, replacement := range replacements[substance] {
			replaced := replaceSubstance(molecule, i, replacement)
			distinct[joinSubstances(replaced)] = struct{}{}
		}
	}
	distinctMolecules := make([][]string, 0, len(distinct))
	for k := range distinct {
		distinctMolecules = append(distinctMolecules, splitMolecule(k))
	}
	return distinctMolecules
}

// This is an implementation of the beautiful solution presented by /u/askalski in this comment:
// https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/cy4etju/
func shortestProduction(molecule []string) int {
	nTokens := len(molecule)
	var nRnAr, nY int
	for _, s := range molecule {
		switch s {
		case "Rn", "Ar":
			nRnAr++
		case "Y":
			nY++
		}
	}
	return nTokens - nRnAr - 2*nY - 1
}
