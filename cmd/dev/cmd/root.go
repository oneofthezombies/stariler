package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "dev",
	Short: "CLI tool for Stariler development",
}

func Execute() error {
	return rootCmd.Execute()
}
