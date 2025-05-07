use std::process::Command;

pub fn clone(url: &str) -> Result<(), String> {
    let mut git: Command = Command::new("git");
    let git_exists = git.arg("--version").output().is_ok();

    if !git_exists {
        return Err("Git is not installed".to_string());
    }

    git = Command::new("git");

    git.arg("clone").arg(url);
    let output = git.output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Git clone failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }

    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    Ok(())
}
