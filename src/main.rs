use lazyman_trading::{
    core::{request::types::RequestProps, run, verify},
    modules::env,
    runtime::functions::{JoinStringProps, join_string},
    std_error_exit,
};

#[tokio::main]
async fn main() {
    // get the config.json data
    // get the token Data

    // IMPERATIVE APPROACH FOR FASTER DEVELOPMENT
    // use Flags

    // FLAGS
    // lmt verify -> secret key verification

    // Flow

    // FLOW PROCESS
    // 1
    // verify the secret key
    // verification success -> failure to do so triggers a panic
    // 2
    // give the user a list option to which token the bot will run
    // these list of options are regarding on what token you have in your account
    // and how much do you have and how much you invested to them
    // 3
    // after picking which token the bot will run

    // Flags Skipping for Imperative Design
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Load The Env
    let env = env::load_env();

    // Convert the api collection to array of Strings
    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.binance_api.clone(),
    });

    // Imperative Choices
    match args.first().map(String::as_str) {
        Some("verify") => {
            verify(RequestProps {
                binance_api_collection,
                env,
            })
            .await;
        }
        Some("run") => {
            // Initialize Run
            run(RequestProps {
                binance_api_collection,
                env,
            })
            .await;   
        }
        Some(_) => {
            println!("help message")
        }
        None => std_error_exit!("Command not Found"),
    }
}
