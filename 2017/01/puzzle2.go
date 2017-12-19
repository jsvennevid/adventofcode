package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)

	scanner := bufio.NewScanner(reader)
	for scanner.Scan() {
		text := scanner.Text()

		s := 0
		l := len(text)
		diff := l / 2
		for i := 0; i < l; i++ {
			c := text[i]
			oc := text[(i+diff) % l]
			if c == oc {
				s = s + (int(c) - 48)
			}
		}

		fmt.Println("Input:", text, "Output:", s)
	}
}
