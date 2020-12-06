package day22

import (
	"bufio"
	"container/heap"
	"errors"
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
		return "", fmt.Errorf("year 2015, day 22, part %d: %w", part, err)
	}
	player := playerStats{
		hitpoints: 50,
		armor:     0,
		mana:      500,
	}
	mana, err := search(player, boss, part == 2)
	if err != nil {
		return "", fmt.Errorf("year 2015, day 22, part %d: %w", part, err)
	}
	return fmt.Sprint(mana), nil
}

type bossStats struct {
	hitpoints int
	damage    int
}

type playerStats struct {
	hitpoints int
	armor     int
	mana      int
}

func parse(r io.Reader) (bossStats, error) {
	sc := bufio.NewScanner(r)
	sc.Split(bufio.ScanLines)
	var values []int
	for sc.Scan() {
		parts := strings.Split(sc.Text(), ": ")
		v, err := strconv.Atoi(parts[1])
		if err != nil {
			return bossStats{}, fmt.Errorf("parse: %w", err)
		}
		values = append(values, v)
	}
	return bossStats{
		hitpoints: values[0],
		damage:    values[1],
	}, nil
}

type spell struct {
	name string
	mana int
}

var (
	magicMissile = spell{name: "Magic Missile", mana: 53}
	drain        = spell{name: "Drain", mana: 73}
	shield       = spell{name: "Shield", mana: 113}
	poison       = spell{name: "Poison", mana: 173}
	recharge     = spell{name: "Recharge", mana: 229}
)

type state struct {
	player        playerStats
	boss          bossStats
	playerTurn    bool
	shieldTimer   int
	poisonTimer   int
	rechargeTimer int
}

func (s *state) tickEffects() {
	if s.shieldTimer > 0 {
		s.shieldTimer--
		if s.shieldTimer == 0 {
			s.player.armor -= 7
		}
	}
	if s.poisonTimer > 0 {
		s.boss.hitpoints -= 3
		s.poisonTimer--
	}
	if s.rechargeTimer > 0 {
		s.player.mana += 101
		s.rechargeTimer--
	}
}

func (s *state) canCast(sp spell) bool {
	if s.player.mana < sp.mana {
		return false
	}
	switch sp {
	case magicMissile, drain:
		return true
	case shield:
		return s.shieldTimer == 0
	case poison:
		return s.poisonTimer == 0
	case recharge:
		return s.rechargeTimer == 0
	}
	return false
}

func (s *state) cast(sp spell) {
	switch sp {
	case magicMissile:
		s.boss.hitpoints -= 4
	case drain:
		s.boss.hitpoints -= 2
		s.player.hitpoints += 2
	case shield:
		s.player.armor += 7
		s.shieldTimer = 6
	case poison:
		s.poisonTimer = 6
	case recharge:
		s.rechargeTimer = 5
	}
	s.player.mana -= sp.mana
}

func (s *state) bossAttack() {
	damage := s.boss.damage - s.player.armor
	if damage < 1 {
		damage = 1
	}
	s.player.hitpoints -= damage
}

type item struct {
	state state
	mana  int
}

type priorityQueue []item

func (pq priorityQueue) Len() int {
	return len(pq)
}

func (pq priorityQueue) Less(i, j int) bool {
	return pq[i].mana < pq[j].mana
}

func (pq priorityQueue) Swap(i, j int) {
	pq[i], pq[j] = pq[j], pq[i]
}

func (pq *priorityQueue) Push(x interface{}) {
	*pq = append(*pq, x.(item))
}

func (pq *priorityQueue) Pop() interface{} {
	n := len(*pq)
	item := (*pq)[n-1]
	*pq = (*pq)[0 : n-1]
	return item
}

func search(player playerStats, boss bossStats, hard bool) (int, error) {
	state := state{
		player:        player,
		boss:          boss,
		playerTurn:    true,
		shieldTimer:   0,
		poisonTimer:   0,
		rechargeTimer: 0,
	}
	pq := make(priorityQueue, 1)
	pq[0] = item{
		state: state,
		mana:  0,
	}
	heap.Init(&pq)
	for pq.Len() > 0 {
		item := heap.Pop(&pq).(item)
		if item.state.playerTurn && hard {
			item.state.player.hitpoints -= 1
		}
		item.state.tickEffects()
		if item.state.boss.hitpoints <= 0 {
			return item.mana, nil
		} else if item.state.player.hitpoints <= 0 {
			continue
		}
		if !item.state.playerTurn {
			item.state.bossAttack()
			item.state.playerTurn = true
			heap.Push(&pq, item)
			continue
		}
		for _, sp := range []spell{magicMissile, drain, shield, poison, recharge} {
			if !item.state.canCast(sp) {
				continue
			}
			newItem := item
			newItem.state.cast(sp)
			newItem.state.playerTurn = false
			newItem.mana += sp.mana
			heap.Push(&pq, newItem)
		}
	}
	return 0, errors.New("no way to win")
}
