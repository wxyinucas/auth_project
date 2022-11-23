# 动态配置、停机和部署

---

## 遗留问题
- 查（两种）-- 增、删、改：
  - 分别应该设计怎样的输入和输出？（注意，强烈建议只有"查"结果`Option<T>,Vec<T>`与其余不同`Result<T, Err>`。）
  - 在 `db层面` 和  `service层面`，分别应该处理什么样的问题？从而接口有什么区别？
- DB：
  - 如何更高效的使用 `pgcli` 验证开发目的？（所见即所得）
  - 什么是"页"，应该有什么方法？如何翻页？
  - 怎样引起冲突，使得值唯一？
- URL 管理：
  - router，html 跳转 和 handler redirect，如何管理，是否硬编码？
  - 确定一个开发 和 测试流程。
  - headless auto test browser.
- 配置：
  - 哪些字段需要动态配置？怎么动态配置？
  - tyr chen 中，config需要设计哪些方法？有哪些载入的实践？
- 测试：
  - 先手动测试后自动测试，生成测试的标准是什么？测试的trigger是什么？
  - 写测试的流程是什么？
- CommonClaims<T>:
  - 方法 与 泛型 的抽象内容分别是什么？
  - 这样做有什么好处？
- 时间/日期
  - 结构？
  - 和string int 的转换？
  - 运算，local & UTC？
  - 区间？
- 学习/开发时的 `反馈`
  - 哪些场景 需要反馈？
  - 怎样可以更快的得到反馈？
- 架构设计：
  - 变与不变：如何保证 test 不随开发而改变？本质上什么应该不变？
  - 如何汇总设计 与 配置信息？
  - refactor：如何追踪 refactor 过程，如何判断结果？
  - 如何设计路由？如何制定带反馈的开发流程？
  - handler 和 日志更加合理的结合方式？
- Network:(later)
  - http 究竟什么，header 和 body 有什么内容，有什么分类？
  - tcp 与 http 的关系？

### 讲一个自己真正理解的故事

---
## 开发标准

### 开发目标（功能）
- （自动化）发布与部署
- 所有服务的cli 一键启动/停止 与日志展示
- 格式化日志的[收集与消费](https://github.com/prometheus/prometheus).
   更推荐的另一个[参考open-telemetry](https://docs.rs/tracing-opentelemetry/latest/tracing_opentelemetry/).

### 开发流程（方法论）
- 学习法：目标与快速反馈
- 开发流程：随时test
- 设计法：变与不变

---
## 开发流程
- SSH
  - 如何建立无需登陆的连接？
  - 如何持久链接？
  - 如何端口转发？
  - 如何传递文件，command line tools or Jetbrains
- Tmux 的操作
  - 如何暂退/重连窗口？
- 持久化：
  - tmux
  - nohup and redirect to file.
- Linux:(later)
  - create new user
  - modify user's authority

## Nginx + ssh
- ssh: ```bash  ssh  -R 3000:localhost:3000 root@123.249.103.19```
- nginx:
```
  server{
    listen 3000;
    server_name localhost;

          location / {
              proxy_set_header  X-Real-IP  $remote_addr;
              proxy_set_header  X-Forwarded-For $proxy_add_x_forwarded_for;
              proxy_set_header Host $http_host;
              proxy_redirect off;

              proxy_pass http://localhost:3000;
            }
    }
```
- 配置防火墙，允许自定义TCP通过3000端口。
