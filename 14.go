package main

import (
	"fmt"
	"log"
	"strconv"
	"strings"
)

func main() {
	cost(reactions)
}

type reagent struct {
	n        *node
	required int
}

type node struct {
	name     string
	produced int
	outputs  []reagent
	required int
}

func cost(s string) int {
	ore := parse(s)
	fmt.Println("topSorted")
	var sorted []*node
	topSort(ore, &sorted, make(map[string]bool))
	if sorted[0].name != "FUEL" {
		log.Fatalf("%s != FUEL", sorted[0].name)
	}
	sorted[0].required = 1

	for _, n := range sorted {
		propagateRequirements(n)
	}
	dot(ore, "", 0, make(map[string]bool))
	return ore.required
}

func propagateRequirements(n *node) {
	fmt.Printf("prop(%s)\n", n.name)
	for _, o := range n.outputs {
		units := divRoundUp(o.n.required, o.n.produced)
		n.required += units * o.required
	}
}

func divRoundUp(a, b int) int {
	n := a / b
	if a%b != 0 {
		n++
	}
	return n
}

func topSort(n *node, sorted *[]*node, visited map[string]bool) {
	if visited[n.name] {
		return
	}
	for _, o := range n.outputs {
		topSort(o.n, sorted, visited)
	}
	visited[n.name] = true
	*sorted = append(*sorted, n)
}

func parse(s string) *node {
	lines := strings.Split(s, "\n")
	nodes := make(map[string]*node)
	ore := &node{
		name:     "ORE",
		produced: 1,
	}
	nodes["ORE"] = ore

	for _, line := range lines {
		line = strings.TrimSpace(line)
		if len(line) == 0 {
			continue
		}
		chunks := strings.Split(line, "=>")
		srcs, dst := chunks[0], chunks[1]
		dstName, dstAmount := unpack(dst)
		dstNode := findOrAdd(dstName, nodes)
		if dstNode.produced != 0 {
			log.Fatalf("xx %v %v", dstName, dstAmount)
		}
		dstNode.produced = dstAmount
		for _, src := range strings.Split(srcs, ",") {
			srcName, srcAmount := unpack(src)

			srcNode := findOrAdd(srcName, nodes)
			srcNode.outputs = append(srcNode.outputs, reagent{
				n:        dstNode,
				required: srcAmount,
			})
		}
	}
	return ore
}

func dot(n *node, src string, required int, visited map[string]bool) {
	if !visited[n.name] {
		fmt.Printf(`%s [label="%s\n%d,%d"];%s`, n.name, n.name, n.produced, n.required, "\n")
	}
	if src != "" {
		fmt.Printf(`%s -> %s [label="%d"]%s`, src, n.name, required, "\n")
	}
	if !visited[n.name] {
		visited[n.name] = true
		for _, o := range n.outputs {
			dot(o.n, n.name, o.required, visited)
		}
	}
}

func findOrAdd(name string, nodes map[string]*node) *node {
	n, ok := nodes[name]
	if !ok {
		n = &node{name: name}
		nodes[name] = n
	}
	return n
}
func unpack(s string) (string, int) {
	chunks := strings.Fields(s)
	return chunks[1], atoi(chunks[0])
}

func atoi(s string) int {
	n, err := strconv.Atoi(s)
	if err != nil {
		log.Fatal(err)
	}
	return n
}

var reactions1 = `
10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL
`

var reactions = `
3 LMPDB, 11 CBTKP => 7 PZDPS
5 CBKW, 4 CXBH => 9 KXNDF
1 LVDN, 4 HGDHV => 1 PCXS
11 PCXS => 2 XTBRS
5 RVSF => 7 TDCH
1 CXBH, 6 PXVN => 8 GQXV
3 DBCB, 3 QLNK => 4 CTFCD
7 PZDPS, 18 HGDHV, 9 TBKM => 4 JHVL
10 QGSV, 1 DBCB, 7 LTHFX => 3 BLRSQ
12 CBTKP, 7 SPBF => 5 KSQL
1 QXHDQ, 5 MQKH, 10 XRCB, 30 SQWHX, 2 PQZVD, 30 TFST, 39 JPFC, 1 FDGS, 17 LVDN => 1 FUEL
2 TBKM => 8 PFHKT
13 CBTKP => 5 QLNK
12 TVRDM, 6 QGSV, 16 LMPDB => 4 PQZVD
7 TDCH, 17 PXVN, 4 ZLKZ => 6 XRCB
1 QBJQ, 26 CBKW => 4 RVSF
24 KXNDF, 3 BLRSQ => 9 GSHKQ
12 BLRSQ, 3 HGDHV => 9 RQNGQ
2 RFBK, 2 WHWS => 8 CBKW
1 WHWS => 7 LTHFX
13 CKQLD, 10 ZLKZ, 2 GQXV => 8 TVHC
1 DBCB => 2 JZXKW
8 SPBF => 7 CXBH
11 LTHFX, 1 PTGLG, 10 NCQTM => 6 SQWHX
16 PFHKT => 3 HGDHV
3 LVDN, 5 PZDPS, 1 SPBF => 9 CQBCL
19 BLRSQ, 1 BLQRD, 5 GSHKQ, 2 LVDN, 3 LMPDB, 5 KTJR => 1 QXHDQ
1 RFBK, 1 JPFC => 7 PXVN
110 ORE => 3 MQKH
1 FPBRB, 7 MQKH => 7 SDJBT
128 ORE => 7 FPBRB
3 WRWGP => 2 RFBK
1 PFHKT, 4 SPBF => 7 JPFC
14 LTHFX, 2 JZXKW, 2 BLRSQ, 2 MHVJP, 6 RQNGQ, 1 CQBCL, 8 TDCH, 2 NJTR => 2 FDGS
4 SDJBT, 2 LMPDB => 8 PLGS
1 RFBK, 1 TBKM => 6 CBTKP
17 LVDN, 2 CBTKP => 4 QGSV
7 WRWGP => 9 LMPDB
3 CKQLD => 6 WHWS
14 CBTKP, 9 XTBRS, 9 GSHKQ, 12 GQXV, 20 LTHFX, 1 RQNGQ, 1 KTJR, 3 BLRSQ => 7 TFST
1 QPCQ => 5 BLQRD
6 QGSV, 1 HGDHV, 1 JPFC => 1 NJTR
1 HGDHV, 7 JHVL, 5 PZDPS => 9 MGRT
1 KSQL => 5 QBJQ
2 QLNK => 2 CKQLD
13 JZXKW, 14 XTBRS => 3 PTGLG
1 BNPXT, 2 PLGS => 7 DBCB
1 RFBK, 9 CTFCD => 1 MHVJP
1 NJTR, 1 TVHC, 2 PCXS => 1 KTJR
2 WRWGP => 6 TBKM
12 QLNK, 1 NJTR => 3 NCQTM
13 ZHCKP, 2 DBCB, 5 PXVN => 9 QPCQ
125 ORE => 3 WRWGP
6 CBTKP, 9 TBKM => 9 SPBF
1 GQXV => 6 ZHCKP
1 MGRT => 8 BNPXT
2 SPBF => 4 ZLKZ
9 TVHC, 5 KXNDF, 3 QPCQ => 3 TVRDM
1 PLGS, 7 TBKM => 8 LVDN
`
