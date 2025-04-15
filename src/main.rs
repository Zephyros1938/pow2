use std::fs::{File, create_dir_all};
use std::io::{self, BufWriter, Write};
use std::time::Instant;

const POWER: u8 = 2;
const MODSTEP: u8 = 10;
const ONE_USIZE: usize = 1;
const ONE_HUNDRED_F64: f64 = 100.0;
const ROUNDING_POINT_F64: f64 = 1000.0;

fn main() -> io::Result<()> {
    println!("\n\n\tWelcome to pow2!\n\t\tMade by Zephyros1938\n");

    let mut display_status = false;
    let mut times: usize = 0;
    let mut step_div: usize = 8;

    for arg in std::env::args().skip(1) {
        if let Some(val) = arg.strip_prefix("-t=") {
            times = val.parse().expect("Could not parse times");
        } else if let Some(val) = arg.strip_prefix("-s=") {
            step_div = val.parse().expect("Could not set step_div");
        } else if arg == "--clear" {
            for entry in std::fs::read_dir("./output")? {
                let path = entry?.path();
                std::fs::remove_file(&path)?;
                println!("File Deleted: {}", path.display());
            }
        } else if arg == "--display" {
            display_status = true;
        }
    }

    create_dir_all("./output")?;

    let required_length = ((times as f64) * 0.30103).ceil() as usize + 1;
    let mut set = vec![0u8; required_length];
    set[required_length - 1] = POWER;
    println!("Buffer Length: {0:08x} ({0:08})", set.len());

    let timer = Instant::now();
    let mut last_time = timer.elapsed();

    //TODO: Implement writeln!
    let file_name = format!(
        "output/OUTPUT_{}_{:04x}.bin",
        chrono::Local::now().format("%Y-%m-%d_%H:%M:%S"),
        times
    );
    let mut file = FileWriter::new(&file_name)?;

    let update_step = if times / step_div < step_div {
        16
    } else {
        times / step_div
    };

    for n in 1..=times {
        let mut carry = 0u8;
        for digit in set.iter_mut().rev() {
            let prod = *digit * POWER + carry;
            *digit = prod % MODSTEP;
            carry = prod / MODSTEP;
        }
        if carry > 0 {
            set.insert(0, carry);
        }

        let digits: String = set.iter().map(|&d| d.to_string()).collect();
        file.write_str(&format!("{:08} (0x{:08x}) : {}\n", n, n, digits))?;

        if display_status && n % update_step == 0 {
            let percent = ((n as f64 / times as f64) * ONE_HUNDRED_F64 * ROUNDING_POINT_F64)
                .round()
                / ROUNDING_POINT_F64;
            println!(
                "[0x{:08x}] ({:>7}%)\ttime: [{:?}]\tcalc: [{:?}]",
                n,
                percent,
                timer.elapsed(),
                timer.elapsed() - last_time
            );
            last_time = timer.elapsed();
        }
    }

    file.flush()?;
    println!("Total elapsed time: {:?}", timer.elapsed());
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

    pub fn write_bytes(&mut self, data: &[u8]) -> io::Result<()> {
        self.f.write_all(data)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.f.flush()
    }
}
