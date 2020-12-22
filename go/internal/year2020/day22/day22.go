package day22

import (
	"container/list"
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

func solve(input string, part int) (string, error) {
	if part == 2 {
		return "", fmt.Errorf("solution not implemented for part %v", part)
	}
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	player1 := list.New()
	for _, card := range parsePlayer(paragraphs[0]) {
		player1.PushBack(card)
	}
	player2 := list.New()
	for _, card := range parsePlayer(paragraphs[1]) {
		player2.PushBack(card)
	}
	for player1.Front() != nil && player2.Front() != nil {
		card1 := player1.Remove(player1.Front()).(int)
		card2 := player2.Remove(player2.Front()).(int)
		var (
			high, low int
			winner    *list.List
		)
		if card1 > card2 {
			high = card1
			low = card2
			winner = player1
		} else {
			high = card2
			low = card1
			winner = player2
		}
		winner.PushBack(high)
		winner.PushBack(low)
	}
	var winner *list.List
	if player1.Front() != nil {
		winner = player1
	} else {
		winner = player2
	}
	score := 0
	for f, elem := 1, winner.Back(); elem != nil; f, elem = f+1, elem.Prev() {
		score += elem.Value.(int) * f
	}
	return fmt.Sprint(score), nil
}
