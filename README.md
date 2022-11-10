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
	repeated User users = 1;
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

```dotenv
DATABASE_URL = 'postgres://localhost:5432/new_db'
page_management_addr = '127.0.0.1:3000'
svc_user = '127.0.0.1:3001'
```

## 开发笔记

### 开发 util-pb

感觉`tonic_build`或`prost-build`值得一看，
```rust
println!("cargo:rerun-if-changed=proto/user.proto");
```
也可以再研究研究。

### 开发 util-auth

- 首先，加密需要么？这个问题需要以后回答。

- 其次，[阅读材料](https://github.com/tyrchen/rust-training/blob/master/live_coding/axum-live/examples/basic.rs)
中看一下，token 和 claims 的正确用法。
怎么通过`FromRequest` trait 把`Claims<>`用的像原生 `Form<> Query<> Extension<>`一样，
并在其上增加验证功能。

- cargo test 能不能别doc test，看着头晕。
solution : ` cargo test --all-targets`

### 开发 page-management
