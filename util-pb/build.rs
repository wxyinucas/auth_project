use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sqlx_type(&["user.AccountStatus"], None)
        .with_type_attributes(&["user.User"], &[r#"#[derive(serde::Serizalize)]"#])
        .compile(&["./proto/user.proto"], &["."])
        .unwrap();

    println!("cargo:rerun-if-changed=proto/user.proto");
}
