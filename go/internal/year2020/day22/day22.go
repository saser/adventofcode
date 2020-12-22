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
		var (
			high, low int
			winner    *deck
		)
		if card1 > card2 {
			high = card1
			low = card2
			winner = deck1
		} else {
			high = card2
			low = card1
			winner = deck2
		}
		winner.PushBack(high)
		winner.PushBack(low)
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
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	deck1 := newDeck(parsePlayer(paragraphs[0]))
	deck2 := newDeck(parsePlayer(paragraphs[1]))
	winner := combat(deck1, deck2)
	score := 0
	f := len(*winner)
	for _, card := range *winner {
		score += card * f
		f--
	}
	return fmt.Sprint(score), nil
}
