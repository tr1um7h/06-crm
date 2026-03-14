use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;
    let config = tonic_prost_build::configure();
    config.out_dir("src/pb").compile_protos(
        &[
            "../protos/user_stats/messages.proto",
            "../protos/user_stats/rpc.proto",
        ],
        &["../protos"],
    )?;

    println!("cargo:rerun-if-changed=../protos/user_stats/*.proto");

    Ok(())
}
