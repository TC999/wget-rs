mod download;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("用法: {} <url> <保存文件名> [线程数]", args[0]);
        return;
    }
    let url = &args[1];
    let output = &args[2];
    let threads = if args.len() > 3 {
        args[3].parse().unwrap_or(4)
    } else {
        4
    };

    if let Err(e) = download::download_file_multithread(url, output, threads) {
        eprintln!("下载失败: {}", e);
    }
}