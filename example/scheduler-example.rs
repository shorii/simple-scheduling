use async_stream;
use futures;
use scheduling::scheduling;
use tokio;

#[scheduling(5)]
fn foo() {
    println!("hoge");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    foo()?;
    Ok(())
}
