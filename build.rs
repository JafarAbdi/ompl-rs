fn main() -> miette::Result<()> {
    // let path = std::path::PathBuf::from("src");
    let mut b = autocxx_build::Builder::new(
        "src/main.rs",
        ["/usr/include/ompl-1.5", "/usr/include/eigen3"],
    )
    .build()?;
    b.flag_if_supported("-std=c++17").compile("ompl-rs");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
