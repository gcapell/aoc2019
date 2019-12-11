package main

import (
	"fmt"
	"io/ioutil"
	"log"
	"sort"
	"strings"
)

func main() {
	data, err := ioutil.ReadFile("10.txt")
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(best(string(data)))
}

type point struct{ x, y int }
type asteroid struct {
	point
	d int
}

func best(s string) (point, int) {
	points := findAsteroids(s)
	withDistance := makeAsteroids(points)
	max := 0
	var maxA point
	for _, p := range points {
		d := count(withDistance, p)
		if d > max {
			max = d
			maxA = p
		}
	}
	return maxA, max
}

func makeAsteroids(points []point) []*asteroid {
	withDistance := make([]*asteroid, len(points))
	for j, p := range points {
		withDistance[j] = &asteroid{p, 0}
	}
	return withDistance
}

func findAsteroids(s string) []point {
	lines := strings.Fields(s)
	var reply []point
	for y := 0; y < len(lines); y++ {
		for x := 0; x < len(lines[0]); x++ {
			if lines[y][x] == '#' {
				reply = append(reply, point{x, y})
			}
		}
	}
	return reply
}

func count(asteroids []*asteroid, o point) int {
	for _, a := range asteroids {
		a.d = d2(a.point, o)
	}
	sort.Slice(asteroids, func(i, j int) bool { return asteroids[i].d < asteroids[j].d })
	if asteroids[0].point != o {
		panic("sort?")
	}
	count := 0
outer:
	for n, a := range asteroids {
		if n == 0 {
			continue
		}
		for _, b := range asteroids[1:n] {
			if inline(o, b.point, a.point) {
				continue outer
			}
		}
		count++
	}
	return count
}

func d2(a, b point) int {
	dx := a.x - b.x
	dy := a.y - b.y
	return dx*dx + dy*dy
}

func inline(a, b, c point) bool {
	dx1 := b.x - a.x
	dx2 := c.x - b.x
	if !sameSign(dx1, dx2) {
		return false
	}
	dy1 := b.y - a.y
	dy2 := c.y - b.y
	if !sameSign(dy1, dy2) {
		return false
	}
	return dy1*dx2 == dx1*dy2
}

func sameSign(a, b int) bool {
	switch {
	case a < 0:
		return b < 0
	case a == 0:
		return b == 0
	default:
		return b > 0
	}
}
