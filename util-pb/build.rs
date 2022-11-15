use std::process::Command;

use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sqlx_type(&["user.AccountStatus"], None)
        .with_sqlx_from_row(&["user.User"], Some(&[r#"#[derive(serde::Serialize)]"#]))
        // .with_type_attributes(&["user.User"], &[r#"#[derive(serde::Serialize)]"#])
        .compile(&["./proto/user.proto"], &["."])
        .unwrap();

    Command::new("cargo").args(["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=proto/user.proto");
    println!("cargo:rerun-if-changed=build.rs");
}
