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
