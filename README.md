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
    - 服务: svc-users
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
