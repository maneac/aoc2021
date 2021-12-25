package main

import (
    "log"
    "os"
)

type input []string

func main() {
    data := readData()

    log.Println("Part 1: ", part1(data))
    log.Println("Part 2: ", part2(data))
}

func readData() input {
    _, err := os.ReadFile("../../data/day_24.txt")
    if err != nil {
        panic(err)
    }


    panic("unimplemented")
}

func part1(input input) int {
    panic("unimplemented")
}

func part2(input input) int {
    panic("unimplemented")
}
