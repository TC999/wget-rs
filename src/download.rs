use std::fs::File;
use std::io::copy;
use reqwest::blocking::get;

pub fn download_file(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = get(url)?;
    let mut dest = File::create(output)?;
    copy(&mut response, &mut dest)?;
    println!("下载完成: {}", output);
    Ok(())
}