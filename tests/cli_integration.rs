//! CLI integration: sign/verify and stdin/stdout piping.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn otm() -> Command {
    Command::new(env!("CARGO_BIN_EXE_its_otm"))
}

struct TempDir(PathBuf);

impl TempDir {
    fn new(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "its_otm_cli_{name}_{}_{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&path).expect("temp dir");
        Self(path)
    }

    fn path(&self) -> &Path {
        &self.0
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[test]
fn cli_keygen_sign_verify_file() {
    let dir = TempDir::new("sign");
    let state = dir.path().join("signer.state");
    let payload = dir.path().join("payload.bin");
    let bundle = dir.path().join("attest.otm");

    fs::write(&payload, b"public attestation payload").unwrap();

    assert!(otm()
        .args(["keygen", "--out", state.to_str().unwrap()])
        .status()
        .unwrap()
        .success());

    assert!(otm()
        .args([
            "sign",
            "--state",
            state.to_str().unwrap(),
            "--in",
            payload.to_str().unwrap(),
            "--out",
            bundle.to_str().unwrap(),
        ])
        .status()
        .unwrap()
        .success());

    let verify = otm()
        .args([
            "verify",
            "--bundle",
            bundle.to_str().unwrap(),
            "--payload",
            payload.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(verify.status.success(), "{:?}", verify.stderr);
    assert!(String::from_utf8_lossy(&verify.stdout).contains("VALID"));
}

#[test]
fn cli_sign_from_stdin() {
    let dir = TempDir::new("stdin");
    let state = dir.path().join("s.state");
    let bundle = dir.path().join("b.otm");

    otm().args(["keygen", "--out", state.to_str().unwrap()]).status().unwrap();

    let mut child = otm()
        .args([
            "sign",
            "--state",
            state.to_str().unwrap(),
            "--in",
            "-",
            "--out",
            bundle.to_str().unwrap(),
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(b"stdin payload bytes")
        .unwrap();
    assert!(child.wait().unwrap().success());

    assert!(otm()
        .args(["verify", "--bundle", bundle.to_str().unwrap()])
        .status()
        .unwrap()
        .success());
}

#[test]
fn cli_demo_exits_zero() {
    let out = otm().arg("demo").output().unwrap();
    assert!(out.status.success());
    assert!(String::from_utf8_lossy(&out.stdout).contains("message:"));
}

#[test]
fn cli_verify_bundle_stdin() {
    let demo = otm().arg("demo").output().unwrap();
    assert!(demo.status.success());

    let mut child = otm()
        .args(["verify", "--bundle", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.as_mut().unwrap().write_all(&demo.stdout).unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
}
