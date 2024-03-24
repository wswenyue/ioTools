use clap::CommandFactory;

#[path="src/cli.rs"]
pub mod cli;
fn main() -> std::io::Result<()> {
    println!("cargo:warning= TARGET={:?}", ::std::env::var_os("TARGET"));
    println!("cargo:warning= CARGO_PKG_VERSION={:?}", ::std::env::var_os("CARGO_PKG_VERSION"));
    println!("cargo:warning= CARGO_PKG_AUTHORS={:?}", ::std::env::var_os("CARGO_PKG_AUTHORS"));
    println!("cargo:warning= CARGO_PKG_NAME={:?}", ::std::env::var_os("CARGO_PKG_NAME"));
    println!("cargo:warning= CARGO_MANIFEST_DIR={:?}", ::std::env::var_os("CARGO_MANIFEST_DIR"));
    println!("cargo:warning= PROFILE={:?}", ::std::env::var_os("PROFILE"));

    let mut man_page_path: ::std::path::PathBuf =
        ::std::env::var_os("CARGO_MANIFEST_DIR").unwrap().into();
    man_page_path.push("../target");
    // match ::std::env::var_os("TARGET") {
    //     None => {}
    //     Some(target) => {
    //         man_page_path.push(target);
    //     }
    // }
    match ::std::env::var_os("PROFILE") {
        None => {}
        Some(profile) => {
            man_page_path.push(profile);
        }
    }
    let pkg_name = ::std::env::var_os("CARGO_PKG_NAME").unwrap().into_string().unwrap();
    man_page_path.push(pkg_name+".1");
    println!("cargo:warning={:?}", man_page_path);
    let cmd = cli::Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(man_page_path, buffer)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}