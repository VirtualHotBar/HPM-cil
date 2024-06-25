use std::{
    fs::File,
    io::{self, Write},
    path::PathBuf,
};

use std::fs::create_dir_all;
use indicatif::{ProgressBar, ProgressStyle};
pub fn decompress_file_7z<F>(
    input_path: &PathBuf,
    output_path: &PathBuf,
    mut progress_callback: F,
    target_pattern: Option<&str>,
) -> io::Result<()>
where
    F: FnMut(usize, usize),
{
    let mut sz = sevenz_rust::SevenZReader::open(input_path.to_str().unwrap(), "".into()).unwrap();

    let total_size: u64 = sz
        .archive()
        .files
        .iter()
        .filter(|f| f.has_stream() && target_pattern.map_or(true, |p| f.name().contains(p)))
        .map(|f| f.size())
        .sum();

    let pb = ProgressBar::new(total_size as u64);
    pb.set_style(ProgressStyle::with_template("/* {spinner:.green}  */[{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .unwrap()
        //.with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#> "));

    let dest =output_path.to_owned();

    let mut processed_size: usize = 0;

    let _ = sz.for_each_entries(|entry,  reader| {
        if target_pattern.is_some() && !entry.name().contains(target_pattern.unwrap()) {
            // comsume the reader to skip the file, even if we don't need it
            //std::io::copy(reader, &mut std::io::sink())?;
            while let Ok(n) = reader.read(&mut [0; 4096]) {
                if n == 0 {
                    break;
                }
            }
        } else {
            let path = dest.join(entry.name());
            create_dir_all(path.parent().unwrap())?;
            let mut file = File::create(&path)?;

            let mut buf = [0u8; 1024];
            let mut uncompressed_size = 0;

            loop {
                let read_size = reader
                    .read(&mut buf)
                .unwrap();
                if read_size == 0 {
                    break;
                }
                file.write_all(&buf[..read_size])?;
                uncompressed_size += read_size;
                pb.set_position(processed_size as u64);
                progress_callback(processed_size + uncompressed_size, total_size as usize);
            }

            processed_size += uncompressed_size;


            if file.metadata()?.len() == 0 {
                println!("Warning: Decompressed file is empty -> {}", entry.name());
            }
        }
        Ok(true)
    });

    //pb.finish_with_message("Done");

    Ok(())
}


