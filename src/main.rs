use lazyman_trading::{
    modules::{env, rest},
    runtime::functions::{JoinStringProps, join_string},
    std_error_exit
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
    // lmt verify
    // checks the secret key if its verified maybe run a wallet check and check what the response is

    // lmt run -> initial operation
    // sends a request to wallet and check what tokens do we have
    // shows an option where you can select which token the bot will be used for
    // after selecting we need to make a sure a configuration in token folder exist the structure should be
    // filename = the token name the data inside are the configuration standard structure
    // after finding the correct configuration
    // algorithm will run
    // it will loop on spot trading request
    // and this is where our logic begins it will all rely on the spot trading request

    // lmt wallet list -> wallet list shows you a list of wallet

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
    //

    // check for flags
    let args: Vec<String> = std::env::args().skip(1).collect();

    let env = env::load_env();

    let binance_api_collection = join_string(JoinStringProps {
        split_prefix: ",".to_string(),
        string_collection: env.binance_api,
    });

    match args.first().map(String::as_str) {
        Some("verify") => {
            
            let mut count = binance_api_collection.len();
            
            for api_url  in binance_api_collection {
                // send request here
                // if request is successful break the loop
                // if request fails proceed to the next api
                // if all api fails return proper error
            }
            
            rest::get::request(rest::get::GetProps {
                url: "".to_string(),
                params:None
            })
            .await;

            println!("verifies token")
        }
        Some("roi") => {
            println!("returns an roi result regarding on what changed")
        }
        Some("run") => {
            println!("sends request and check what tokens you have to offer")
        }
        Some(_) => {
            println!("help message")
        }
        None => std_error_exit!("Command not Found"),
    }
}
