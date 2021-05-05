use version_check::{Channel, Version};

// We assume that features are never stabilized in patch versions.
// If a "Rust 2.0" is ever released, we'll have to handle that explicitly.
const MSRV_MINOR: u16 = 36;
const CURRENT_MINOR: u16 = 52;

fn main() {
    let msrv = Version::from_mmp(1, MSRV_MINOR, 0);

    let mut minor_used = match Version::read() {
        Some(version) => version,
        None => {
            println!(
                "cargo:warning=Unable to determine rustc version. Assuming rustc {}.",
                msrv
            );
            msrv
        }
    }
    .to_mmp()
    .1;

    // Treat as the stable release, even if not on it.
    let channel = Channel::read();
    match channel {
        Some(channel) if channel.is_beta() => minor_used -= 1,
        Some(channel) if channel.supports_features() => minor_used -= 2,
        _ => {}
    }

    #[allow(unused_assignments, unused_mut)]
    let mut explicit_msrv_set = false;

    // Eager macro expansion would be nice!
    macro_rules! old_stable_compilers {
        ($($msrv_str:literal $minor:literal),+,) => {$(
            #[allow(unused_assignments)]
            #[cfg(feature = $msrv_str)]
            {
                explicit_msrv_set = true;
            }
            if $minor < minor_used {
                #[cfg(feature = $msrv_str)]
                println!(r#"cargo:rustc-cfg=reexport="1.{}""#, $minor + 1);
                println!(r#"cargo:rustc-cfg=since="1.{}""#, $minor + 1);
            } else {
                #[cfg(feature = $msrv_str)]
                println!(r#"cargo:rustc-cfg=shim="1.{}""#, $minor + 1);
                println!(r#"cargo:rustc-cfg=before="1.{}""#, $minor + 1);
            }
        )*};
    }

    // There's no need to include the most recent release of the stable compiler here.
    old_stable_compilers![
        "msrv-1-36" 36,
        "msrv-1-37" 37,
        "msrv-1-38" 38,
        "msrv-1-39" 39,
        "msrv-1-40" 40,
        "msrv-1-41" 41,
        "msrv-1-42" 42,
        "msrv-1-43" 43,
        "msrv-1-44" 44,
        "msrv-1-45" 45,
        "msrv-1-46" 46,
        "msrv-1-47" 47,
        "msrv-1-48" 48,
        "msrv-1-49" 49,
        "msrv-1-50" 50,
        "msrv-1-51" 51,
    ];

    if !explicit_msrv_set {
        for minor in (MSRV_MINOR + 1)..=CURRENT_MINOR {
            println!(r#"cargo:rustc-cfg=shim="1.{}""#, minor);
        }
    }
}
