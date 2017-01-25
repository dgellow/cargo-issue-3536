use std::time::Duration;
use std::thread::sleep;

fn main() {
    println!("dependency C: build.rs: Waiting 5 seconds");
    sleep(Duration::from_secs(5));
}
