package day20

import (
	"fmt"
	"strings"
)

func Part1(input string) (string, error) {
	return solve(input, 1)
}

func Part2(input string) (string, error) {
	return solve(input, 2)
}

type direction int

const (
	dirTop direction = iota
	dirRight
	dirBottom
	dirLeft
)

type border int64

func newBorder(bits []bool) border {
	b := 0
	for _, bit := range bits {
		b <<= 1
		if bit {
			b |= 1
		}
	}
	return border(b)
}

func (b border) Reverse() border {
	r := 0
	for i := 0; i < 10; i++ {
		if b&(1<<i) != 0 {
			r |= 1 << (9 - i)
		}
	}
	return border(r)
}

func (b border) BitString() string {
	var sb strings.Builder
	for i := 9; i >= 0; i-- {
		bit := b&(1<<i) != 0
		if bit {
			sb.WriteRune('#')
		} else {
			sb.WriteRune('.')
		}
	}
	return sb.String()
}

type grid [][]bool

func parseGrid(lines []string) *grid {
	rowCount := len(lines)
	colCount := len(lines[0])
	g := make([][]bool, rowCount)
	for row, line := range lines {
		g[row] = make([]bool, colCount)
		for col, r := range line {
			g[row][col] = r == '#'
		}
	}
	return (*grid)(&g)
}

func (g *grid) Clone() *grid {
	g2 := make([][]bool, len(*g))
	for row := 0; row < g.rowCount(); row++ {
		g2[row] = make([]bool, g.colCount())
		copy(g2[row], g.Row(row))
	}
	return (*grid)(&g2)
}

func (g *grid) Shrink() {
	*g = (*g)[1 : len(*g)-1] // removes the top and bottom rows
	for i, row := range *g {
		(*g)[i] = row[1 : len(row)-1] // removes the first and last column
	}
}

func (g *grid) BitCount() int {
	count := 0
	for _, row := range *g {
		for _, b := range row {
			if b {
				count++
			}
		}
	}
	return count
}

func (g *grid) rowCount() int {
	return len(*g)
}

func (g *grid) colCount() int {
	return len((*g)[0])
}

func (g *grid) Row(i int) []bool {
	return (*g)[i]
}

func (g *grid) Col(i int) []bool {
	col := make([]bool, g.rowCount())
	for row := 0; row < g.rowCount(); row++ {
		col[row] = (*g)[row][i]
	}
	return col
}

func (g *grid) Top() border {
	return newBorder(g.Row(0))
}

func (g *grid) Right() border {
	return newBorder(g.Col(g.colCount() - 1))
}

func (g *grid) Bottom() border {
	return newBorder(g.Row(g.rowCount() - 1)).Reverse()
}

func (g *grid) Left() border {
	return newBorder(g.Col(0)).Reverse()
}

func (g *grid) RotateRight() {
	cols := g.colCount()
	rows := g.rowCount()
	g2 := make([][]bool, cols)
	for col := 0; col < cols; col++ {
		// The i:th row in the new grid is the reverse of the i:th
		// column in the old grid.
		reversed := make([]bool, rows)
		copy(reversed, g.Col(col))
		for row := 0; row < rows/2; row++ {
			reversed[row], reversed[rows-row-1] = reversed[rows-row-1], reversed[row]
		}
		g2[col] = reversed
	}
	*g = g2
}

func (g *grid) FlipVertically() {
	for row := 0; row < g.rowCount()/2; row++ {
		opposite := g.rowCount() - row - 1
		(*g)[row], (*g)[opposite] = (*g)[opposite], (*g)[row]
	}
}

func (g *grid) String() string {
	var sb strings.Builder
	for row := 0; row < g.rowCount(); row++ {
		for col := 0; col < g.colCount(); col++ {
			if (*g)[row][col] {
				sb.WriteRune('#')
			} else {
				sb.WriteRune('.')
			}
		}
		sb.WriteRune('\n')
	}
	return sb.String()
}

func (g *grid) Print() {
	fmt.Println(g.String())
}

func (g *grid) AppendRight(other *grid) {
	if g.rowCount() != other.rowCount() {
		panic(fmt.Errorf("g.rowCount(), other.rowCount() = %v, %v", g.rowCount(), other.rowCount()))
	}
	for row := 0; row < g.rowCount(); row++ {
		(*g)[row] = append((*g)[row], (*other)[row]...)
	}
}

func (g *grid) AppendBelow(other *grid) {
	if g.colCount() != other.colCount() {
		panic(fmt.Errorf("g.colCount(), other.colCount() = %v, %v", g.colCount(), other.colCount()))
	}
	for _, row := range *other {
		r := make([]bool, len(row))
		copy(r, row)
		*g = append(*g, r)
	}
}

func (g *grid) patternMatch(pattern *grid, startRow, startCol int) bool {
	count := 0
	for row := 0; row < pattern.rowCount(); row++ {
		for col := 0; col < pattern.colCount(); col++ {
			if !(*pattern)[row][col] {
				continue
			}
			if !(*g)[row+startRow][col+startCol] {
				return false
			}
			count++
		}
	}
	return true
}

func (g *grid) PatternMatches(pattern *grid) int {
	count := 0
	for startRow := 0; startRow < g.rowCount()-pattern.rowCount()+1; startRow++ {
		for startCol := 0; startCol < g.colCount()-pattern.colCount()+1; startCol++ {
			if g.patternMatch(pattern, startRow, startCol) {
				count++
			}
		}
	}
	return count
}

type tile struct {
	id int64
	g  *grid
}

func parseTile(paragraph string) *tile {
	lines := strings.Split(paragraph, "\n")
	idLine := lines[0]
	var id int64
	if _, err := fmt.Sscanf(idLine, "Tile %d:", &id); err != nil {
		panic(err)
	}
	g := parseGrid(lines[1:])
	return &tile{
		id: id,
		g:  g,
	}
}

func (t *tile) Clone() *tile {
	return &tile{
		id: t.id,
		g:  t.g.Clone(),
	}
}

func (t *tile) Top() border {
	return t.g.Top()
}

func (t *tile) Right() border {
	return t.g.Right()
}

func (t *tile) Bottom() border {
	return t.g.Bottom()
}

func (t *tile) Left() border {
	return t.g.Left()
}

func (t *tile) Borders() []border {
	return []border{
		t.Top(),
		t.Right(),
		t.Bottom(),
		t.Left(),
	}
}

func (t *tile) contains(b border) bool {
	switch b {
	case t.Top(), t.Right(), t.Bottom(), t.Left():
		return true
	default:
		return false
	}
}

func (t *tile) Border(dir direction) border {
	switch dir {
	case dirTop:
		return t.Top()
	case dirRight:
		return t.Right()
	case dirBottom:
		return t.Bottom()
	case dirLeft:
		return t.Left()
	}
	panic("unreachable")
}

func (t *tile) Direction(b border) direction {
	switch b {
	case t.Top():
		return dirTop
	case t.Right():
		return dirRight
	case t.Bottom():
		return dirBottom
	case t.Left():
		return dirLeft
	}
	panic("unreachable")
}

func (t *tile) RotateRight() {
	t.g.RotateRight()
}

func (t *tile) FlipVertically() {
	t.g.FlipVertically()
}

func (t *tile) Orient(b border, dir direction) {
	if !t.contains(b) {
		// In its current configuration, the tile does not contain the
		// border at all. This means that the border we are looking for
		// are one of the reverses. Flipping (either horizontally or
		// vertically) will result in all current borders being reversed
		// (but not necessarily in their original position).
		t.FlipVertically()
	}
	for t.Border(dir) != b {
		t.RotateRight()
	}
}

type puzzle struct {
	tiles map[border][]*tile // border -> tiles containing that border
}

func newPuzzle(tiles []*tile) *puzzle {
	matches := make(map[border][]*tile)
	for _, t := range tiles {
		for _, b := range t.Borders() {
			matches[b] = append(matches[b], t)
		}
		flipped := t.Clone()
		flipped.FlipVertically()
		for _, b := range flipped.Borders() {
			matches[b] = append(matches[b], flipped)
		}
	}
	return &puzzle{
		tiles: matches,
	}
}

// UnmatchedSides returns the number of tile borders for which there is no other
// tile in the puzzle that matches.
func (p *puzzle) UnmatchedSides(t *tile) int {
	count := 4
	for _, b := range t.Borders() {
		for _, other := range p.tiles[b] {
			if other.id == t.id {
				continue
			}
			count--
		}
	}
	return count
}

// Matching returns all tiles that share a border with the given tile. Two tiles
// t1 and t2 share a border b if t1.Borders() contains b and t2.Borders()
// contains b.Reverse(). Any tile in the returned map will have a different ID
// than the given tile.
func (p *puzzle) Matching(t *tile) map[border]*tile {
	matching := make(map[border]*tile)
	for _, b := range t.Borders() {
		for _, other := range p.tiles[b.Reverse()] {
			if other.id == t.id {
				continue
			}
			matching[b] = other
		}
	}
	return matching
}

func (p *puzzle) AssembleRow(leftMost *tile) []*tile {
	row := []*tile{leftMost}
	rightMost := leftMost
	for {
		matching := p.Matching(rightMost)
		b := rightMost.Right()
		next, ok := matching[b]
		if !ok {
			break
		}
		next.Orient(b.Reverse(), dirLeft)
		row = append(row, next)
		rightMost = next
	}
	return row
}

func solve(input string, part int) (string, error) {
	paragraphs := strings.Split(strings.TrimSpace(input), "\n\n")
	tiles := make([]*tile, len(paragraphs))
	for i, p := range paragraphs {
		tiles[i] = parseTile(p)
	}
	p := newPuzzle(tiles)
	var prod int64 = 1
	var cornerTile *tile
	for _, t := range tiles {
		if p.UnmatchedSides(t) == 2 {
			if cornerTile == nil {
				cornerTile = t
			}
			prod *= t.id
		}
	}
	if part == 1 {
		return fmt.Sprint(prod), nil
	}

	// Orient the corner tile so that its matched borders are facing right
	// and bottom (i.e., it is the top-left corner tile). Do this by taking
	// one of the matched borders, orient it to the right. If the other
	// border is then facing upwards, flip the tile vertically.
	var one, other *border
	for b := range p.Matching(cornerTile) {
		b := b
		if one == nil {
			one = &b
			continue
		}
		other = &b
	}
	cornerTile.Orient(*one, dirRight)
	if cornerTile.Direction(*other) == dirTop {
		cornerTile.FlipVertically()
	}

	// When the corner tile has been oriented, we can begin assembling the
	// entire image. This is done by assembling each row left to right, and
	// assembling all rows top to bottom.
	var assembled [][]*tile
	leftMost := cornerTile
	for {
		assembled = append(assembled, p.AssembleRow(leftMost))
		matching := p.Matching(leftMost)
		b := leftMost.Bottom()
		down, ok := matching[b]
		if !ok {
			break
		}
		down.Orient(b.Reverse(), dirTop)
		leftMost = down
	}

	// The entire image has now been assembled as a 2D array of
	// tiles. Construct one giant grid from that array, to represent the
	// entire image which can then be rotated and flipped.
	var image *grid
	for _, row := range assembled {
		var imageRow *grid
		for _, t := range row {
			if imageRow == nil {
				imageRow = t.g.Clone()
				imageRow.Shrink()
				continue
			}
			shrunk := t.g.Clone()
			shrunk.Shrink()
			imageRow.AppendRight(shrunk)
		}
		if image == nil {
			image = imageRow
			continue
		}
		image.AppendBelow(imageRow)
	}

	// Look for the number of times the pattern appears in the image. I
	// interpret the instructions to mean that the pattern only appears in
	// one configuration of the input. Therefore, if the pattern is not
	// found in the current configuration, the image is rotated and checked
	// again. This happens at most 4 times. If the pattern is still not
	// found, the image is flipped and then all rotations are searched
	// again.
	pattern := parseGrid([]string{
		"                  # ",
		"#    ##    ##    ###",
		" #  #  #  #  #  #   ",
	})
	count := image.PatternMatches(pattern)
	rotations := 0
	for count == 0 {
		count = image.PatternMatches(pattern)
		image.RotateRight()
		rotations++
		if rotations == 4 {
			image.FlipVertically()
			rotations = 0
		}
	}
	roughness := image.BitCount() - count*pattern.BitCount()
	return fmt.Sprint(roughness), nil
}
