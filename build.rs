use clap::CommandFactory;

#[path="src/cli.rs"]
pub mod cli;
fn main() -> std::io::Result<()> {
    let mut target_dir: ::std::path::PathBuf =
        ::std::env::var_os("CARGO_MANIFEST_DIR")
            .unwrap()
            .into()
        ;
    target_dir.push("target");
    target_dir.push(::std::env::var_os("PROFILE").unwrap());
    let target_dir = &*target_dir;
    println!("cargo:warning={:?}", target_dir);
    let out_dir = std::path::PathBuf::from(target_dir);
    let cmd = cli::Cli::command();
    let man = clap_mangen::Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;
    std::fs::write(out_dir.join("iotools.1"), buffer)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}