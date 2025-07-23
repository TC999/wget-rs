use std::fs::File;
use std::io::{Write, Read};
use std::thread;
use std::sync::{Arc, Mutex};
use reqwest::blocking::{get, Client};
use reqwest::header::{CONTENT_DISPOSITION, CONTENT_LENGTH, RANGE, ACCEPT_RANGES, HeaderMap};
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

fn supports_range_requests(headers: &HeaderMap) -> bool {
    headers.get(ACCEPT_RANGES)
        .and_then(|v| v.to_str().ok())
        .map(|v| v == "bytes")
        .unwrap_or(false)
}

fn download_chunk(
    client: &Client,
    url: &str,
    start: u64,
    end: u64,
    chunk_data: Arc<Mutex<Vec<u8>>>,
    progress: Arc<Mutex<ProgressBar>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let range_header = format!("bytes={}-{}", start, end);
    let response = client
        .get(url)
        .header(RANGE, range_header)
        .send()?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let mut buffer = Vec::new();
    let mut response_reader = response;
    response_reader.read_to_end(&mut buffer)?;

    {
        let mut data = chunk_data.lock().unwrap();
        *data = buffer;
    }

    {
        let pb = progress.lock().unwrap();
        pb.inc(end - start + 1);
    }

    Ok(())
}

fn download_single_threaded(
    url: &str,
    filename: &str,
    total_size: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {percent}% {eta}")
        .unwrap()
        .progress_chars("##-"));

    let mut dest = File::create(filename)?;
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
    Ok(())
}

pub fn download_file(url: &str, output: &Option<String>, threads: u32) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.head(url).send()?;
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

    // 如果文件大小未知或服务器不支持范围请求，使用单线程下载
    if total_size == 0 || !supports_range_requests(&headers) || threads == 1 {
        println!("使用单线程下载...");
        return download_single_threaded(url, &filename, total_size);
    }

    println!("使用 {} 线程下载，文件大小: {} 字节", threads, total_size);

    let pb = Arc::new(Mutex::new(ProgressBar::new(total_size)));
    {
        let pb_guard = pb.lock().unwrap();
        pb_guard.set_style(ProgressStyle::default_bar()
            .template("{bar:40.cyan/blue} {bytes}/{total_bytes} {percent}% {eta}")
            .unwrap()
            .progress_chars("##-"));
    }

    let chunk_size = total_size / threads as u64;
    
    // If chunk size is too small (less than 1 byte per thread), use single thread  
    if chunk_size == 0 {
        println!("文件太小，使用单线程下载...");
        return download_single_threaded(url, &filename, total_size);
    }
    
    let mut handles = vec![];
    let mut chunk_data = vec![];

    for i in 0..threads {
        let start = i as u64 * chunk_size;
        let end = if i == threads - 1 {
            total_size - 1
        } else {
            (i + 1) as u64 * chunk_size - 1
        };

        let chunk_storage = Arc::new(Mutex::new(Vec::new()));
        chunk_data.push(chunk_storage.clone());

        let client_clone = client.clone();
        let url_clone = url.to_string();
        let pb_clone = pb.clone();

        let handle = thread::spawn(move || {
            download_chunk(&client_clone, &url_clone, start, end, chunk_storage, pb_clone)
        });

        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        match handle.join() {
            Ok(result) => {
                if let Err(e) = result {
                    return Err(format!("下载块失败: {}", e).into());
                }
            }
            Err(_) => {
                return Err("线程 panic".into());
            }
        }
    }

    // 合并所有块到最终文件
    let mut dest = File::create(&filename)?;
    for chunk in chunk_data {
        let data = chunk.lock().unwrap();
        dest.write_all(&data)?;
    }

    {
        let pb_guard = pb.lock().unwrap();
        pb_guard.finish_with_message("下载完成!");
    }
    println!("文件保存为: {}", filename);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_supports_range_requests() {
        let mut headers = HeaderMap::new();
        
        // Test when Accept-Ranges is not present
        assert!(!supports_range_requests(&headers));
        
        // Test when Accept-Ranges is bytes
        headers.insert(ACCEPT_RANGES, "bytes".parse().unwrap());
        assert!(supports_range_requests(&headers));
        
        // Test when Accept-Ranges is none
        headers.insert(ACCEPT_RANGES, "none".parse().unwrap());
        assert!(!supports_range_requests(&headers));
    }

    #[test]
    fn test_extract_filename_from_url() {
        assert_eq!(extract_filename_from_url("https://example.com/file.txt"), "file.txt");
        assert_eq!(extract_filename_from_url("https://example.com/path/to/file.zip"), "file.zip");
        assert_eq!(extract_filename_from_url("https://example.com/"), "output");
        assert_eq!(extract_filename_from_url("https://example.com"), "example.com");
    }

    #[test]
    fn test_extract_filename_from_headers() {
        let mut headers = HeaderMap::new();
        
        // Test when Content-Disposition is not present
        assert!(extract_filename_from_headers(&headers).is_none());
        
        // Test with standard filename
        headers.insert(CONTENT_DISPOSITION, "attachment; filename=\"test.txt\"".parse().unwrap());
        assert_eq!(extract_filename_from_headers(&headers), Some("test.txt".to_string()));
        
        // Test with filename*
        headers.insert(CONTENT_DISPOSITION, "attachment; filename*=UTF-8''test%20file.txt".parse().unwrap());
        assert_eq!(extract_filename_from_headers(&headers), Some("test%20file.txt".to_string()));
    }

    #[test] 
    fn test_chunk_calculation() {
        let total_size = 1000u64;
        let threads = 4u32;
        let chunk_size = total_size / threads as u64;
        
        // Test chunk boundaries
        for i in 0..threads {
            let start = i as u64 * chunk_size;
            let end = if i == threads - 1 {
                total_size - 1
            } else {
                (i + 1) as u64 * chunk_size - 1
            };
            
            // Verify no gaps or overlaps
            if i > 0 {
                let prev_end = (i as u64 * chunk_size) - 1;
                assert_eq!(start, prev_end + 1);
            }
            
            // Verify last chunk goes to the end
            if i == threads - 1 {
                assert_eq!(end, total_size - 1);
            }
        }
    }

    #[test]
    fn test_small_file_edge_case() {
        // Test that very small files would result in chunk_size = 0
        let total_size = 10u64;
        let threads = 32u32;
        let chunk_size = total_size / threads as u64;
        
        // This should be 0, which means we should fall back to single-threaded
        assert_eq!(chunk_size, 0);
    }
}