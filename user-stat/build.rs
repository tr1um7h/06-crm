use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        // 为 User 消息添加 serde 属性
        .type_attribute("User", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("User", r#"#[serde(rename_all = "camelCase")]"#)
        .compile_protos(
            &[
                "../protos/user_stats/messages.proto",
                "../protos/user_stats/rpc.proto",
            ],
            &["../protos"],
        )?;

    println!("cargo:rerun-if-changed=../protos/user_stats/*.proto");

    Ok(())
}
