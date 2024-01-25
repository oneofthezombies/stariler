package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "dev",
	Short: "dev cli tools for Stariler",
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	// Here you will define your flags and configuration settings.
	// Cobra supports persistent flags, which, if defined here,
	// will be global for your application.

	// rootCmd.PersistentFlags().StringVar(&cfgFile, "config", "", "config file (default is $HOME/.dev.yaml)")

	// Cobra also supports local flags, which will only run
	// when this action is called directly.
	rootCmd.Flags().BoolP("toggle", "t", false, "Help message for toggle")
}

/**
package main

import (
	"context"
	"flag"
	"fmt"
	"os"
	"os/exec"
)

const (
	typescriptPath = "third_party/typescript"
)

func main() {
	initFlag := flag.Bool("init", false, "initialize development environment")
	flag.Parse()

	switch {
	case *initFlag:
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
	default:
		flag.Usage()
		os.Exit(1)
	}
}
*/
