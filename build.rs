fn main() -> miette::Result<()> {
    // let path = std::path::PathBuf::from("src");
    let prefix = std::env::var("CONDA_PREFIX").expect("CONDA_PREFIX not set");
    let ompl_version = "1.6";
    let mut b = autocxx_build::Builder::new(
        "src/main.rs",
        [
            format!("{prefix}/include/ompl-{ompl_version}").to_string(),
            format!("{prefix}/include/eigen3").to_string(),
            format!("{prefix}/include").to_string(),
        ],
    )
    .build()?;
    b.flag_if_supported("-std=c++17").compile("ompl-rs");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/input.h");
    Ok(())
}
