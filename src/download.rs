use std::fs::{File, OpenOptions};
use std::io::{Seek, SeekFrom, Write, Read};
use std::sync::{Arc, Mutex};

use reqwest::blocking::Client;
use reqwest::header::{CONTENT_LENGTH, RANGE, ACCEPT_RANGES};
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};

const CHUNK_SIZE: u64 = 4 * 1024 * 1024; // 4MB

/// 多线程下载文件
pub fn download_file_multithread(url: &str, output: &str, threads: usize) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    // 获取文件大小与是否支持分片
    let head = client.head(url).send()?;
    let total_size = head
        .headers()
        .get(CONTENT_LENGTH)
        .ok_or("无法获取文件大小")?
        .to_str()?
        .parse::<u64>()?;
    let accept_ranges = head
        .headers()
        .get(ACCEPT_RANGES)
        .map(|v| v == "bytes")
        .unwrap_or(false);

    if !accept_ranges {
        return Err("服务器不支持多线程下载".into());
    }

    // 预分配目标文件
    let file = File::create(output)?;
    file.set_len(total_size)?;

    let arc_file = Arc::new(Mutex::new(
        OpenOptions::new().write(true).read(true).open(output)?
    ));

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {percent}% {eta}")
            .unwrap()
            .progress_chars("##-"),
    );

    // 生成所有分片
    let mut ranges = Vec::new();
    let mut start = 0;
    while start < total_size {
        let end = std::cmp::min(start + CHUNK_SIZE - 1, total_size - 1);
        ranges.push((start, end));
        start += CHUNK_SIZE;
    }

    // 多线程并行下载
    ranges.par_iter().for_each(|&(start, end)| {
        let mut resp = client
            .get(url)
            .header(RANGE, format!("bytes={}-{}", start, end))
            .send()
            .expect("下载分片失败");
        let mut buf = Vec::new();
        resp.read_to_end(&mut buf).expect("读取分片失败");
        let mut f = arc_file.lock().unwrap();
        f.seek(SeekFrom::Start(start)).expect("定位失败");
        f.write_all(&buf).expect("写入分片失败");
        pb.inc((end - start + 1) as u64);
    });

    pb.finish_with_message("下载完成!");
    println!("文件保存为: {}", output);
    Ok(())
}