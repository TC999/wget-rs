mod cli;
mod download;

use cli::parse_args;
use download::download_file;

/// Parses command-line arguments and downloads a file from a specified URL using multiple threads.
///
/// Prints an error message to standard error if the download fails.
///
/// # Examples
///
/// ```
/// // Run the program from the command line with appropriate arguments:
/// // cargo run -- <URL> <OUTPUT_PATH> --threads <N>
/// ```
fn main() {
    let args = parse_args();
    if let Err(e) = download_file(&args.url, &args.output, args.threads) {
        eprintln!("下载失败: {}", e);
    }
}