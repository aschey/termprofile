use std::process::{Command, Stdio};

fn main() {
    let handle = Command::new("cargo")
        .args(["run", "--example=convert", "--features=convert"])
        .stdout(Stdio::piped())
        // set TTY_FORCE to force the profile detector to treat the subprocess like a TTY
        .env("TTY_FORCE", "1")
        .spawn()
        .unwrap();
    let out = handle.wait_with_output().unwrap();
    println!("{}", String::from_utf8(out.stdout).unwrap());
}
