package main

import "testing"

type td struct {
	grid string
	p    point
	n    int
}

var board1 = `
	.#..#
	.....
	#####
	....#
	...##`

var board2 = `......#.#.
		#..#.#....
		..#######.
		.#.#.###..
		.#..#.....
		..#....#.#
		#..#....#.
		.##.#..###
		##...#..#.
		.#....####`

func TestCount(t *testing.T) {
	testdata := []td{
		{board1, point{1, 0}, 7},
	}
	for _, td := range testdata {
		got := count(makeAsteroids(findAsteroids(td.grid)), td.p)
		if got != td.n {
			t.Errorf("count(%s) -> %d want %d",
				td.grid, got, td.n)
		}
	}
}

func TestDetect(t *testing.T) {
	testdata := []td{
		{board1, point{3, 4}, 8},
		{board2, point{5, 8}, 33},
	}
	for _, td := range testdata {
		p, n := best(td.grid)
		if !(p == td.p && n == td.n) {
			t.Errorf("best(%s) -> %v,%v; want %v, %v",
				td.grid, p, n, td.p, td.n)
		}
	}
}
