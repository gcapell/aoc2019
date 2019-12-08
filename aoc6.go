package main

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strings"
)

type Node struct {
	name     string
	children []*Node
	parent   *Node
	height   int
}

func main() {
	f, err := os.Open("input.txt")
	if err != nil {
		log.Fatal(err)
	}
	nodes := make(map[string]*Node)
	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := scanner.Text()
		chunks := strings.Split(line, ")")

		insert(nodes, chunks[0], chunks[1])
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "reading standard input:", err)
	}
	part := 2
	if part == 1 {
		// part 1
		root := nodes["COM"]
		var total int
		treeprint(root, "", 0, &total)
		fmt.Println(total)
	} else {
		for n, c := nodes["YOU"].parent, 0; n != nil; n, c = n.parent, c+1 {
			n.height = c
		}
		for n, c := nodes["SAN"].parent, 0; n != nil; n, c = n.parent, c+1 {
			if n.height != 0 {
				fmt.Println(n.height + c)
				break
			}
		}
	}
}

func insert(m map[string]*Node, parentName, childName string) {
	parent := findOrInsert(m, parentName)
	child := findOrInsert(m, childName)
	parent.children = append(parent.children, child)
	child.parent = parent
}

func findOrInsert(m map[string]*Node, s string) *Node {
	if n, ok := m[s]; ok {
		return n
	}
	n := &Node{name: s}
	m[s] = n
	return n
}

func treeprint(n *Node, ind string, depth int, total *int) {
	fmt.Printf("%s%s\n", ind, n.name)
	*total += depth
	for _, c := range n.children {
		treeprint(c, ind+"  ", depth+1, total)
	}
}

func path(n *Node) []string {
	var reply []string
	for n != nil {
		reply = append(reply, n.name)
		n = n.parent
	}
	return reply
}
