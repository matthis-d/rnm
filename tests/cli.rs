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

#[test]
fn with_several_parts() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let file_path = dir
        .path()
        .join("my-favourite-serie-s01e01[SOME weird things]1080x720.avi");

    let file_path2 = dir
        .path()
        .join("my-favourite-serie-s01e02[SOME weird things]1080x720.avi");

    let file = File::create(&file_path)?;
    let file2 = File::create(&file_path2)?;

    let mut cmd = Command::cargo_bin("rnm")?;
    cmd.current_dir(dir.path())
        .arg("my-favourite-serie-s01e(\\d+).*\\.avi")
        .arg("My favourite serie S01E$1.avi");
    cmd.assert().success();

    let expected_file_path = dir.path().join("My favourite serie S01E01.avi");
    let expected_file_path2 = dir.path().join("My favourite serie S01E02.avi");
    assert!(predicate::path::missing().eval(&file_path));
    assert!(predicate::path::missing().eval(&file_path2));
    assert!(predicate::path::exists().eval(&expected_file_path));
    assert!(predicate::path::exists().eval(&expected_file_path2));

    drop(file);
    drop(file2);
    dir.close()?;

    Ok(())
}

#[test]
fn with_non_utf8_filename() -> Result<(), Box<dyn std::error::Error>> {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    // This test is Unix-specific due to how non-UTF8 paths work
    // Note: Some filesystems (like macOS APFS) enforce UTF-8 and won't allow
    // creating files with invalid UTF-8 names, so this test may skip on those systems
    #[cfg(unix)]
    {
        let dir = tempfile::tempdir()?;

        // Create a filename with invalid UTF-8 bytes
        // 0xFF 0xFE are not valid UTF-8 sequences
        let invalid_utf8 = b"file\xFF\xFEtest.txt";
        let os_str = OsStr::from_bytes(invalid_utf8);
        let file_path = dir.path().join(os_str);

        // Try to create the file - some filesystems may reject this
        let file_result = File::create(&file_path);

        match file_result {
            Ok(file) => {
                // File was created successfully, proceed with the test
                drop(file);

                // Try to rename using a regex that matches "test"
                // The lossy conversion should convert the invalid bytes to
                let mut cmd = Command::cargo_bin("rnm")?;
                cmd.current_dir(dir.path()).arg("test").arg("renamed");
                cmd.assert().success();

                // The file should have been renamed (lossy conversion allows processing)
                // We can't easily check the exact new name since it depends on lossy conversion,
                // but we can verify the original file is gone
                assert!(predicate::path::missing().eval(&file_path));
            }
            Err(_) => {
                // Filesystem doesn't allow invalid UTF-8 filenames (e.g., macOS APFS)
                // This is expected behavior - the filesystem enforces UTF-8
                // Our code would still handle such paths correctly if they existed,
                // but we can't create them for testing on this filesystem
                // Test passes by skipping
            }
        }

        dir.close()?;
    }

    // On non-Unix systems, skip this test
    #[cfg(not(unix))]
    {
        // Test passes trivially on non-Unix systems
    }

    Ok(())
}
