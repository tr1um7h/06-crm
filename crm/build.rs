use anyhow::Result;
use std::fs;
// use prost_build::Config;

fn main() -> Result<()> {
    // prost_build::compile_protos(&["../protos/crm.proto"], &["../protos"])?;

    fs::create_dir_all("src/pb")?;
    // let mut config = Config::new();
    let config = tonic_prost_build::configure();
    config
        .out_dir("src/pb")
        .compile_protos(&["../protos/crm/crm.proto"], &["../protos"])?;

    println!("cargo:rerun-if-changed=../protos/crm/crm.proto");

    Ok(())
}
