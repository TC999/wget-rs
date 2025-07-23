mod cli;
mod download;

use cli::parse_args;
use download::download_file;

fn main() {
    let args = parse_args();
    if let Err(e) = download_file(&args.url, &args.output) {
        eprintln!("下载失败: {}", e);
    }
}
