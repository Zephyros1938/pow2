use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    println!("Welcome to pow2!");

    let mut times_input = String::new();
    io::stdin()
        .read_line(&mut times_input)
        .expect("Failed to read line");
    let times: u32 = times_input.trim().parse().expect("Invalid input");

    let required_length = ((times as f64) * 0.30103).ceil() as usize + 1;

    let mut set = vec![0u8; required_length];
    set[required_length - 1] = 2;

    let timer = Instant::now();

    let mut file = FileWriter::new(
        format!(
            "OUTPUT_{}_{times:04x}.bin",
            chrono::Local::now().format("%Y-%m-%d_%H:%M:%S").to_string()
        )
        .as_str(),
    )
    .unwrap();

    for n in 0..times {
        for x in 0..set.len() {
            set[x] *= 2;
        }

        // Process carries from right to left
        for idx in (0..set.len()).rev() {
            if set[idx] >= 10 {
                let carry = set[idx] / 10;
                set[idx] %= 10;
                if idx == 0 {
                    // Prepend the carry if it exceeds the most significant digit
                    set.insert(0, carry);
                } else {
                    set[idx - 1] += carry;
                }
            }
        }
        // file.write(format!(
        //     "{:08} (0x{:08x}) : {}\n",
        //     n,
        //     n,
        //     set.iter().map(|&x| x.to_string()).collect::<String>()
        // ))?;
        file.write_bytes(set.as_slice())?;
    }

    file.flush()?;

    let elapsed = timer.elapsed();
    println!("Elapsed time: {:?}", elapsed);
    Ok(())
}

pub struct FileWriter {
    f: BufWriter<File>,
}

impl FileWriter {
    pub fn new(file_path: &str) -> io::Result<FileWriter> {
        let file = File::create(file_path)?;
        let f = BufWriter::new(file);
        Ok(FileWriter { f })
    }

    pub fn write_str(&mut self, data: &str) -> io::Result<()> {
        self.f.write_all(data.as_bytes())
    }

    pub fn write(&mut self, data: String) -> io::Result<()> {
        self.f.write_all(data.as_bytes())
    }

    pub fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        self.f.write_all(data)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.f.flush()
    }
}
