use proto_builder_trait::tonic::BuilderAttributes;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .with_sqlx_type(&["user.UserAuthLevel", "user.UserStatus"], None)
        // .with_sqlx_from_row(
        //     &["user.User"],
        //     Some(&[r#"#[derive(serde::Serialize)]"#]), // TODO 阅读crate源码，注意实现方式，非常棒的设计；看看怎么写的，是一个范本。
        // ) // TODO 为了测试自己写from row 注释了
        .with_type_attributes(&["user.User"], &[r#"#[derive(serde::Serialize)]"#])
        .compile(&["./user.proto"], &["."])
        .unwrap();

    println!("cargo:rerun-if-changed=./build.rs"); // TODO 注意这个用法！
}
