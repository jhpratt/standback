use std::{env, process::Command, time::SystemTime};

/// Return the minor version of the compiler currently in use.
/// `None` is returned if a version cannot be determined.
fn compiler_in_use() -> Option<usize> {
    let raw_output = Command::new("rustc")
        .args(&["--version", "--verbose"])
        .output()
        .ok()?
        .stdout;
    let output = std::str::from_utf8(&raw_output).ok()?;

    let release_str =
        &output.lines().find(|line| line.starts_with("release: "))?["release: ".len()..];

    release_str.split('.').nth(1)?.parse().ok()
}

fn main() {
    const MSRV: usize = 31;

    let seconds_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let seconds_since_v1_0 = seconds_since_epoch - 1_431_648_000;
    let current_rustc_release = (seconds_since_v1_0 / (6 * 604_800)) as usize;

    let compiler_used = match compiler_in_use() {
        Some(version) => version,
        None => {
            println!(
                "cargo:warning=Unable to determine rustc version. Assuming rustc 1.{}.0.",
                MSRV
            );
            MSRV
        }
    };

    for minor in (MSRV + 1)..=current_rustc_release {
        if minor <= compiler_used {
            println!("cargo:rustc-cfg=since_1_{}", minor);
        } else {
            println!("cargo:rustc-cfg=before_1_{}", minor);
        }
    }

    if env::var("CARGO_FEATURE_STD").is_ok() {
        println!("cargo:rustc-cfg=std");
    }
}
