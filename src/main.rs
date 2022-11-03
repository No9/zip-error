use std::fs::File;
use std::process;
use zip::write::FileOptions;
use zip::ZipWriter;
use std::io;
use std::io::prelude::*;

fn main() -> Result<(), anyhow::Error> {
    // Create the base zip file that we are going to put everything into
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o444)
        .large_file(true);

    let filename = "test.zip";

    let file = match File::create(filename) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to create file: {}", e);
            process::exit(1);
        }
    };
    let mut zip = ZipWriter::new(&file);
    match zip.start_file("new_test.file", options) {
        Ok(v) => v,
        Err(e) => println!("Error starting core file \n{}", e),
    };

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut data = [0; 16384];

    while let Ok(n) = stdin.read(&mut data) {
        if n == 0 {
            break;
        }
        match zip.write(&data) {
            Ok(v) => v,
            Err(e) => {
                println!("Error writing core file \n{}", e);
                process::exit(1);
            }
        };
    }

    zip.flush()?;
    zip.finish()?;
    process::exit(0);
}
