# 授权鉴权练习

本项目通过建立网站来实现用户的认证练习。

## 练习内容

- `tera` 和 [模板例子](https://getbootstrap.com/docs/5.2/examples/blog) 来搭建网页。
- `Jwt`实现鉴权；`bcrypt`对明文密码加密(需要么？)。
- 设计使用`Option`、`Result`的，逻辑清晰的增删查改规则。
- 使用`gRPC`方式实现两个功能：
    - proto 的传递
    - 用户的增删查改，(先不实现改，和删在这里有点类似，将来用别的抽象代替)
    - 用户登录时令牌的发放，访问网页时的鉴权。
- 合理的`tracing` message.
- `sqlx::test` 的 练习。

## 未来想法

- 增加对 "paginate" 的支持
- 增加对 时间处理 的支持
- 增加对 md article 的支持
- 增加对 category 和 tag 的支持
- 增加对 留言系统 的支持
- proto stream 的理解

## 开发流程(尽量独立完成)

- 需求分析
- 设计:
    - crate
    - gRPC 的 proto 接口 和 数据库
    - 设计trait，结合逻辑，定下接口和相应 error
    - 全局配置
- 设计 test 并 实现
- 实现核心功能
- 实现全局 cli 控制器

## 开发流程v2

- 需求分析
- 设计:
    - crate
    - gRPC 的 proto 接口 和 数据库
    - 设计trait，结合逻辑，定下接口和相应 error
    - 全局配置
- 按 crate 实现：
    - 写出 函数 和 预期行为 和 error
    - 设计 test 并 实现
    - 实现 具体功能
    - 增加 log

### 需求分析

![需求图](doc/pic/aim.png)

### 设计

- crate：
    - 网页: page-management
    - 功能: util-pb util-auth
    - 服务: svc-auth
- Proto:

```proto
syntax = "proto3";

package user;

message User{
	int32 id = 1;
	string email = 2;
	string password = 3;
	AccountStatus status = 4;
}

enum AccountStatus{
	ACTIVE = 0;
	FROZEN = 1;
}

message CreateUserRequest{
	string email = 1;
	string password = 2;
}

message CreateUserResponse{
	int32 id = 1;
}

message QueryUserRequest{
	oneof identity{
		int32 id = 1;
		string email = 2;
	}
	AccountStatus status = 3;
}

message QueryUserResponse{
	optional User users = 1;
}

message DeleteUserRequest{
	oneof identity{
		int32 id = 1;
		string email = 2;
	}
}

message DeleteUserResponse{
	int32 id = 1;
}

service UserService{
	rpc create(CreateUserRequest) returns (CreateUserResponse);
	rpc query(QueryUserRequest) returns (QueryUserResponse);
	rpc delete(DeleteUserRequest) returns (DeleteUserResponse);
}

```

- 数据库建表

```postgresql
CREATE SCHEMA auth;
CREATE TYPE auth.user_status AS ENUM ('active', 'freeze');

CREATE TABLE auth.users
(
    id       SERIAL PRIMARY KEY,
    email    VARCHAR(255)     Not Null,
    password VARCHAR(255)     NOT NULL,
    status   auth.user_status NOT NULL DEFAULT 'active'
)
```

- 全局设置

[参见文件](.env)

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
      ```rust
      let cookies = req.headers().typed_get::<Cookie>();
      let state = req.extensions().get::<InnerState>().unwrap();

    - 额外注意，这个middleware 和 Extension 在 router 中的关系。
    - 用`dotenv` 取变量:
    ```rust
     use dotenv;
     dotenv::dotenv().ok();

     let cookie_name = std::env::var("rex_auth_token").expect("AUTH token name is required.");
  ```

  - 注意处理error的层级与时机
  ```rust
   let cookie_name = std::env::var("rex_auth_token").expect("AUTH token name is required.");
  ```
