use std::env;
use std::path::PathBuf;
use std::process::Command;
use clap::CommandFactory;

#[path= "src/cli.rs"]
pub mod cli;

pub fn get_workspace_root() -> anyhow::Result<PathBuf> {
    let current_dir = env::current_dir()?;
    let cmd_output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()?;

    if !cmd_output.status.success() {
        return Ok(current_dir);
    }

    let json =
        serde_json::from_str::<serde_json::Value>(String::from_utf8(cmd_output.stdout)?.as_str())?;
    let path = match json.get("workspace_root") {
        Some(val) => match val.as_str() {
            Some(val) => val,
            None => return Ok(current_dir),
        },
        None => return Ok(current_dir),
    };
    Ok(PathBuf::from(path))
}

fn main() -> std::io::Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning= get_workspace_root={:?}", get_workspace_root());
    println!("cargo:warning= TARGET={:?}", env::var_os("TARGET"));
    println!("cargo:warning= CARGO_PKG_VERSION={:?}", env::var_os("CARGO_PKG_VERSION"));
    println!("cargo:warning= CARGO_PKG_AUTHORS={:?}", env::var_os("CARGO_PKG_AUTHORS"));
    println!("cargo:warning= CARGO_PKG_NAME={:?}", env::var_os("CARGO_PKG_NAME"));
    println!("cargo:warning= CARGO_MANIFEST_DIR={:?}", env::var_os("CARGO_MANIFEST_DIR"));
    println!("cargo:warning= PROFILE={:?}", env::var_os("PROFILE"));

    let mut man_page_path: PathBuf = get_workspace_root().unwrap();
    man_page_path.push("target");
    // match ::std::env::var_os("TARGET") {
    //     None => {}
    //     Some(target) => {
    //         man_page_path.push(target);
    //     }
    // }
    match env::var_os("PROFILE") {
        None => {}
        Some(profile) => {
            man_page_path.push(profile);
        }
    }
    let pkg_name = env::var_os("CARGO_PKG_NAME").unwrap().into_string().unwrap();
    man_page_path.push(pkg_name + ".1");
    println!("cargo:warning={:?}", man_page_path);
    let cmd = cli::Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(man_page_path, buffer)?;
    Ok(())
}