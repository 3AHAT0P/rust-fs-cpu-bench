mod cpu_test;
mod fs_test;

#[tokio::main]
async fn main() {
  cpu_test::run_benchmark().await;
}
