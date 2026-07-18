#[cfg(windows)]
extern crate winres;

fn main() {
    // Version management: read VERSION and inject it as the compile-time env var APP_VERSION.
    // VERSION is the single source of truth, maintained manually; build.rs no longer rewrites it
    // (to avoid bumping the version number on every build).
    let version_path = "VERSION";
    let version = std::fs::read_to_string(version_path)
        .unwrap_or_else(|_| "1.0.0".to_string())
        .trim()
        .to_string();

    println!("cargo:rustc-env=APP_VERSION={}", version);
    // Rebuild when VERSION changes (so include_str!/env! pick up the new value)
    println!("cargo:rerun-if-changed=VERSION");

    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("static/icon.ico");
        if let Err(e) = res.compile() {
            println!("cargo:warning=Failed to compile windows resource: {}", e);
        }
    }

    // Compile the md4x C sources into a static library
    compile_md4x();
}

fn compile_md4x() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let md4x_src = std::path::Path::new(&manifest_dir).join("csrc/md4x");
    let md4x_renderers = md4x_src.join("renderers");
    let libyaml_include = std::path::Path::new(&manifest_dir).join("csrc/libyaml/include");
    let libyaml_src_dir = std::path::Path::new(&manifest_dir).join("csrc/libyaml/src");

    // Use the host target triple so the C objects match the Rust target.
    let target = rustc_target_to_zig(&rustc_host_target());
    let is_windows = target.contains("windows");

    // List of C source files
    // All files need YAML_DECLARE_STATIC, because md4x-html.c includes yaml.h
    // Common YAML defines (for all files that reference yaml.h)
    let yaml_common_defines = vec![
        "-DYAML_DECLARE_STATIC",
        "-DYAML_VERSION_MAJOR=0",
        "-DYAML_VERSION_MINOR=2",
        "-DYAML_VERSION_PATCH=5",
        r#"-DYAML_VERSION_STRING="0.2.5""#,
    ];

    // libyaml internal compilation needs POSIX compatibility (MSVC lacks strdup/snprintf)
    let yaml_internal_defines = {
        let mut v = yaml_common_defines.clone();
        if is_windows {
            v.push("-Dstrdup=_strdup");
        }
        v
    };

    let c_sources = [
        (md4x_src.join("md4x.c"), vec!["-DMD4X_USE_UTF8"]),
        (md4x_src.join("entity.c"), vec![]),
        (md4x_renderers.join("md4x-html.c"), yaml_common_defines.clone()),
        (md4x_renderers.join("md4x-heal.c"), vec![]),
        (libyaml_src_dir.join("api.c"), yaml_internal_defines.clone()),
        (libyaml_src_dir.join("reader.c"), yaml_internal_defines.clone()),
        (libyaml_src_dir.join("scanner.c"), yaml_internal_defines.clone()),
        (libyaml_src_dir.join("parser.c"), yaml_internal_defines.clone()),
    ];

    let include_flags: Vec<String> = [
        md4x_src.as_os_str().to_str().unwrap(),
        md4x_renderers.as_os_str().to_str().unwrap(),
        libyaml_include.as_os_str().to_str().unwrap(),
        libyaml_src_dir.as_os_str().to_str().unwrap(),
    ]
    .iter()
    .flat_map(|p| vec!["-I".to_string(), p.to_string()])
    .collect();

    // Find zig
    let zig = find_zig();

    // Compile each .c file into a .o
    let mut obj_files: Vec<std::path::PathBuf> = Vec::new();
    for (src, extra_defines) in &c_sources {
        let src_str = src.as_os_str().to_str().unwrap();
        let stem = src.file_stem().unwrap().to_str().unwrap();
        let obj = std::path::Path::new(&out_dir).join(format!("{}.o", stem));

        let mut args: Vec<String> = vec![
            "cc".to_string(),
            format!("--target={}", target),
            "-c".to_string(),
            "-O2".to_string(),
            "-w".to_string(), // suppress warnings
        ];
        args.extend(include_flags.iter().cloned());
        for def in extra_defines {
            args.push(def.to_string());
        }
        args.extend(["-o".to_string(), obj.as_os_str().to_str().unwrap().to_string()]);
        args.push(src_str.to_string());

        let status = std::process::Command::new(&zig)
            .args(&args)
            .current_dir(&manifest_dir)
            .status()
            .unwrap_or_else(|e| panic!("Failed to execute {:?}: {}", zig, e));

        if !status.success() {
            panic!("Failed to compile {}", src_str);
        }
        obj_files.push(obj);
    }

    // Create the static library using zig ar
    let lib_name = if is_windows { "md4x.lib" } else { "libmd4x.a" };
    let lib_path = std::path::Path::new(&out_dir).join(lib_name);
    let mut ar_args: Vec<String> = vec![
        "ar".to_string(),
        "rcs".to_string(),
        lib_path.as_os_str().to_str().unwrap().to_string(),
    ];
    for obj in &obj_files {
        ar_args.push(obj.as_os_str().to_str().unwrap().to_string());
    }

    let status = std::process::Command::new(&zig)
        .args(&ar_args)
        .current_dir(&out_dir)
        .status()
        .unwrap_or_else(|e| panic!("Failed to execute {:?} ar: {}", zig, e));

    if !status.success() {
        panic!("Failed to create the static library");
    }

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=md4x");

    // Tell cargo to rebuild when these files change
    println!("cargo:rerun-if-changed=csrc/md4x");
    println!("cargo:rerun-if-changed=csrc/libyaml");
}

/// Return the host target triple reported by `rustc -vV`.
/// This lets the C sources be compiled for the same target as the Rust code.
fn rustc_host_target() -> String {
    let output = std::process::Command::new("rustc")
        .args(["-vV"])
        .output()
        .expect("failed to run rustc to determine host target");
    let stdout = String::from_utf8(output.stdout).expect("rustc output is not UTF-8");
    for line in stdout.lines() {
        if let Some(host) = line.strip_prefix("host:") {
            return host.trim().to_string();
        }
    }
    panic!("could not determine rustc host target from: {}", stdout)
}

/// Convert a rustc target triple into the format zig expects.
/// rustc: <arch>-<vendor>-<os>-<abi>   e.g. x86_64-unknown-linux-gnu
/// zig:   <arch>-<os>-<abi>            e.g. x86_64-linux-gnu
fn rustc_target_to_zig(target: &str) -> String {
    let parts: Vec<&str> = target.split('-').collect();
    match parts.len() {
        4 => format!("{}-{}-{}", parts[0], parts[2], parts[3]),
        3 => format!("{}-{}", parts[0], parts[2]),
        _ => target.to_string(),
    }
}

/// Find the zig executable
fn find_zig() -> std::path::PathBuf {
    // First, check for zig on PATH
    if let Ok(output) = std::process::Command::new("zig").arg("version").output() {
        if output.status.success() {
            return std::path::PathBuf::from("zig");
        }
    }
    // Fall back to known locations
    let known_paths = [
        r"D:\zig\zig.exe",
        r"C:\zig\zig.exe",
    ];
    for p in &known_paths {
        if std::path::Path::new(p).exists() {
            return std::path::PathBuf::from(p);
        }
    }
    panic!("zig not found; please install zig or add it to PATH");
}
