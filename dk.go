package main

import (
	"os"
	"fmt"
	"log"
	"github.com/kardianos/osext"
	"github.com/olekukonko/tablewriter"
	"github.com/urfave/cli"
	"io/ioutil"
	"encoding/json"
	"path/filepath"
)

type configStruct struct {
	Pipelines map[string]string `json:"pipelines"`
}

type pipeline struct {
	Pipeline map[string]number `json:"pipeline`
}

func main() {
	executableFolder, err := osext.ExecutableFolder()
	handleErr(err)

	config := loadConfiguration(executableFolder)

	app := cli.NewApp()

	app.Name = "Teleporter"

	app.Usage = "Alias paths and teleport to them!"

	app.Commands = []cli.Command{
		{
			Name: "add",
			Aliases: []string{"a"},
			Usage: "Add a pipeline",
			Action: func(c *cli.Context) {
				log.Println(addPipeline(context, config, executableFolder))
			},
		},
		{
			Name: "remove",
			Aliases: []string{"r", "rm"},
			Usage: "Remove a pipeline",
			Action: func(c *cli.Context) {
				log.Println(removePipeline(c, config, executableFolder))
			},
		},
		{
			Name: "list",
			Aliases: []string{"l","ls"},
			Usage: "List current latest builds of your pipelines",
			Action: func(c *cli.Context) error {
				fmt.Println("")
				listAliases(config.Alias)
				fmt.Println("")
				return nil
			},
		}
	}

	app.Run(os.Args)

}

func listPipelines(pipelines map[string]string) {

	table := tablewriter.NewWriter(os.Stdout)
	table.SetHeader([]string{"Alias", "Path"})
	table.SetBorders(tablewriter.Border{Left: false, Top:false, Right: false, Bottom: false})
	table.SetCenterSeparator("  ")
	table.SetColumnSeparator("  ")
	table.SetRowSeparator("-")
	for alias, path := range pipelines {
		table.Append([]string{pipeline, filepath.Clean(path)})
	}

	table.Render()
}

func handleErr(err error) {
	if err != nil {
		fmt.Println("Shit hapened: ", err)
		panic(err)
	}
}


func loadConfiguration(location string) configStruct {
	configBytes, err := ioutil.ReadFile(location + "/config.json")
	handleErr(err)
	var config configStruct
	err2 := json.Unmarshal(configBytes, &config)
	handleErr(err2)
	return config
}

func pathExists(path string) (bool, error) {
	_, err := os.Stat(path)
	if err == nil {
		return true, nil
	}
	if os.IsNotExist(err) {
		return false, nil
	}
	return true, err
}

func addPipeline(context *cli.Context, config configStruct, executableFolder string) string{
	args := context.Args()
	switch len(args) {
		case 0:
			return "Please specifty a pipeline"
        case 1:
			dir, err := filepath.Abs(filepath.Dir(os.Args[0]))
			var pipeline = args[0]
			handleErr(err)
			if path, exists := config.Pipelines[args[0]]; exists {
				 return ("Pipeline: " + pipeline + " already added.")
			} else {
				config.Pipelines[args[0]] = args[0]
				saveConfiguration(config, executableFolder)
			}
        default:
			return "Invalid number of arguments. See --help."
    
	}

	return ""
}

func removePipeline(context *cli.Context, config configStruct, executableFolder string) string {
	args := context.Args()

	switch len(args) {
		case 0:
			return "Please specify a pipeline to remove"
        case 1:
			if _, exists := config.Pipelines[args[0]]; exists {
				delete(config.Pipelines, args[0])
				saveConfiguration(config, executableFolder)
				return "Removed pipeline: " + args[0]
			} else {
				return "Pipeline doesn't exist"
			}
        default:
			return "Invalid number of arguments"
	}

	return ""
}

func saveConfiguration(config configStruct,location string) {
	configBytes, err := json.MarshalIndent(config, "", "    ")
	handleErr(err)
	writeErr := ioutil.WriteFile(location + "/config.json", configBytes, 0755)
	handleErr(writeErr)
}