package main
import (
	"testing"
	"strings"
)

type td struct {
	grid string
	p point
	n int
}
var testdata = []td{
	{`
	.#..#
	.....
	#####
	....#
	...##`,
	point{3,4},
	8,
	},
	{
	`......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####`,
	point{5,8},
	33,
	},
}

func
TestDetect(t *testing.T) {
	for _, td := range testdata {
		p, n := best(strings.Fields(td.grid))
	if ! (p == td.p && n == td.n) {
		t.Errorf("best(%s) -> %v,%v; want %v, %v", 
			td.grid, p, n, td.p, td.n)
	}
	}
}
