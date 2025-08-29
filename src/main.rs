use lazyman_trading::{
    core::{run, verify},
    std_error_exit,
};

#[tokio::main]
async fn main() {
    // Flags Skipping for Imperative Design
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Imperative Choices
    match args.first().map(String::as_str) {
        Some("verify") => verify().await,
        Some("run") => run().await,
        Some(_) => {
            println!("help message")
        }
        None => std_error_exit!("Command not Found"),
    }
}
