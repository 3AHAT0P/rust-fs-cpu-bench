mod cpu_test;
mod fs_test;

// https://docs.rs/tokio/latest/tokio/runtime/index.html
fn main() {
  cpu_test::run_benchmark();
}
