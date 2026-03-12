use std::env;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn fix_icon(path: &str, dry_run: bool) -> Result<(), String> {
    println!("Processing: {path:?}");

    let path = Path::new(path);

    // Check file extension
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("url") => {}
        _ => return Err("File is not a .url file.".into()),
    }

    // Read and parse the .url file
    let contents = fs::read_to_string(path).map_err(|e| format!("Failed to read file: {e}"))?;

    let mut url = None;
    let mut icon_file = None;

    for line in contents.lines() {
        if let Some((key, value)) = line.split_once('=') {
            match key.trim() {
                "URL" => url = Some(value.trim().to_string()),
                "IconFile" => icon_file = Some(value.trim().to_string()),
                _ => {}
            }
        }
    }

    // Extract game ID from URL
    let url = url.ok_or("No URL found in file.")?;
    let gameid = url
        .strip_prefix("steam://rungameid/")
        .ok_or_else(|| format!("Invalid URL: {url:?}"))?;

    // Get icon file path
    let icon_path_str = icon_file.ok_or("No IconFile found in file.")?;
    let icon_path = Path::new(&icon_path_str);

    if icon_path.exists() {
        return Err(format!("Icon file already exists: {icon_path_str:?}"));
    }

    // Extract icon filename
    let icon_name = icon_path
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| format!("Invalid icon file path: {icon_path_str:?}"))?;

    // Build the download URL
    let icon_url = format!(
        "https://cdn.cloudflare.steamstatic.com/steamcommunity/public/images/apps/{gameid}/{icon_name}"
    );

    if dry_run {
        println!("Would download: {icon_url:?}");
        println!("Would save to:  {icon_path_str:?}");
        return Ok(());
    }

    println!("Downloading icon: {icon_url:?}");

    let response = ureq::get(&icon_url)
        .call()
        .map_err(|e| format!("Download failed: {e}"))?;

    // Create parent directories if needed
    if let Some(parent) = icon_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {e}"))?;
        }
    }

    let body = response
        .into_body()
        .read_to_vec()
        .map_err(|e| format!("Failed to read response: {e}"))?;
    fs::write(icon_path, &body).map_err(|e| format!("Failed to write icon file: {e}"))?;

    println!("Saved to: {icon_path_str:?}");
    Ok(())
}

fn main() -> ExitCode {
    let raw_args: Vec<String> = env::args().skip(1).collect();

    let dry_run = raw_args.iter().any(|a| a == "--dry-run");
    let patterns: Vec<&str> = raw_args
        .iter()
        .filter(|a| *a != "--dry-run")
        .map(|s| s.as_str())
        .collect();

    if patterns.is_empty() {
        eprintln!("Usage: fix-steam-blank-icon-rs [--dry-run] <path1.url> [path2.url ...]");
        eprintln!("  Supports glob patterns, e.g. *.url or C:\\Users\\**\\*.url");
        return ExitCode::FAILURE;
    }

    // Expand globs
    let mut paths: Vec<String> = Vec::new();
    for pattern in &patterns {
        match glob::glob(pattern) {
            Ok(entries) => {
                let mut matched = false;
                for entry in entries {
                    match entry {
                        Ok(path) => {
                            matched = true;
                            paths.push(path.to_string_lossy().into_owned());
                        }
                        Err(e) => eprintln!("Glob error: {e}"),
                    }
                }
                if !matched {
                    eprintln!("No files matched: {pattern:?}");
                }
            }
            Err(e) => {
                eprintln!("Invalid pattern {pattern:?}: {e}");
            }
        }
    }

    if paths.is_empty() {
        eprintln!("No files to process.");
        return ExitCode::FAILURE;
    }

    if dry_run {
        println!("Dry run mode — no files will be downloaded.\n");
    }

    let mut had_errors = false;

    for path in &paths {
        if let Err(e) = fix_icon(path, dry_run) {
            eprintln!("Error ({path}): {e}");
            had_errors = true;
        }
    }

    println!("All done.");

    if had_errors {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
