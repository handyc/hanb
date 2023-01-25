use std::fs::read_to_string;

use assert_cmd::Command;

#[test]
fn test_example() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hanb")?;
    let output = read_to_string("tests/example_output.txt")?;
    let assert = cmd.arg("-f").arg("examples/demo.hanb").assert();
    assert.success().stdout(output);
    Ok(())
}
