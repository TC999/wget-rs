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
}

pub fn parse_args() -> Args {
    Args::parse()
}