use std::fs;

use anyhow::Result;
use prost_build::Config;

fn main() -> Result<()> {
    // prost_build::compile_protos(&["../protos/crm.proto"], &["../protos"])?;

    fs::create_dir_all("src/pb")?;
    let mut config = Config::new();
    config
        .out_dir("src/pb")
        .compile_protos(&["../protos/crm.proto"], &["../protos"])?;

    Ok(())
}
