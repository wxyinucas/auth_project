## 开发笔记

### 开发 util-pb

- 感觉`tonic_build`或`prost-build`值得一看，

```rust
println!("cargo:rerun-if-changed=proto/user.proto");
```

也可以再研究研究。

- QueryResponse 可以再考率：只有"查"可能没有结果，所以可以用option。
  但是查也要支持"多返回值"，所以，需要改这个设计。

### 开发 util-auth

- 首先，加密需要么？这个问题需要以后回答。

- 其次，[阅读材料](https://github.com/tyrchen/rust-training/blob/master/live_coding/axum-live/examples/basic.rs)
  中看一下，token 和 claims 的正确用法。
  怎么通过`FromRequest` trait 把`Claims<>`用的像原生 `Form<> Query<> Extension<>`一样，
  并在其上增加验证功能。

- cargo test 能不能别doc test，看着头晕。
  solution : ` cargo test --all-targets`

### 开发 page-management

- 目标：
    - tera
    - sqlx::type
    - 为后续开发保留接口；全局路由 与 `.env`配置
    - tracing
- 想法：
    - 所有tracing 重定向到同一个位置？
    - static 文件的重新布局？
    - 将来尝试用 dyn 抽象 CommonClaims
    - 登陆失败，带失败原因的呈现
    - 中间件的test怎么写啊，啥时候写啊。。
    - 陈天的CommonClaims范型传递没学会。
- Router:
    - login: page, 向post login 传入 form
    - post login: 操作cookie
    - dashboard: 展示 claims 内容
    - sign out: 删除cookie

- 笔记：
    - 注意axum.rs 在auth middleware 中的技巧，extension，可以拿到额外信息。
      ```rust-
      let cookies = req.headers().typed_get::<Cookie>();
      let state = req.extensions().get::<InnerState>().unwrap();
      ```

    - 额外注意，这个middleware 和 Extension 在 router 中的关系。
    - 用`dotenv` 取变量:
      ```rust-
      use dotenv;
      dotenv::dotenv().ok();
      let cookie_name = std::env::var("rex_auth_token").expect("AUTH token name is required.");
      ```

    - 注意处理error的层级与时机
      ```rust-
      let cookie_name = std::env::var("rex_auth_token").expect("AUTH token name is required.");
      ```

    - 使用Cookie遇到大问题: 看看最后怎么解决了`Extend<HeadValue>`，有什么启示。
    - Deref AsRef! 重构state
    - 图片 location 与 template。
    - cookie 的读与写
    -
        - cookie 的读写很不优雅; 注意header map
        - html 改 标签里的 id 以 配饰 tera

## 消化时间！

整理代码，处理cargo clippy 错误，看下之前的问题，解决一下。

- `Asref` 一个直接的变换；`Deref` 对类似智能指针，用*触发。
- `Extend<HeadValue>`，找到实现 trait extend 的数据结构vec 即可。

- 通过 tyr chen 的例子学auth 结构
- 各种 test 的增加

### 开发 util-auth v2

- 思考，范型夹在哪里，用幽灵结构放在 struct 还是放在函数？
    - 放在具体函数里，这样Jwt可以在同一个服务中处理不同类型的Claims（虽然可能不会有这种需求）
- 给 claims 定义一个 trait
- ! 我对 trait Claims 的 builder 设计甚至还有点巧妙，点个赞！
    - 使用 trait DeserializeOwned 代替 Deserialize<'a>
- 将来可以进行一个`Rust 内存效率`的新专题。
- `get_epoch()` 包进trait claims中?
- （时刻）现在写完了 FromRequest，test呢？
    - 直接写一个脚手架代码，手动测试--是自动化的好入手点
- `git rm --cache` 只删git记录，不删文件。

- 静态文件处理：
    - 将配套css js 都放到assets下，修改html中对应路径
- 重新考虑 CommonClaims 和 Jwt 泛型的关系

### 开发 svc-users

- 现在的问题，如何实现trait UserService，为什么 tyr 又做了一层抽象？
    - 好像是，有一层抽象是用于和db交互的，US这一层是用于 gRPC 的。
    - 注意：用于sqlx的trait接口和 用于gRPC的接口是不同的；这是因为，在gRPC调用sqlx时，可以做一些预先处理。所以，sqlx的接口和结果都更加明确。
- database migration:
    - `cargo install sqlx-cli`
    - `sqlx migrate add init -r`
    - `sqlx migrate run`
- Deref AsRef: 证明了deref就已经足够了。
- 注意，sqlx::type 与 postgres::type 的关系，`svc-users/src/db_pool.rs:106:14`.
    - 一个是`impl FromRow for User`, 一个是针对 type 的专用转换。
    - 结论是，enum必须写一个结构来转换，然后 FromRow
- database test:
    - `cargo test -- --nocapture`  `cargo test --all-targets`
        - `cargo test --lib` is enough.
    - 再参考 tyr reservation/service/src/test_utils, `struct TestConfig`. 非常好的资料！
    - 生成user 和 password
    - test db 如何从环境中读取信息
- 肯定要重新学习 `stream`

## 开发 svc-users 与 util-auth 的联合接口
- fn `svc_users()`，脑洞打开的函数，并不优雅，但是锻炼了使用spawn的技巧。
