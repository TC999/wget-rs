# wget-rs

[![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)

## 项目简介

**wget-rs** 是一个用 Rust 语言重写的类 wget 命令行工具。它旨在提供高性能、安全且易于使用的文件下载能力，适合需要跨平台、现代化下载体验的开发者和终端用户。

## 功能特性

- 支持 HTTP/HTTPS 协议下载
- 多线程或异步下载（如有实现）
- 断点续传（如有实现）
- 支持自定义请求头
- 下载进度显示
- 跨平台支持（Windows/Linux/macOS）

## 安装方法

### 使用 Cargo

```bash
cargo install wget-rs
```

或者克隆本仓库后手动编译：

```bash
git clone https://github.com/TC999/wget-rs.git
cd wget-rs
cargo build --release
```

可执行文件位于 `target/release/wget-rs`。

## 使用方法

```bash
wget-rs [选项] <URL>
```

### 示例

```bash
wget-rs https://example.com/file.zip
```

### 常用选项

- `-O, --output <文件名>` 指定输出文件名
- `-c, --continue`      断点续传
- `-h, --help`          查看帮助信息

## 贡献指南

欢迎贡献代码或提出建议！请提交 Pull Request 或 Issue。

1. Fork 本仓库
2. 新建分支进行开发
3. 提交 Pull Request

## 许可证

本项目采用 MIT 许可证，详情见 [LICENSE](./LICENSE)。

## 致谢

感谢 Rust 社区和 wget 项目的启发。
