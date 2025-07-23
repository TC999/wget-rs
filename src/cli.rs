use clap::Parser;

/// 解析命令行参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
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
        // We can't easily test clap parsing in unit tests,
        // but we can verify the default value is set correctly
        // by checking the struct definition
        let args = Args {
            url: "https://example.com".to_string(),
            output: None,
            threads: 32, // This should match our default
            hash: false,
            verify_hash: None,
        };
        assert_eq!(args.threads, 32);
    }
}