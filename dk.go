package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"log"
	"net/http"
	"os"
	"strconv"
	"time"

	"github.com/kardianos/osext"
	"github.com/olekukonko/tablewriter"
	"github.com/urfave/cli"
)

type configStruct struct {
	Pipelines map[string]string `json:"pipelines"`
}

type pipelineStatus struct {
	build string
}

type baseBuild struct {
	Number int    `json:"number"`
	Branch string `json:"branch"`
}

const baseBuildkiteURL string = "https://api.buildkite.com/v2/organizations/"

func main() {
	executableFolder, err := osext.ExecutableFolder()
	handleErr(err)

	buildkiteToken := os.Getenv("BUILDKITE_TOKEN")

	if len(buildkiteToken) < 1 {
		fmt.Println("Missing BUILDKITE_TOKEN!")
		return
	}

	config := loadConfiguration(executableFolder)

	app := cli.NewApp()

	app.Name = "Deploy kite cli"

	app.Usage = "View latest builds and deploy them"

	app.Commands = []cli.Command{
		{
			Name:    "add",
			Aliases: []string{"a"},
			Usage:   "Add a pipeline",
			Action: func(context *cli.Context) {
				log.Println(addPipeline(context, config, executableFolder, buildkiteToken))
			},
		},
		{
			Name:    "remove",
			Aliases: []string{"r", "rm"},
			Usage:   "Remove a pipeline",
			Action: func(c *cli.Context) {
				log.Println(removePipeline(c, config, executableFolder))
			},
		},
		{
			Name:    "list",
			Aliases: []string{"l", "ls"},
			Usage:   "List current latest builds of your pipelines",
			Action: func(c *cli.Context) error {
				fmt.Println("")
				listPipelines(config.Pipelines, buildkiteToken)
				fmt.Println("")
				return nil
			},
		},
	}

	app.Run(os.Args)

}

func findLatestBuild(pipeline string, buildkiteToken string) int {
	client := &http.Client{Timeout: 10 * time.Second}
	uri := baseBuildkiteURL + "/siteminder/pipelines/" + pipeline + "/builds"
	req, err := http.NewRequest("GET", uri, nil)
	handleErr(err)
	req.Header.Add("Authorization", "Bearer "+buildkiteToken)
	resp, err := client.Do(req)
	handleErr(err)

	defer resp.Body.Close()

	body, readErr := ioutil.ReadAll(resp.Body)
	handleErr(readErr)
	var builds []*json.RawMessage
	jsonErr := json.Unmarshal(body, &builds)
	handleErr(jsonErr)

	build := builds[0]

	var buildNumber baseBuild

	jsonErr2 := json.Unmarshal(*build, &buildNumber)
	handleErr(jsonErr2)

	return buildNumber.Number
}

func checkExistingPipeline(pipeline string, buildkiteToken string) bool {
	client := &http.Client{Timeout: 10 * time.Second}
	uri := baseBuildkiteURL + "/siteminder/pipelines/" + pipeline
	req, err := http.NewRequest("GET", uri, nil)
	handleErr(err)
	req.Header.Add("Authorization", "Bearer "+buildkiteToken)
	resp, err := client.Do(req)
	handleErr(err)

	defer resp.Body.Close()

	body, readErr := ioutil.ReadAll(resp.Body)
	handleErr(readErr)
	var builds interface{}
	jsonErr := json.Unmarshal(body, &builds)
	handleErr(jsonErr)
	existingPipeline := builds.(map[string]interface{})

	if _, exists := existingPipeline["id"]; exists {
		return true
	}

	return false
}

func listPipelines(pipelines map[string]string, buildkiteToken string) {

	table := tablewriter.NewWriter(os.Stdout)
	table.SetHeader([]string{"Pipeline", "Latest build"})
	table.SetBorders(tablewriter.Border{Left: false, Top: false, Right: false, Bottom: false})
	table.SetCenterSeparator("  ")
	table.SetColumnSeparator("  ")
	table.SetRowSeparator("-")
	for pipeline := range pipelines {
		build := findLatestBuild(pipeline, buildkiteToken)

		table.Append([]string{pipeline, strconv.Itoa(build)})
	}

	table.Render()
}

func handleErr(err error) {
	if err != nil {
		fmt.Println("Shit hapened: ", err)
		panic(err)
	}
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

func addPipeline(context *cli.Context, config configStruct, executableFolder string, buildkiteToken string) string {
	args := context.Args()
	switch len(args) {
	case 0:
		return "Please specifty a pipeline"
	case 1:
		if _, exists := config.Pipelines[args[0]]; exists {
			return ("Pipeline: " + args[0] + " already added.")
		}
		if !checkExistingPipeline(args[0], buildkiteToken) {
			return ("Pipeline: " + args[0] + " doesn't exist.")
		}

		config.Pipelines[args[0]] = ""
		saveConfiguration(config, executableFolder)
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
		}
		return "Pipeline doesn't exist"
	default:
		return "Invalid number of arguments"
	}
}

func saveConfiguration(config configStruct, location string) {
	fmt.Println(config)
	configBytes, err := json.MarshalIndent(config, "", "    ")
	handleErr(err)
	writeErr := ioutil.WriteFile(location+"/config.json", configBytes, 0755)
	handleErr(writeErr)
}

func loadConfiguration(location string) configStruct {
	configBytes, err := ioutil.ReadFile(location + "/config.json")
	handleErr(err)
	var config configStruct
	err2 := json.Unmarshal(configBytes, &config)
	handleErr(err2)
	return config
}
