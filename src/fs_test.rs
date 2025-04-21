use std::env;
use std::fs;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::time::Instant;

const MAX_FILE_SIZE_ENV: &str = "MAX_FILE_SIZE";
const SEEK_SKIP_BYTES_ENV: &str = "SEEK_SKIP_BYTES";
const READ_BYTES_ENV: &str = "READ_BYTES";

const WRITE_STRING: &str = "KDtBzffg%;K]r7D#3KcS1,UXZykBJb};v}Tp!q0B0@WU1f*hiaE1SUYExwSVhDXASHZV*.t$vGn2ph?+N=i=/KC?pT[=&Gwk+2:HS=tD!4V8rLD.aZ&TFV:nzMp/.}Qqp%fy)NP50B,,]*XrK4@$7&";
const TEST_DIR: &str = "./fs_test";
const TEST_FILE: &str = "test_file.txt";
const PRINT_WIDHT: usize = 50;

fn write_file() {
  println!("\nWrite file test...");
  let max_file_size = match env::var(MAX_FILE_SIZE_ENV) {
    Ok(n) => n.parse::<u64>().unwrap_or(20_000_000_000),
    Err(_) => 20_000_000_000,
  };

  let mut file_descriptor = fs::File::create(format!("{}/{}", TEST_DIR, TEST_FILE)).unwrap();

  let now = Instant::now();
  loop {
    if file_descriptor.metadata().unwrap().size() >= max_file_size {
      break;
    }

    file_descriptor.write(WRITE_STRING.as_bytes()).unwrap();
  }
  let elapsed = now.elapsed();
  println!(
    "{:.<width$}{:.2?}bytes",
    "Total file size:",
    file_descriptor.metadata().unwrap().size(),
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Total write file runtime:",
    elapsed,
    width = PRINT_WIDHT
  );
}

fn read_file() {
  println!("\nRead file...");
  let mut file_descriptor = fs::File::open(format!("{}/{}", TEST_DIR, TEST_FILE)).unwrap();
  let read_bytes = match env::var(READ_BYTES_ENV) {
    Ok(v) => v.parse::<usize>().unwrap_or(1024),
    Err(_) => 1024,
  };
  let mut buf = vec![0; read_bytes];
  let now = Instant::now();
  let mut read_chunks = 0;
  loop {
    match file_descriptor.read(&mut buf) {
      Ok(n) => {
        read_chunks += 1;
        if n == 0 {
          break;
        }
      }
      Err(e) => panic!("{}", e),
    };
  }
  let elapsed = now.elapsed();
  println!(
    "{:.<width$}{:.2?}",
    format!("Total read chunks by {} bytes:", read_bytes),
    read_chunks,
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Total read file runtime:",
    elapsed,
    width = PRINT_WIDHT
  );
}

fn seek_forward() {
  println!("\nSeek forward file...");
  let skip_bytes = match env::var(SEEK_SKIP_BYTES_ENV) {
    Ok(n) => n.parse::<i64>().unwrap_or(1024),
    Err(_) => 1024,
  };

  let mut file_descriptor = fs::File::open(format!("{}/{}", TEST_DIR, TEST_FILE)).unwrap();
  let mut seek_jumps = 0;

  let file_len = file_descriptor.metadata().unwrap().size();
  let now = Instant::now();

  loop {
    if file_descriptor.seek(SeekFrom::Current(skip_bytes)).unwrap() >= file_len {
      break;
    }
    seek_jumps += 1;
  }

  let elapsed = now.elapsed();
  println!(
    "{:.<width$}{:.2?}",
    "Seek skipping by bytes:",
    skip_bytes,
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Total seek jumps:",
    seek_jumps,
    width = PRINT_WIDHT
  );
  println!(
    "{:.<width$}{:.2?}",
    "Total seek file runtime:",
    elapsed,
    width = PRINT_WIDHT
  );
}

pub fn run_benchmark() {
  println!("\nRunning FS benchmark test");
  let _ = fs::remove_dir_all(TEST_DIR);
  fs::create_dir(TEST_DIR).unwrap();

  write_file();
  read_file();
  seek_forward();
  fs::remove_dir_all(TEST_DIR).unwrap();
}
