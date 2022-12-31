use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn test_can_view_bookmarks() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("bookit")?;

    cmd.arg("--config").arg(".bookit");
    cmd.assert().success().stdout(predicate::str::contains(""));

    // cmd.arg("foobar").arg("test/file/doesnt/exist");
    // cmd.assert()
    //     .failure()
    //     .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}
