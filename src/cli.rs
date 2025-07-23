use clap::Parser;

/// wget-rs：一个现代 Rust 版多线程命令行下载器
#[derive(Parser, Debug)]
#[command(
    author = "TC999 <your_email@example.com>",
    version,
    about = "Rust 实现的多线程命令行下载工具，支持断点续传、哈希校验等功能。",
    long_about = r#"wget-rs
========

这是一个现代化的命令行下载器，采用 Rust 编写，具备以下特性：

- 支持多线程高速下载（可指定线程数）
- 支持断点续传（服务器支持时自动启用）
- 支持自动推断文件名
- 支持下载完成后文件哈希计算与校验（MD5/SHA1/SHA256/CRC32）
- 兼容 http/https
- 命令行参数简洁易用

作者: TC999
版本: 0.1.0
项目地址: https://github.com/TC999/wget-rs
"#
)]
pub struct Args {
    /// 要下载的 URL
    pub url: String,
    /// 输出文件名（可选，默认从服务器获取或URL推断）
    #[arg(short, long)]
    pub output: Option<String>,
    /// 线程数（默认32）
    #[arg(short, long, default_value = "32")]
    pub threads: u32,
    /// 下载完成后计算文件哈希值
    #[arg(long)]
    pub hash: bool,
    /// 验证下载文件的哈希值（格式：MD5、SHA1、SHA256或CRC32）
    #[arg(long, value_name = "HASH")]
    pub verify_hash: Option<String>,
}

pub fn parse_args() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_threads() {
        let args = Args {
            url: "https://example.com".to_string(),
            output: None,
            threads: 32,
            hash: false,
            verify_hash: None,
        };
        assert_eq!(args.threads, 32);
    }
}
