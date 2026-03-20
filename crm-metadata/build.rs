use anyhow::Result;
// use proto_builder_trait::tonic_prost::BuilderAttributes;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        // .with_serde(
        //     &["Content", "ContentType", "Publisher"],
        //     true,
        //     true,
        //     Some(&[r#"#[serde(rename_all = "camelCase")]"#]),
        // )
        // .with_type_attributes(&["MaterializeRequest"], &[r#"#[derive(Eq, Hash)]"#])
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
