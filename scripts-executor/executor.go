package main

import (
	"encoding/json"
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"strings"
)

// ScriptConfig definierar ett script och dess dependencies
type ScriptConfig struct {
	Name         string   `json:"name"`
	Type         string   `json:"type"` // "python" eller "powershell"
	ScriptPath   string   `json:"script_path"`
	PythonDeps   []string `json:"python_deps,omitempty"`
	PSModules    []string `json:"ps_modules,omitempty"`
	WorkingDir   string   `json:"working_dir,omitempty"`
	Args         []string `json:"args,omitempty"`
}

// ExecutorConfig konfiguration för executor
type ExecutorConfig struct {
	PythonPath    string        `json:"python_path,omitempty"`
	PythonEnv     string        `json:"python_env,omitempty"` // Path to venv
	PowerShellPath string        `json:"powershell_path,omitempty"`
	Scripts       []ScriptConfig `json:"scripts"`
}

func main() {
	if len(os.Args) < 2 {
		fmt.Fprintf(os.Stderr, "Usage: %s <config.json> [script_name] [args...]\n", os.Args[0])
		os.Exit(1)
	}

	configPath := os.Args[1]
	config, err := loadConfig(configPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error loading config: %v\n", err)
		os.Exit(1)
	}

	// Om script_name anges, kör bara det scriptet
	if len(os.Args) >= 3 {
		scriptName := os.Args[2]
		scriptArgs := os.Args[3:]
		
		err := runScript(config, scriptName, scriptArgs)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error running script: %v\n", err)
			os.Exit(1)
		}
		return
	}

	// Annars kör alla scripts i ordning
	for _, script := range config.Scripts {
		fmt.Printf("Running script: %s\n", script.Name)
		err := executeScript(config, script, nil)
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error running %s: %v\n", script.Name, err)
			os.Exit(1)
		}
	}
}

func loadConfig(path string) (*ExecutorConfig, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, err
	}

	var config ExecutorConfig
	err = json.Unmarshal(data, &config)
	if err != nil {
		return nil, err
	}

	// Set defaults
	if config.PythonPath == "" {
		config.PythonPath = "python3"
	}
	if config.PowerShellPath == "" {
		config.PowerShellPath = "pwsh"
	}

	return &config, nil
}

func runScript(config *ExecutorConfig, scriptName string, args []string) error {
	for _, script := range config.Scripts {
		if script.Name == scriptName {
			return executeScript(config, script, args)
		}
	}
	return fmt.Errorf("script '%s' not found", scriptName)
}

func executeScript(config *ExecutorConfig, script ScriptConfig, args []string) error {
	var cmd *exec.Cmd
	workingDir := script.WorkingDir
	if workingDir == "" {
		workingDir = filepath.Dir(script.ScriptPath)
	}

	switch strings.ToLower(script.Type) {
	case "python":
		cmd = buildPythonCommand(config, script, args, workingDir)
	case "powershell":
		cmd = buildPowerShellCommand(config, script, args, workingDir)
	default:
		return fmt.Errorf("unknown script type: %s", script.Type)
	}

	cmd.Dir = workingDir
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin

	return cmd.Run()
}

func buildPythonCommand(config *ExecutorConfig, script ScriptConfig, args []string, workingDir string) *exec.Cmd {
	var pythonExe string
	
	// Använd virtualenv om specificerad
	if config.PythonEnv != "" {
		if _, err := os.Stat(config.PythonEnv); err == nil {
			// Windows: Scripts\python.exe, Linux/Mac: bin/python
			if _, err := os.Stat(filepath.Join(config.PythonEnv, "Scripts", "python.exe")); err == nil {
				pythonExe = filepath.Join(config.PythonEnv, "Scripts", "python.exe")
			} else if _, err := os.Stat(filepath.Join(config.PythonEnv, "bin", "python")); err == nil {
				pythonExe = filepath.Join(config.PythonEnv, "bin", "python")
			}
		}
	}
	
	if pythonExe == "" {
		pythonExe = config.PythonPath
	}

	cmdArgs := []string{script.ScriptPath}
	if len(script.Args) > 0 {
		cmdArgs = append(cmdArgs, script.Args...)
	}
	if len(args) > 0 {
		cmdArgs = append(cmdArgs, args...)
	}

	return exec.Command(pythonExe, cmdArgs...)
}

func buildPowerShellCommand(config *ExecutorConfig, script ScriptConfig, args []string, workingDir string) *exec.Cmd {
	cmdArgs := []string{"-File", script.ScriptPath}
	if len(script.Args) > 0 {
		cmdArgs = append(cmdArgs, script.Args...)
	}
	if len(args) > 0 {
		cmdArgs = append(cmdArgs, args...)
	}

	return exec.Command(config.PowerShellPath, cmdArgs...)
}

