#!/usr/bin/python

def main():
	sif = open('sif.txt').read().strip()
	layers = [sif[n*150:(n+1)*150] for n in range(100)]
	
	for y in range(6):
		line = ''
		for x in range(25):
			line += firstVisible((layers), y*25+x)
		print line

def firstVisible(layers, n):
	for layer in layers:
		p = layer[n]
		if p == '2':
			continue
		elif p == '1':
			return '*'
		else:
			return ' '
	return '_'

if __name__ == '__main__':
	main()
