use anyhow::Result;
use std::fs;

fn main() -> Result<()> {
    fs::create_dir_all("src/pb")?;

    tonic_prost_build::configure()
        .out_dir("src/pb")
        // 为 User 消息添加 serde 属性
        .type_attribute("User", "#[derive(serde::Serialize, serde::Deserialize)]")
        .type_attribute("User", r#"#[serde(rename_all = "camelCase")]"#)
        // 为 User 消息添加 Builder 派生宏
        .type_attribute("User", "#[derive(derive_builder::Builder)]")
        .type_attribute("User", "#[builder(setter(into, strip_option), default)]")
        .type_attribute("QueryRequest", "#[derive(derive_builder::Builder)]")
        .type_attribute("RawQueryRequest", "#[derive(derive_builder::Builder)]")
        .type_attribute("TimeQuery", "#[derive(derive_builder::Builder)]")
        .type_attribute("IdQuery", "#[derive(derive_builder::Builder)]")
        .field_attribute("User.email", "#[builder(setter(into))]")
        .field_attribute("User.name", "#[builder(setter(into))]")
        .field_attribute("RawQueryRequest.query", "#[builder(setter(into))]")
        .field_attribute("TimeQuery.lower", "#[builder(setter(into, strip_option))]")
        .field_attribute("TimeQuery.upper", "#[builder(setter(into, strip_option))]")
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
