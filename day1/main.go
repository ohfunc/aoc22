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
	"sort"
	"strconv"
)

var (
	input = flag.String("input", "./input.txt", "The file to use as input.")
)

func findMostCalories(r io.Reader) (int, error) {
	elfCalories := []int{}

	scanner := bufio.NewScanner(r)
	sum := 0
	for scanner.Scan() {
		err := scanner.Err()
		line := scanner.Text()

		if line == "" || err == io.EOF {
			elfCalories = append(elfCalories, sum)
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

	sort.Ints(elfCalories)

	fmt.Println(elfCalories)

	calories := 0
	for _, c := range elfCalories[:len(elfCalories)-3] {
		fmt.Println(c)
	}

	return calories, nil
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
