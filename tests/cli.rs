use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs::File;
use std::process::Command;

#[test]
fn rename_part_of_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;

    let file_path = dir.path().join("file-to-rename.txt");
    let file = File::create(&file_path)?;

    let mut cmd = Command::cargo_bin("rnm")?;
    cmd.current_dir(dir.path()).arg("to-rename").arg("renamed");
    cmd.assert().success();

    let expected_file_path = dir.path().join("file-renamed.txt");
    assert!(predicate::path::exists().eval(&expected_file_path));
    assert!(predicate::path::missing().eval(&file_path));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn no_error_when_no_match() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;

    let file_path = dir.path().join("file-to-stay.txt");
    let file = File::create(&file_path)?;

    let mut cmd = Command::cargo_bin("rnm")?;
    cmd.current_dir(dir.path()).arg("to-rename").arg("renamed");
    cmd.assert().success();

    assert!(predicate::path::exists().eval(&file_path));

    drop(file);
    dir.close()?;

    Ok(())
}

#[test]
fn using_regex() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let file_path = dir.path().join("1.txt");
    let file_path2 = dir.path().join("2.txt");
    let file = File::create(&file_path)?;
    let file2 = File::create(&file_path2)?;

    let mut cmd = Command::cargo_bin("rnm")?;
    cmd.current_dir(dir.path())
        .arg("(\\d+)\\.txt")
        .arg("$1.dcm");
    cmd.assert().success();

    let expected_file_path = dir.path().join("1.dcm");
    let expected_file_path2 = dir.path().join("2.dcm");
    assert!(predicate::path::exists().eval(&expected_file_path));
    assert!(predicate::path::exists().eval(&expected_file_path2));
    assert!(predicate::path::missing().eval(&file_path));
    assert!(predicate::path::missing().eval(&file_path2));

    drop(file);
    drop(file2);
    dir.close()?;

    Ok(())
}

#[test]
fn using_numbers_regex_only() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let file_path = dir.path().join("1");
    let file_path2 = dir.path().join("2.txt");
    let file = File::create(&file_path)?;
    let file2 = File::create(&file_path2)?;

    let mut cmd = Command::cargo_bin("rnm")?;
    cmd.current_dir(dir.path()).arg("^(\\d+)$").arg("$1.dcm");
    cmd.assert().success();

    let expected_file_path = dir.path().join("1.dcm");
    assert!(predicate::path::exists().eval(&expected_file_path));
    assert!(predicate::path::missing().eval(&file_path));
    assert!(predicate::path::exists().eval(&file_path2));

    drop(file);
    drop(file2);
    dir.close()?;

    Ok(())
}
