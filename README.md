# WebDAV Backup

通过 WebDAV 备份本地文件和 MySQL / PostgreSQL 数据库的 CLI 工具。

## 目录结构

- `rust/` — Rust 实现
- （预留）其他语言实现

## 功能

- 文件全量备份（支持类 `.gitignore` 排除规则）
- MySQL 数据库备份（`mysqldump`）
- PostgreSQL 数据库备份（`pg_dump`）
- 多 WebDAV 远程源配置，每个项目可指定使用哪个源
- ZIP 压缩 + AES-256 密码保护
- 三级继承配置：全局 → 远程源 → 备份项目
- 远程备份保留策略（保留最新 N 个，0 表示不删除）
- 美观的控制台日志输出

## 环境要求

- Rust 1.75+
- `mysqldump`（MySQL 备份时需要）
- `pg_dump`（PostgreSQL 备份时需要）

## 配置

复制示例配置文件并编辑：

```bash
cd rust
cp config.example.toml config.toml
```

配置说明：

- `[global]` — 全局默认设置
  - `zip_password` — 全局 ZIP 解压密码，不设置则不加密
  - `retain_count` — 全局保留数量，`0` 表示不删除旧备份
  - `log_level` — 日志级别：`trace`, `debug`, `info`, `warn`, `error`

- `[[source]]` — 远程 WebDAV 源（可配置多个）
  - `name` — 源名称，供项目引用
  - `url`, `username`, `password` — WebDAV 连接信息
  - `sub_dir` — 该源下的默认远程子目录
  - `zip_password` — 覆盖全局密码
  - `retain_count` — 覆盖全局保留数量

- `[[project]]` — 备份项目（每个项目只能是文件 / MySQL / PgSQL 中的一种）
  - `name` — 项目名称
  - `source` — 引用哪个远程源
  - `sub_dir` / `zip_password` / `retain_count` — 覆盖源级配置
  - `[project.file]` / `[project.mysql]` / `[project.pgsql]` — 类型专属配置

## 调试命令

```bash
cd rust
cargo run -- --config config.toml
```

## 构建

```bash
cd rust
cargo build --release
```

Windows 下若需静态编译（不依赖外部 MSVC 运行时），可在 PowerShell 中执行：

```powershell
$env:RUSTFLAGS='-C target-feature=+crt-static'
cargo build --release
```

## 定时任务

手动测试通过后，可加入系统定时任务：

- **Linux**: `crontab -e` 添加一行
- **Windows**: 使用"任务计划程序"创建基本任务
