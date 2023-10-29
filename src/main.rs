use std::process::Stdio;
use futures::StreamExt; // 0.3.1
use tokio::{io::BufReader, prelude::*, process::Command}; // 0.2.4, features = ["full"]

#[tokio::main]
async fn main() {
    let mut cmd = Command::new("/tmp/slow.bash")
        .stdout(Stdio::piped()) // Can do the same for stderr
        .spawn()
        .expect("cannot spawn");

    let stdout = cmd.stdout().take().expect("no stdout");
    // Can do the same for stderr

    // To print out each line
    // BufReader::new(stdout)
    //     .lines()
    //     .for_each(|s| async move { println!("> {:?}", s) })
    //     .await;

    // To print out each line *and* collect it all into a Vec
    let result: Vec<_> = BufReader::new(stdout)
        .lines()
        .inspect(|s| println!("> {:?}", s))
        .collect()
        .await;

    println!("All the lines: {:?}", result);
}