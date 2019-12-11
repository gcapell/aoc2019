package main
import (
	"log"
	"io/ioutil"
	"strings"
	"fmt"
)

func main() {
	data, err := ioutil.ReadFile("9.txt")
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Fields(string(data))
	fmt.Println(len(lines), len(lines[0]), lines)
	fmt.Println(best(lines))
}

func best(lines[]string) (point, int) {
	max := 0
	var maxp point
	
	for k :=0; k< len(lines); k++ {
		for j :=0; j< len(lines[0]); j++ {
			if lines[k][j] != '#' {
				continue
			}
			p := point{j,k}
			d := detect(lines, point{j,k})
			// fmt.Printf("detect %d,%d -> %d\n", j,k, d)
			if d > max {
				maxp = p
				max = d
			}
		}
	}
	return maxp, max
}


type point struct  {x,y int}

type queue struct  {
	inbound, outbound []point
}

func(q *queue) push(p point) {
	q.inbound = append(q.inbound, p)
}

func (q *queue) empty() bool {
	return len(q.inbound) + len(q.outbound) == 0
}

func (q * queue) pop() point {
	if len(q.outbound) == 0 {
		q.outbound = q.inbound
		q.inbound = nil
		reverse(q.outbound)
	}
	reply := q.outbound[len(q.outbound)-1]
	q.outbound = q.outbound[:len(q.outbound)-1]
	return reply
}

func reverse(r []point) {
	for j,k := 0, len(r)-1; j<k; j,k = j+1, k-1 {
		r[j],r[k] = r[k],r[j]
	}
}

func detect(lines[]string, o point) int {
	obscured := make(map[point]bool)
	seen := make(map[point]bool)
	edge := queue{}
	edge.push(o)
	seen[o] = true
	
	var neighbours [4]point
	
	count := 0
	for !edge.empty() {
		p := edge.pop()
		listNeighbours(p, neighbours[:])
		for _, q := range neighbours {
			if !inBounds(lines, q) {
				continue
			}
			if seen[q] {
				continue
			}
			seen[q] = true
			edge.push(q)
			if lines[q.y][q.x] == '.' || obscured[q] {
				continue
			}
			count++
			obscure(lines, obscured, o, q)
		}
	}
	return count
}

func inBounds(lines[]string, p point) bool {
	return p.y>=0 && p.y < len(lines) && p.x >=0 && p.x < len(lines[0])
}

func listNeighbours(p point, dst []point) {
	dst[0] = point{p.x, p.y+1}
	dst[1] = point{p.x, p.y-1}
	dst[2] = point{p.x+1, p.y}
	dst[3] = point{p.x-1, p.y}
}

func obscure(lines []string, obscure map[point]bool, o, p point) {
	dx := p.x - o.x
	dy := p.y - o.y
	
	g := gcd(abs(dx), abs(dy))
	dx, dy = dx/g, dy/g
	
	q := p
	if o==p {
		log.Fatal("obscure", o, p)
	}
	for {
		q.x += dx
		q.y += dy
		if !inBounds(lines,q) {
			return
		}
		if lines[q.y][q.x] == '#' {
			obscure[q] = true
		}
	}
}

func abs(a int) int {
	if a<0 {
		return -a
	}
	return a
}

func gcd (a,b int) int {
	if a==0 || b ==0 {
		return 1
	}
	if b>a {
		a,b = b,a
	}
	for b != 0 {
		a, b = b, a%b
	}
	return a
}
