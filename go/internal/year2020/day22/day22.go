package day22

import (
	"bytes"
	"fmt"
	"hash/maphash"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type deck []uint8

func newDeck(cards []uint8) *deck {
	d := make(deck, len(cards))
	copy(d, cards)
	return &d
}

func (d *deck) Clone() *deck {
	d2 := make(deck, len(*d))
	copy(d2, *d)
	return &d2
}

func (d *deck) Empty() bool {
	return len(*d) == 0
}

func (d *deck) PopFront() uint8 {
	v := (*d)[0]
	*d = (*d)[1:]
	return v
}

func (d *deck) PushBack(v uint8) {
	*d = append(*d, v)
}

func (d *deck) String() string {
	ss := make([]string, len(*d))
	for i, card := range *d {
		ss[i] = fmt.Sprint(card)
	}
	return strings.Join(ss, ",")
}

func parsePlayer(paragraph string) []uint8 {
	lines := strings.Split(paragraph, "\n")[1:] // skip the first line
	cards := make([]uint8, len(lines))
	for i, line := range lines {
		card, err := strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
		cards[i] = uint8(card)
	}
	return cards
}

func combat(deck1, deck2 *deck) *deck {
	for !deck1.Empty() && !deck2.Empty() {
		card1 := deck1.PopFront()
		card2 := deck2.PopFront()
		if card1 > card2 {
			deck1.PushBack(card1)
			deck1.PushBack(card2)
		} else {
			deck2.PushBack(card2)
			deck2.PushBack(card1)
		}
	}
	if !deck1.Empty() {
		return deck1
	}
	return deck2
}

type cacheEntry struct {
	deck1, deck2 *deck
}

type cache struct {
	m map[uint64][]cacheEntry
	h maphash.Hash
}

func newCache() *cache {
	return &cache{
		m: make(map[uint64][]cacheEntry),
	}
}

func (c *cache) hash(deck1, deck2 *deck) uint64 {
	c.h.Reset()
	c.h.Write(*deck1)
	c.h.WriteByte(0) // assumes that 0 does not appear in the input
	c.h.Write(*deck2)
	return c.h.Sum64()
}

func (c *cache) Contains(deck1, deck2 *deck) bool {
	hash := c.hash(deck1, deck2)
	entries, ok := c.m[hash]
	if !ok {
		return false
	}
	for _, entry := range entries {
		if bytes.Equal(*entry.deck1, *deck1) && bytes.Equal(*entry.deck2, *deck2) {
			return true
		}
	}
	return false
}

func (c *cache) Store(deck1, deck2 *deck) {
	hash := c.hash(deck1, deck2)
	entry := cacheEntry{
		deck1: deck1.Clone(),
		deck2: deck2.Clone(),
	}
	c.m[hash] = append(c.m[hash], entry)
}

func subGame(deck1, deck2 *deck) (winner *deck) {
	// seen holds the set of previously seen configurations in this
	// game. Its elements are computed using the memoKey function.
	seen := newCache()
	for !deck1.Empty() && !deck2.Empty() {
		if ok := seen.Contains(deck1, deck2); ok {
			return deck1
		}
		seen.Store(deck1, deck2)
		card1 := deck1.PopFront()
		card2 := deck2.PopFront()
		var (
			roundWinner   *deck
			first, second uint8
		)
		if uint8(len(*deck1)) >= card1 && uint8(len(*deck2)) >= card2 {
			subDeck1 := make(deck, card1)
			copy(subDeck1, (*deck1)[:card1])
			subDeck2 := make(deck, card2)
			copy(subDeck2, (*deck2)[:card2])
			subWinner := subGame(&subDeck1, &subDeck2)
			switch subWinner {
			case &subDeck1:
				roundWinner = deck1
				first = card1
				second = card2
			case &subDeck2:
				roundWinner = deck2
				first = card2
				second = card1
			}
		} else {
			if card1 > card2 {
				roundWinner = deck1
				first = card1
				second = card2
			} else {
				roundWinner = deck2
				first = card2
				second = card1
			}
		}
		roundWinner.PushBack(first)
		roundWinner.PushBack(second)
	}
	if !deck1.Empty() {
		return deck1
	}
	return deck2
}

func recursiveCombat(deck1, deck2 *deck) *deck {
	return subGame(deck1, deck2)
}

func solve(input string, part int) (string, error) {
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	deck1 := newDeck(parsePlayer(paragraphs[0]))
	deck2 := newDeck(parsePlayer(paragraphs[1]))
	var winner *deck
	switch part {
	case 1:
		winner = combat(deck1, deck2)
	case 2:
		winner = recursiveCombat(deck1, deck2)
	}
	score := 0
	f := len(*winner)
	for _, card := range *winner {
		score += int(card) * f
		f--
	}
	return fmt.Sprint(score), nil
}
