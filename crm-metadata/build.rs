use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/meta_data/messages.proto",
                "../protos/meta_data/rpc.proto",
            ],
            &["../protos"],
        )?;

    println!("cargo:rerun-if-changed=../protos/meta_data/*.proto");

    Ok(())
}
