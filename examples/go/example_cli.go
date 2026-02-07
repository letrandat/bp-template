package main

import (
	"encoding/json"
	"flag"
	"fmt"
	"os"
)

type output struct {
	Ok      bool `json:"ok"`
	Version int  `json:"version"`
	Seed    int  `json:"seed"`
	Data    struct {
		Hello string `json:"hello"`
	} `json:"data"`
}

func main() {
	jsonFlag := flag.Bool("json", false, "emit JSON to stdout")
	_ = flag.Bool("stable", false, "no-op; output already deterministic via struct fields")
	seed := flag.Int("seed", 0, "seed for deterministic behavior")
	flag.Parse()

	if !*jsonFlag {
		fmt.Fprintln(os.Stderr, "example_cli.go: missing --json")
		os.Exit(2)
	}

	fmt.Fprintf(os.Stderr, "example_cli.go: seed=%d\n", *seed)

	var out output
	out.Ok = true
	out.Version = 1
	out.Seed = *seed
	out.Data.Hello = "world"

	b, err := json.MarshalIndent(out, "", "  ")
	if err != nil {
		fmt.Fprintf(os.Stderr, "example_cli.go: json marshal failed: %v\n", err)
		os.Exit(1)
	}
	fmt.Println(string(b))
}
