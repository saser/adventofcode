package day22

import (
	"fmt"
	"strconv"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type deck []int

func newDeck(cards []int) *deck {
	d := make(deck, len(cards))
	copy(d, cards)
	return &d
}

func (d *deck) Empty() bool {
	return len(*d) == 0
}

func (d *deck) PopFront() int {
	v := (*d)[0]
	*d = (*d)[1:]
	return v
}

func (d *deck) PushBack(v int) {
	*d = append(*d, v)
}

func (d *deck) String() string {
	ss := make([]string, len(*d))
	for i, card := range *d {
		ss[i] = fmt.Sprint(card)
	}
	return strings.Join(ss, ",")
}

func parsePlayer(paragraph string) []int {
	lines := strings.Split(paragraph, "\n")[1:] // skip the first line
	cards := make([]int, len(lines))
	for i, line := range lines {
		card, err := strconv.Atoi(line)
		if err != nil {
			panic(err)
		}
		cards[i] = card
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

func recursiveCombat(deck1, deck2 *deck) *deck {
	// memo holds the set of previously seen configurations in this
	// game. Its elements are computed using the memoKey function.
	memo := make(map[string]bool)
	memoKey := func(deck1, deck2 *deck) string {
		return deck1.String() + "/" + deck2.String()
	}
	for !deck1.Empty() && !deck2.Empty() {
		if memo[memoKey(deck1, deck2)] {
			return deck1
		}
		memo[memoKey(deck1, deck2)] = true
		card1 := deck1.PopFront()
		card2 := deck2.PopFront()
		var (
			winner        *deck
			first, second int
		)
		if len(*deck1) >= card1 && len(*deck2) >= card2 {
			subDeck1 := make(deck, card1)
			copy(subDeck1, (*deck1)[:card1])
			subDeck2 := make(deck, card2)
			copy(subDeck2, (*deck2)[:card2])
			subWinner := recursiveCombat(&subDeck1, &subDeck2)
			switch subWinner {
			case &subDeck1:
				winner = deck1
				first = card1
				second = card2
			case &subDeck2:
				winner = deck2
				first = card2
				second = card1
			}
		} else {
			if card1 > card2 {
				winner = deck1
				first = card1
				second = card2
			} else {
				winner = deck2
				first = card2
				second = card1
			}
		}
		winner.PushBack(first)
		winner.PushBack(second)
	}
	var winner *deck
	if !deck1.Empty() {
		winner = deck1
	} else {
		winner = deck2
	}
	return winner
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
		score += card * f
		f--
	}
	return fmt.Sprint(score), nil
}
