// Calorie counting for the elves.
// Input: calories each elf is carrying
//
// Eleves separate their inventories by a blank line.
//
// Find the Elf carrying the most Calories. How many total Calories is that Elf carrying?
package main

import (
	"bufio"
	"flag"
	"fmt"
	"io"
	"os"
	"strconv"
)

var (
	input = flag.String("input", "./input.txt", "The file to use as input.")
)

func findMostCalories(r io.Reader) (int, error) {
	max := 0

	scanner := bufio.NewScanner(r)
	sum := 0
	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			if sum > max {
				max = sum
			}

			sum = 0
			continue
		}

		i, err := strconv.Atoi(line)
		if err != nil {
			// Not a number, just move to the next line.
			continue
		}

		sum += i
	}

	if err := scanner.Err(); err != nil {
		return 0, err
	}

	return max, nil
}

func main() {
	flag.Parse()

	f, err := os.Open(*input)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
		os.Exit(1)
	}
	defer f.Close()

	cals, err := findMostCalories(f)
	if err != nil {
		fmt.Fprintf(os.Stderr, "%v", err)
		os.Exit(1)
	}

	fmt.Printf("The most calories carried by an elf is %d\n", cals)
}
