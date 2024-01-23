package main

import (
	"fmt"
	"os"
	"path/filepath"
)

func main() {
	fmt.Println("Hello, World!")

	cwd, err := os.Getwd()
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}
	fmt.Println("Current working directory:", cwd)

	tsconfigPath := filepath.Join(cwd, "tsconfig.json")
	fmt.Println("tsconfig.json path:", tsconfigPath)

	if _, err := os.Stat(tsconfigPath); os.IsNotExist(err) {
		fmt.Println("tsconfig.json does not exist")
		os.Exit(1)
	}
}
