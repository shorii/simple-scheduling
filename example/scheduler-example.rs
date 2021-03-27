use scheduling::scheduling;
use async_stream;
use tokio;
use futures;

#[scheduling(5)]
fn foo() {
    println!("hoge");
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    foo()?;
    Ok(())
}
