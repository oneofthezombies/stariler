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

		if err := installTypeScript(ctx); err != nil {
			return err
		}

		if err := installReferenceProject(ctx); err != nil {
			return err
		}

		return nil
	},
}

func init() {
	rootCmd.AddCommand(initCmd)
}

func installTypeScript(ctx context.Context) error {
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
}

func installReferenceProject(ctx context.Context) error {
	const (
		referenceProjectPath = "third_party/reference_project"
	)

	proc := exec.CommandContext(ctx, "npm", "install")
	proc.Dir = referenceProjectPath
	proc.Stdout = os.Stdout
	proc.Stderr = os.Stderr
	return proc.Run()
}
