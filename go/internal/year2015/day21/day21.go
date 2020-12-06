package day21

import (
	"bufio"
	"fmt"
	"io"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

func solve(input string, part int) (string, error) {
	boss, err := parse(r)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 21, part %d: %w", part, err)
	}
	player := character{hitpoints: 100}
	minCost := -1
	maxCost := -1
	for _, loadout := range possibleLoadouts(shopInventory) {
		p := player
		for _, item := range loadout {
			p.apply(item)
		}
		cost := loadoutCost(loadout)
		if playerWins(p, boss) {
			if minCost == -1 || cost < minCost {
				minCost = cost
			}
		} else {
			if maxCost == -1 || cost > maxCost {
				maxCost = cost
			}
		}
	}
	var answer int
	switch part {
	case 1:
		answer = minCost
	case 2:
		answer = maxCost
	}
	return fmt.Sprint(answer), nil
}

func parse(r io.Reader) (character, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	lines := make([]string, 0)
	for sc.Scan() {
		lines = append(lines, sc.Text())
	}
	values := make([]int, 0, len(lines))
	for _, line := range lines {
		parts := strings.Split(line, ": ")
		value, err := strconv.Atoi(parts[1])
		if err != nil {
			return character{}, fmt.Errorf("parse: %w", err)
		}
		values = append(values, value)
	}
	boss := character{
		hitpoints: values[0],
		damage:    values[1],
		armor:     values[2],
	}
	return boss, nil
}

type character struct {
	hitpoints int
	damage    int
	armor     int
}

func (c *character) apply(item item) {
	c.damage += item.damage
	c.armor += item.armor
}

type item struct {
	name   string
	cost   int
	damage int
	armor  int
}

type inventory struct {
	weapons []item
	armors  []item
	rings   []item
}

var shopInventory = inventory{
	weapons: []item{
		{"dagger", 8, 4, 0},
		{"shortsword", 10, 5, 0},
		{"warhammer", 25, 6, 0},
		{"longsword", 40, 7, 0},
		{"greataxe", 74, 8, 0},
	},
	armors: []item{
		{"leather", 13, 0, 1},
		{"chainmail", 31, 0, 2},
		{"splintmail", 53, 0, 3},
		{"bandedmail", 75, 0, 4},
		{"platemail", 102, 0, 5},
	},
	rings: []item{
		{"damage+1", 25, 1, 0},
		{"damage+2", 50, 2, 0},
		{"damage+3", 100, 3, 0},
		{"defense+1", 20, 0, 1},
		{"defense+2", 40, 0, 2},
		{"defense+3", 80, 0, 3},
	},
}

func loadoutCost(loadout []item) int {
	cost := 0
	for _, item := range loadout {
		cost += item.cost
	}
	return cost
}

func possibleLoadouts(inv inventory) [][]item {
	loadouts := make([][]item, 0)
	for _, weapon := range inv.weapons {
		loadouts = append(loadouts, []item{weapon})
	}
	withArmor := make([][]item, 0)
	for _, loadout := range loadouts {
		for _, armor := range inv.armors {
			withArmor = append(withArmor, append(loadout, armor))
		}
	}
	loadouts = append(loadouts, withArmor...)
	withRings := make([][]item, 0)
	for _, loadout := range loadouts {
		for i, ring := range inv.rings {
			withRing := append(loadout, ring)
			withRings = append(withRings, withRing)
			for j := i + 1; j < len(inv.rings); j++ {
				withRings = append(withRings, append(withRing, inv.rings[j]))
			}
		}
	}
	loadouts = append(loadouts, withRings...)
	return loadouts
}

func strikeDamage(damage, armor int) int {
	strikeDamage := damage - armor
	if strikeDamage < 1 {
		strikeDamage = 1
	}
	return strikeDamage
}

func playerWins(player, boss character) bool {
	playerStrikeDamage := strikeDamage(player.damage, boss.armor)
	playerRequiredStrikes := boss.hitpoints/playerStrikeDamage + 1
	bossStrikeDamage := strikeDamage(boss.damage, player.armor)
	bossRequiredStrikes := player.hitpoints/bossStrikeDamage + 1
	return playerRequiredStrikes <= bossRequiredStrikes
}
