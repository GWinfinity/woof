use std::process::Command;
use tempfile::TempDir;
use std::fs;

fn woof_bin() -> String {
    // Use the debug binary during tests
    concat!(env!("CARGO_MANIFEST_DIR"), "/target/debug/woof").to_string()
}

#[test]
fn test_check_empty_block() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.go");
    fs::write(&file, r#"package main
func main() {
    if true {
    }
}
"#).unwrap();

    let output = Command::new(woof_bin())
        .args(["check", file.to_str().unwrap()])
        .output()
        .expect("Failed to run woof");

    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stderr.contains("warning") || stdout.contains("warning"),
        "Should detect empty block"
    );
}

#[test]
fn test_format_stdout() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.go");
    // Test basic formatting - the formatter should at least produce valid output
    fs::write(&file, r#"package main

func main() {
x := 1
}
"#).unwrap();

    let output = Command::new(woof_bin())
        .args(["format", file.to_str().unwrap(), "--stdout"])
        .output()
        .expect("Failed to run woof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // The formatter should produce output that still contains the function
    assert!(
        stdout.contains("func main()"),
        "Should contain the main function: {}", stdout
    );
    assert!(
        stdout.contains("package main"),
        "Should format package declaration: {}", stdout
    );
}

#[test]
fn test_rules_list() {
    let output = Command::new(woof_bin())
        .arg("rules")
        .output()
        .expect("Failed to run woof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("E001") && stdout.contains("unused-import"),
        "Should list E001 rule: {}", stdout
    );
}

#[test]
fn test_init_config() {
    let temp = TempDir::new().unwrap();
    let config_file = temp.path().join("woof.toml");
    
    let output = Command::new(woof_bin())
        .arg("init")
        .current_dir(&temp)
        .output()
        .expect("Failed to run woof");

    assert!(
        config_file.exists(),
        "Config file should be created: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
    
    let content = fs::read_to_string(&config_file).unwrap();
    assert!(content.contains("[global]"));
    assert!(content.contains("[linter]"));
    assert!(content.contains("[formatter]"));
}

#[test]
fn test_check_with_statistics() {
    let temp = TempDir::new().unwrap();
    let file = temp.path().join("test.go");
    fs::write(&file, r#"package main
func main() {
    if true {
    }
}
"#).unwrap();

    let output = Command::new(woof_bin())
        .args(["check", file.to_str().unwrap(), "--statistics"])
        .output()
        .expect("Failed to run woof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    assert!(
        stdout.contains("Statistics:") && stdout.contains("Files checked:"),
        "Should show statistics: {}", stdout
    );
}

#[test]
fn test_version() {
    let output = Command::new(woof_bin())
        .arg("--version")
        .output()
        .expect("Failed to run woof");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("woof"));
}
