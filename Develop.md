# 开发文档

## 技术栈

- `axum` 后端 (备选 `poem` `actix-web`)
  - `minio` 做文件储存 (s3 api)
  - `postgresql` 做数据库， `sea-orm` 做 orm 框架

- `vue3` `ts` 前端 (~~未实现~~, 已跑路)

## Feat

- 一个普通的文件分享平台
- 不仅支持文件派发功能, 同时支持文件收取功能
- 抽象出 Block 概念, 一个 Block 对应着一个文件收/发任务, 同时权限颗粒度也为 Block 级别
- `Rust/axum` 编写, `tokio` 异步运行时
- 采用`minio (s3 api)` 支持的文件存储
- 可配置化 Dcoker 集成
- 支持收取文件的格式化重命名 example: `format!("{}-{}-{}.zip", student_id, task_name, date)`
- 支持文件加密?

## 数据库设计

所有 entity 与 Block 相联系。每个 block 块有两种类别：`sending block` 和 `receiving block`。

`block` 实体
- `block_name` 块名
- `block_description` 块描述
- `block_type` 块种类
- `block_bucket_path` 块桶路径
- `block_format` 块的格式化模板
- `block_fields` 块格式化需要的字符串

`object` 实体
- `object_name` 对象原名 (文件名)
- `object_type` 对象类型 (文件类型)
- `object_size` 对象大小 (文件大小)
- `object_description` 对象描述 (文件描述)
- `object_bucket_name` 对象在桶中的name
- `block_id` 关联的`block`

`object_field` 实体
 - `object_id`
 - `field_name`

## API (`*` means need auth)
- `pong`

- `block`
  - `/block/create` *
  - `/block/list`
  - `/block/delete` *
  - `/block/:block_uuid/info` 获取该 `block` 所有 `object` 信息
  - `/block/:block_uuid/delete` *
  - `/block/:block_uuid/receive`
  - `/block/:block_uuid/send`

- `auth`
  - `/auth` 颁发 JWT

## TODO List
- 数据校验
- 丰富查询接口
- 优化异步大文件传输
- 对每个 Block 增加可选的额外身份验证
- 优化 sending 与 receiving 块的特殊处理
- 配置打包 Docker