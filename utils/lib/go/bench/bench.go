package bench

import (
	"fmt"
	"os"
	"runtime"
	"strings"
	"time"
)

type Day interface {
	Part1() string
	Part2() string
}

type Config struct {
	Filename      string
	DataDirectory string
	ReadData      func(string) Day
	Day           Day
	Part1Solution string
	Part2Solution string
}

func (c Config) Run() {
	resultsFile, err := os.OpenFile(c.Filename, os.O_CREATE|os.O_WRONLY|os.O_TRUNC, 0666)
	if err != nil {
		panic(err)
	}
	defer resultsFile.Close()

	_, err = resultsFile.WriteString("Part,Run,Elapsed (ns),Max. memory (B), Num. allocations\n")
	if err != nil {
		panic(err)
	}

	data := c.ReadData(c.DataDirectory)

	parts := []struct {
		label    string
		toRun    func()
		printout string
	}{
		{
			label: "read",
			toRun: func() {
				data := c.ReadData(c.DataDirectory)
				if data == nil {
					panic("nil data")
				}
			},
			printout: "Parse:\n\t%s\n",
		},
		{
			label: "part 1",
			toRun: func() {
				if data.Part1() != c.Part1Solution {
					panic("part 1 incorrect")
				}
			},
			printout: "Part 1:\n\t%s\n",
		},
		{
			label: "part 2",
			toRun: func() {
				if data.Part2() != c.Part2Solution {
					panic("part 2 incorrect")
				}
			},
			printout: "Part 2:\n\t%s\n",
		},
		{
			label: "total",
			toRun: func() {
				data := c.ReadData(c.DataDirectory)
				if data == nil {
					panic("nil data")
				}
				if data.Part1() != c.Part1Solution {
					panic("part 1 incorrect")
				}
				if data.Part2() != c.Part2Solution {
					panic("part 2 incorrect")
				}
			},
			printout: "Total:\n\t%s\n",
		},
	}

	for _, part := range parts {
		results := runBench(part.label, part.toRun)

		_, err = resultsFile.Write(results.toCSV())
		if err != nil {
			panic(err)
		}

		fmt.Printf(part.printout, processResults(results))
	}
}

type result struct {
	elapsed  time.Duration
	maxAlloc uint64
	numAlloc uint64
}

type benchResults struct {
	results []result
	part    string
}

func (b *benchResults) add(res result) {
	b.results = append(b.results, res)
}

func (b *benchResults) means() (time.Duration, uint64, uint64) {
	dur := int64(0)
	mem := uint64(0)
	allocs := uint64(0)

	for _, result := range b.results {
		dur += result.elapsed.Nanoseconds()
		mem += result.maxAlloc
		allocs += result.numAlloc
	}

	return time.Duration(dur / int64(len(b.results))), mem / uint64(len(b.results)), allocs / uint64(len(b.results))
}

func (b *benchResults) toCSV() []byte {
	var out strings.Builder
	out.Grow(128 * len(b.results))

	for run, result := range b.results {
		out.WriteString(fmt.Sprintf("%s,%d,%d,%d,%d\n", b.part, run+1, result.elapsed.Nanoseconds(), result.maxAlloc, result.numAlloc))
	}

	return []byte(out.String())
}

const (
	runtimeLimit  = 30 * time.Second
	runtimeTarget = time.Millisecond
	runLimit      = 10240
	minRuns       = 100
)

func runBench(part string, function func()) benchResults {
	out := benchResults{
		results: make([]result, 0, runLimit),
		part:    part,
	}

	totalTime := time.Duration(0)
	totalRuns := 0

	n := uint64(1)
	for totalRuns < minRuns || (totalTime < runtimeLimit && totalRuns < runLimit) {
		totalRuns++
		var start time.Time
		var elapsed time.Duration
		var baseline, final runtime.MemStats
		var i uint64
		for {
			runtime.ReadMemStats(&baseline)
			start = time.Now()
			for i = 0; i < n; i++ {
				function()
			}
			elapsed = time.Since(start)
			runtime.ReadMemStats(&final)
			if elapsed >= runtimeTarget {
				break
			}
			multiplier := uint64(runtimeTarget / elapsed)
			if multiplier < 2 {
				multiplier = 2
			}
			n *= multiplier
		}
		maxAlloc := (final.TotalAlloc - baseline.TotalAlloc) / n
		numAlloc := (final.Mallocs - baseline.Mallocs) / n
		out.add(result{elapsed / time.Duration(n), maxAlloc, numAlloc})
	}

	return out
}

func processResults(res benchResults) string {
	meanTime, maxMem, numMem := res.means()

	return fmt.Sprintf("Took: %s\tMax memory: %d\t# allocations: %d",
		meanTime, maxMem, numMem)
}
