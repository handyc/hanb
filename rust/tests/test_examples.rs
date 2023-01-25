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


#[test]
fn test_serialization() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hanb")?;
    let assert = cmd.arg("-f").arg("examples/demo2.hanb").assert();
    assert.success();
    let output = read_to_string("hanb.hsit")?;
    let example = read_to_string("examples/example.hsit")?;
    assert_eq!(output, example);
    Ok(())
}
