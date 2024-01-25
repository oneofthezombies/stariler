package cmd

import (
	"context"
	"fmt"
	"os"
	"os/exec"
	"os/signal"
	"syscall"

	"github.com/spf13/cobra"
)

var initCmd = &cobra.Command{
	Use:   "init",
	Short: "initialize development environment",
	RunE: func(cmd *cobra.Command, args []string) error {
		ctx, cancel := context.WithCancel(context.Background())
		defer cancel()

		signals := make(chan os.Signal, 1)
		signal.Notify(signals, syscall.SIGINT, syscall.SIGTERM)

		go func() {
			for {
				select {
				case <-ctx.Done():
					return
				case <-signals:
					cancel()
				}
			}
		}()

		const (
			typescriptPath = "third_party/typescript"
		)

		if _, err := os.Stat(typescriptPath); err == nil {
			fmt.Printf("%s already exists. deleting...\n", typescriptPath)
			if err := os.RemoveAll(typescriptPath); err != nil {
				return err
			}
		}

		proc := exec.CommandContext(ctx, "git", "clone", "--depth", "1", "--branch", "v5.3.3", "https://github.com/oneofthezombies/TypeScript.git", typescriptPath)
		proc.Stdout = os.Stdout
		proc.Stderr = os.Stderr
		return proc.Run()
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}
