use anyhow::Result;
use proto_builder_trait::tonic_prost::BuilderAttributes; // 添加这一行
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        .with_derive_builder(&["WelcomeRequest", "RecallRequest", "RemindRequest"], None)
        // .with_field_attributes(
        //     &["User.email", "User.name", "RawQueryRequest.query"],
        //     &[r#"#[builder(setter(into))]"#],
        // )
        .compile_protos(
            &["../protos/crm/messages.proto", "../protos/crm/rpc.proto"],
            &["../protos"],
        )?;

    println!("cargo:rerun-if-changed=../protos/crm/*.proto");

    Ok(())
}
