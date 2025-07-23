use std::fs::File;
use std::io::{self, Write, Read};
use std::path::Path;
use reqwest::blocking::get;
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, HeaderMap};
use regex::Regex;
use indicatif::{ProgressBar, ProgressStyle};

fn extract_filename_from_headers(headers: &HeaderMap) -> Option<String> {
    if let Some(disposition) = headers.get(CONTENT_DISPOSITION) {
        let disposition_str = disposition.to_str().ok()?;
        let re = Regex::new(r#"filename\*?=(?:UTF-8'')?["']?([^;"']+)["']?"#).unwrap();
        if let Some(cap) = re.captures(disposition_str) {
            return Some(cap[1].to_string());
        }
    }
    None
}

fn extract_filename_from_url(url: &str) -> String {
    url.split('/')
        .last()
        .filter(|s| !s.is_empty())
        .unwrap_or("output")
        .to_string()
}

pub fn download_file(url: &str, output: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let headers = response.headers().clone();

    let filename = match output {
        Some(name) => name.clone(),
        None => extract_filename_from_headers(&headers)
            .or_else(|| Some(extract_filename_from_url(url)))
            .unwrap(),
    };

    let total_size = headers
        .get(CONTENT_LENGTH)
        .and_then(|len| len.to_str().ok())
        .and_then(|len| len.parse().ok())
        .unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {percent}% {eta}")
        .unwrap()
        .progress_chars("##-"));

    let mut dest = File::create(&filename)?;
    let mut buffer = [0; 8192];
    let mut downloaded: u64 = 0;

    loop {
        let n = response.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        dest.write_all(&buffer[..n])?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("下载完成!");
    println!("文件保存为: {}", filename);
    Ok(())
}