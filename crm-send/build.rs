use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        .compile_protos(
            &[
                "../protos/crm_send/messages.proto",
                "../protos/crm_send/rpc.proto",
            ],
            &["../protos"],
        )?;

    println!("cargo:rerun-if-changed=../protos/crm_send/*.proto");

    Ok(())
}
