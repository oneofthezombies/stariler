package cmd

import (
	"github.com/spf13/cobra"
)

var rootCmd = &cobra.Command{
	Use:   "star",
	Short: "Stariler TypeScript compiler",
	RunE: func(cmd *cobra.Command, args []string) error {
		if version, _ := cmd.Flags().GetBool("version"); version {
			return printVersion()
		}
		if help, _ := cmd.Flags().GetBool("help"); help {
			return cmd.Help()
		}
		return cmd.Help()
	},
}

func Execute() error {
	return rootCmd.Execute()
}

func init() {
	rootCmd.PersistentFlags().BoolP("version", "v", false, "Print the compiler's version.")
	rootCmd.PersistentFlags().BoolP("help", "h", false, "Print this message.")
	rootCmd.PersistentFlags().BoolP("build", "b", false, "Build one or more projects and their dependencies, if out of date.")
}

func printVersion() error {
	return nil
}
