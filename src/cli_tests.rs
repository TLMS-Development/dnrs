use std::process::Command;

#[test]
fn test_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "--help"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("dnrs"));
    assert!(stdout.contains("Usage:"));
}

#[test]
fn test_generate_config_help() {
    let output = Command::new("cargo")
        .args(&["run", "--", "generate-config", "--help"])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Generate configuration directory structure"));
}

#[test]
fn test_generate_config_execution() {
    let temp_dir = std::env::temp_dir().join("dnrs_test_config");
    if temp_dir.exists() {
        std::fs::remove_dir_all(&temp_dir).unwrap();
    }

    let output = Command::new("cargo")
        .args(&[
            "run",
            "--",
            "generate-config",
            "--output",
            temp_dir.to_str().unwrap(),
        ])
        .output()
        .expect("failed to execute process");

    assert!(output.status.success());
    assert!(temp_dir.exists());
    assert!(temp_dir.join("resolver.yaml").exists());
    assert!(temp_dir.join("providers").exists());
    assert!(temp_dir.join("dns").exists());

    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
}
