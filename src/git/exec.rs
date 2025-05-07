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

pub fn commit(msg: &str, all: bool, push: bool, files: Option<Vec<String>>) -> Result<(), String> {
    let mut git: Command = Command::new("git");
    let git_exists = git.arg("--version").output().is_ok();

    if !git_exists {
        return Err("Git is not installed".to_string());
    }

    git = Command::new("git");

    if all {
        git.arg("add")
            .arg("--all")
            .output()
            .map_err(|e| e.to_string())?;
    } else if files.is_some() {
        for file in files.unwrap() {
            git.arg("add")
                .arg(file)
                .output()
                .map_err(|e| e.to_string())?;
        }
    } else {
        return Err("No files to add".to_string());
    }

    git = Command::new("git");

    git.arg("commit").arg("-m").arg(msg);
    println!("Committing with message: {}", msg);
    let output = git.output().map_err(|e| e.to_string())?;

    if !output.status.success() {
        return Err(format!(
            "Git commit failed: {}",
            String::from_utf8_lossy(&output.stdout)
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

    if push {
        let mut push_cmd = Command::new("git");
        push_cmd.arg("push");
        let output = push_cmd.output().map_err(|e| e.to_string())?;

        if !output.status.success() {
            return Err(format!(
                "Git push failed: {}",
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
    }

    Ok(())
}
