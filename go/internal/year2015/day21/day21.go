package day21

import (
	"errors"
	"fmt"
	"io"
)

func Part1(r io.Reader) (string, error) {
	for _, loadout := range possibleLoadouts(shopInventory) {
		fmt.Println(loadout)
	}
	return "", errors.New("not implemented yet")
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
