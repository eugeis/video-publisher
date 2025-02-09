use reqwest::blocking::{Client, Body};
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::{self, Read};
use anyhow::Result;

pub fn upload_with_progress(api_url: &str, file_path: &str, headers: Vec<(&str, &str)>) -> Result<()> {
    let client = Client::new();
    let mut file = File::open(file_path)?;
    
    let metadata = file.metadata()?;
    let total_size = metadata.len();
    
    let pb = ProgressBar::new(total_size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{wide_bar} {bytes}/{total_bytes} ({eta})")?
        .progress_chars("=>-"));
    
    let mut buffer = Vec::new();
    let mut uploaded_size = 0;
    
    while let Ok(n) = file.by_ref().take(8192).read_to_end(&mut buffer) {
        if n == 0 { break; }
        uploaded_size += n as u64;
        pb.set_position(uploaded_size);
    }
    
    let body = Body::from(buffer);
    let mut req = client.post(api_url).body(body);
    
    for (key, value) in headers {
        req = req.header(key, value);
    }
    
    let res = req.send()?;
    
    pb.finish_with_message("Upload complete");
    println!("Upload response: {:?}", res.text()?);
    
    Ok(())
}
