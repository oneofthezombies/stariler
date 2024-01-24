package main

import (
	"context"
	"fmt"
	"os"
	"os/exec"
)

const (
	typescriptPath = "third_party/typescript"
)

func main() {
	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	if _, err := os.Stat(typescriptPath); err == nil {
		fmt.Printf("%s already exists. deleting...\n", typescriptPath)
		if err := os.RemoveAll(typescriptPath); err != nil {
			panic(err)
		}
	}

	cmd := exec.CommandContext(ctx, "git", "clone", "--depth", "1", "--branch", "v5.3.3", "https://github.com/oneofthezombies/TypeScript.git", typescriptPath)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	if err := cmd.Run(); err != nil {
		panic(err)
	}
}
