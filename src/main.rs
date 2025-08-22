use lazyman_trading::{
    core::request::verification,
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
            verification::account_status(verification::AccountStatusProps {
                binance_api_collection,
                env,
            })
            .await
        }
        // Some("wallet") => {
        // sub commands [ list , funding, spot ]
        //     println!("returns an roi result regarding on what changed")
        // }
        // Some("roi") => {
        //     println!("returns an roi result regarding on what changed")
        // }
        Some("run") => {
            println!("sends request and check what tokens you have to offer")
            // lmt run -> initial operation
            // 
            // sends a request to wallet and check what tokens do we have
            // 
            // shows an option where you can select which token the bot will be used for
            // 
            // after selecting we need to make a sure a configuration in token folder exist the structure should be
            // 
            // filename = the token name the data inside are the configuration standard structure
            // 
            // after finding the correct configuration
            // 
            // algorithm will run
            // 
            // it will loop on spot trading request
            // 
            // and this is where our logic begins it will all rely on the spot trading request
        }
        Some(_) => {
            println!("help message")
        }
        None => std_error_exit!("Command not Found"),
    }
}
