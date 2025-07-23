mod cli;
mod download;
mod hash;

use cli::parse_args;
use download::download_file;
use hash::{calculate_all_hashes, display_hash_results, verify_and_display};

fn main() {
    let args = parse_args();
    
    // 执行下载
    if let Err(e) = download_file(&args.url, &args.output, args.threads) {
        eprintln!("下载失败: {}", e);
        std::process::exit(1);
    }
    
    // 确定下载的文件名
    let filename = match &args.output {
        Some(name) => name.clone(),
        None => {
            // 从URL推断文件名（与download.rs中的逻辑保持一致）
            args.url.split('/')
                .last()
                .filter(|s| !s.is_empty())
                .unwrap_or("output")
                .to_string()
        }
    };
    
    // 处理哈希相关功能
    if args.hash || args.verify_hash.is_some() {
        if let Some(expected_hash) = args.verify_hash {
            // 验证哈希值
            if let Err(e) = verify_and_display(&filename, &expected_hash) {
                eprintln!("哈希验证失败: {}", e);
                std::process::exit(1);
            }
        } else if args.hash {
            // 计算并显示所有哈希值
            match calculate_all_hashes(&filename) {
                Ok(results) => display_hash_results(&results, &filename),
                Err(e) => {
                    eprintln!("哈希计算失败: {}", e);
                    std::process::exit(1);
                }
            }
        }
    }
}