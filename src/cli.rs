use clap::Parser;

/// 解析命令行参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// 要下载的 URL
    pub url: String,
    /// 输出文件名
    #[arg(short, long, default_value = "output")]
    pub output: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}