use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use std::path::PathBuf;
use std::env;

#[derive(Debug, Deserialize)]
struct ScriptConfig {
    name: String,
    #[serde(rename = "type")]
    script_type: String,
    script_path: String,
    #[serde(default)]
    python_deps: Vec<String>,
    #[serde(default)]
    ps_modules: Vec<String>,
    #[serde(default)]
    working_dir: Option<String>,
    #[serde(default)]
    args: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExecutorConfig {
    #[serde(default = "default_python_path")]
    python_path: String,
    python_env: Option<String>,
    #[serde(default = "default_powershell_path")]
    powershell_path: String,
    scripts: Vec<ScriptConfig>,
}

fn default_python_path() -> String {
    "python3".to_string()
}

fn default_powershell_path() -> String {
    "pwsh".to_string()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <config.json> [script_name] [args...]", args[0]);
        std::process::exit(1);
    }

    let config_path = &args[1];
    let config_str = std::fs::read_to_string(config_path)?;
    let config: ExecutorConfig = serde_json::from_str(&config_str)?;

    // Om script_name anges, kör bara det scriptet
    if args.len() >= 3 {
        let script_name = &args[2];
        let script_args: Vec<String> = args[3..].to_vec();
        
        run_script(&config, script_name, &script_args)?;
        return Ok(());
    }

    // Annars kör alla scripts i ordning
    for script in &config.scripts {
        println!("Running script: {}", script.name);
        execute_script(&config, script, &[])?;
    }

    Ok(())
}

fn run_script(config: &ExecutorConfig, script_name: &str, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    for script in &config.scripts {
        if script.name == script_name {
            return execute_script(config, script, args);
        }
    }
    Err(format!("Script '{}' not found", script_name).into())
}

fn execute_script(config: &ExecutorConfig, script: &ScriptConfig, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let working_dir = script.working_dir.as_ref()
        .map(|d| PathBuf::from(d))
        .unwrap_or_else(|| {
            PathBuf::from(&script.script_path)
                .parent()
                .unwrap_or(PathBuf::from(".").as_path())
                .to_path_buf()
        });

    let mut cmd = match script.script_type.to_lowercase().as_str() {
        "python" => build_python_command(config, script, args, &working_dir)?,
        "powershell" => build_powershell_command(config, script, args, &working_dir)?,
        _ => return Err(format!("Unknown script type: {}", script.script_type).into()),
    };

    cmd.current_dir(&working_dir);
    cmd.stdin(Stdio::inherit());
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd.status()?;
    if !status.success() {
        return Err(format!("Script '{}' failed with exit code: {:?}", script.name, status.code()).into());
    }

    Ok(())
}

fn build_python_command(
    config: &ExecutorConfig,
    script: &ScriptConfig,
    args: &[String],
    working_dir: &PathBuf,
) -> Result<Command, Box<dyn std::error::Error>> {
    let python_exe = if let Some(env_path) = &config.python_env {
        let venv_path = PathBuf::from(env_path);
        
        // Windows: Scripts\python.exe
        let win_python = venv_path.join("Scripts").join("python.exe");
        if win_python.exists() {
            win_python
        } else {
            // Linux/Mac: bin/python
            let unix_python = venv_path.join("bin").join("python");
            if unix_python.exists() {
                unix_python
            } else {
                PathBuf::from(&config.python_path)
            }
        }
    } else {
        PathBuf::from(&config.python_path)
    };

    let mut cmd = Command::new(&python_exe);
    cmd.arg(&script.script_path);
    
    for arg in &script.args {
        cmd.arg(arg);
    }
    
    for arg in args {
        cmd.arg(arg);
    }

    Ok(cmd)
}

fn build_powershell_command(
    config: &ExecutorConfig,
    script: &ScriptConfig,
    args: &[String],
    _working_dir: &PathBuf,
) -> Result<Command, Box<dyn std::error::Error>> {
    let mut cmd = Command::new(&config.powershell_path);
    cmd.arg("-File");
    cmd.arg(&script.script_path);
    
    for arg in &script.args {
        cmd.arg(arg);
    }
    
    for arg in args {
        cmd.arg(arg);
    }

    Ok(cmd)
}

