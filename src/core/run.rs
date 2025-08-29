use dialoguer::Select;
use rand;
use std::fs;
use std::io::Write;
use std::{collections::HashMap, path::Path};
use tokio::time::{Duration, sleep};

use crate::core::request::convert;
use crate::{
    core::request::{spot_trading::market_price_ticker, wallet::funding_wallet},
    models::token::Token,
    modules::json,
    std_error_exit,
};

pub async fn run() {
    // sends a request to wallet and check what tokens do we have
    let result = funding_wallet(None).await;

    // shows an option where you can select which token the bot will be used for
    let mut asset_collection: Vec<String> = Vec::new();
    let mut asset_free: HashMap<String, f64> = HashMap::new();

    for funding in result.as_array().unwrap() {
        let asset = funding["asset"].as_str().unwrap().to_string();
        // Skip usdt Collection
        if asset != "USDT" {
            // for token selection
            asset_collection.push(asset.clone());
        }

        // collecting your funding data
        let free = match funding["free"].as_str().unwrap_or("0.0").parse::<f64>() {
            Ok(res) => res,
            Err(err) => std_error_exit!(format!("Failed To Deserialize | ERR: {err}")),
        };

        asset_free.insert(asset, free);
    }

    let selection = Select::new()
        .with_prompt("Pick A Token Where the Bot Will Run")
        .items(&asset_collection)
        .default(0)
        .interact()
        .unwrap();

    let selected_token = &asset_collection[selection];
    let selected_token_formatted = format!("{}USDT", selected_token);

    // after selecting we need to make a sure a configuration in token folder exist
    let token_path_string = format!("./token/{}.json", selected_token.to_uppercase());
    let token_path = Path::new(&token_path_string);

    // file location | change path to constant later
    let file_path = format!("./token/{}.json", selected_token);
    let buy_amount_file_path = format!("./token/buy_amount/{}", selected_token);

    let token: Token = {
        // Create Directory if it does not exist
        fs::create_dir_all("./token").unwrap();
        fs::create_dir_all("./token/buy_amount").unwrap();

        // check if file exist
        if token_path.is_file() {
            println!("File Detected");
            let result = json::read::<Token>(&file_path);

            // file configuration check
            if result.origin_price <= 0.0 {
                std_error_exit!("-- File is not Configured --");
            }

            result
        } else {
            // create the file
            let json_config = Token {
                buy_percentage: 0.0, 
                buy_price: 0.0,
                limiter: 0.0,
                origin_price: 0.0,
                sell_percentage: 0.0,
            };

            json::create::<Token>(&file_path, &json_config);

            // !! transfer to runtime later
            // create file

            let mut file = match fs::File::create(&buy_amount_file_path) {
                Ok(res) => res,
                Err(err) => std_error_exit!(format!("Failed To Create File | ERR: {err}")),
            };

            // write data in the file
            match file.write_all(b"0.0") {
                Ok(_) => {
                    println!(" ---- File Created : {}", file_path);
                    println!(" ---- File Created : {}", buy_amount_file_path);
                    println!(
                        " ---- Kindly check the readme to know what each parameter configuration is for  ---- "
                    );
                    std_error_exit!(
                        " ---- File Have Been Created Please Configure it Before Proceeding  ---- "
                    );
                }
                Err(err) => std_error_exit!(format!("Failed To Create File | ERR: {err}")),
            }
        }
    };

    // it will loop on Funding request
    loop {
        // Collect the buy_amount data first
        // To keep our origin point up to date

        let read_buy_amount = match fs::read_to_string(&buy_amount_file_path) {
            Ok(res) => res,
            Err(err) => std_error_exit!(format!("Failed To Create File | ERR: {err}")),
        };

        match read_buy_amount.as_str().parse::<f64>() {
            Ok(buy_amount) => {
                // ------------
                // Variables
                // ------------

                let new_origin_price = token.origin_price - buy_amount;

                println!("Investment : {}", new_origin_price);

                // open the buy_amount file in buffer
                let mut file = match fs::File::open(&buy_amount_file_path) {
                    Ok(res) => res,
                    Err(err) => {
                        std_error_exit!(format!("Failed To Create File | ERR: {err}"))
                    }
                };

                // Collect Snapshots of the update on the Selected Token Value
                let result = market_price_ticker(selected_token_formatted.clone()).await;

                // ------------
                // Sell Flow
                // ------------

                // Selected Token Amount
                let mut current_price: f64 = 0.0;

                // get the current price data based on the selected token
                for price_ticker in result.as_array().unwrap() {
                    current_price = match price_ticker["price"]
                        .as_str()
                        .unwrap_or("0.0")
                        .parse::<f64>()
                    {
                        Ok(res) => res,
                        Err(err) => std_error_exit!(format!("Failed To Deserialize | ERR: {err}")),
                    };
                }

                // calculate the token amount to get the USDT value
                let token_in_usdt = current_price * asset_free[&asset_collection[selection]];

                // Current Value in USDT
                println!("Current {} Price : {}", selected_token, token_in_usdt);

                // Sell Percentage Calculation
                let sell_value_in_usdt = (token.sell_percentage / 100.0) * new_origin_price;
                // when it reaches the quota it will skim the profits and returns you back to the original investment value in USDT
                let sell_quota = sell_value_in_usdt + new_origin_price;

                println!(
                    "Sell Quota : {} | in USDT : ${}",
                    sell_quota, sell_value_in_usdt
                );

                // Sell Algo
                if token_in_usdt >= sell_quota {
                    // drops you back to your original investment size
                    let sell_value = token_in_usdt - sell_value_in_usdt;

                    // the data that will be passed to convert
                    let skimming_profit =
                        format!("{:.4}", token_in_usdt - sell_value).parse().unwrap();

                    // Convert Request
                    let result = convert::send_quote(
                        "USDT".to_string(),
                        selected_token.to_string(),
                        Some(skimming_profit),
                        None,
                    )
                    .await;

                    // to mimic human behavior not fixed request
                    let random_request_sequence = rand::random_range(4..8);
                    sleep(Duration::from_secs(random_request_sequence)).await;

                    // Accept Request
                    let quote_id = result["quoteId"].as_str().unwrap().to_string();
                    convert::accept_quote(quote_id).await;

                    // after we sell we update the buy_amount by ( buy_amount - token.buy_price ) if its 0.0 do not proceed
                    // write/update/replace_all the buy Amount
                    if buy_amount != 0.0 {
                        // update the value of the buy_amount after the buy sequence is done
                        let new_buy_amount = buy_amount - token.buy_price;

                        match file.write_all(new_buy_amount.to_string().as_bytes()) {
                            Ok(_) => {}
                            Err(err) => {
                                std_error_exit!(format!("Failed To Create File | ERR: {err}"))
                            }
                        }
                    }
                }

                // ------------
                // Buy Flow
                // ------------

                // Buy Percentage
                let buy_value_in_usdt = (token.buy_percentage / 100.0) * new_origin_price;

                // buy quota triggers the buy logic when its true
                let buy_quota = new_origin_price - buy_value_in_usdt;

                println!(
                    "Buy Quota : {} | in USDT : ${}",
                    buy_quota, buy_value_in_usdt
                );

                // limiter expression
                if token.limiter > buy_amount {
                    // buy percentage expression
                    if token_in_usdt <= buy_quota {
                        // Convert Request
                        let result = convert::send_quote(
                            selected_token.to_string(),
                            "USDT".to_string(),
                            None,
                            Some(token.buy_price),
                        )
                        .await;

                        // to mimic human behavior not fixed request
                        let random_request_sequence = rand::random_range(4..8);
                        sleep(Duration::from_secs(random_request_sequence)).await;

                        // Accept Request
                        let quote_id = result["quoteId"].as_str().unwrap().to_string();
                        convert::accept_quote(quote_id).await;

                        // update the value of the buy_amount after the buy sequence is done
                        let new_buy_amount = buy_amount + token.buy_price;

                        // write/update/replace_all the buy Amount
                        match file.write_all(new_buy_amount.to_string().as_bytes()) {
                            Ok(_) => {}
                            Err(err) => {
                                std_error_exit!(format!("Failed To Create File | ERR: {err}"))
                            }
                        }
                    }
                }
            }
            Err(err) => std_error_exit!(format!("Failed To Deserialize | ERR: {err}")),
        }

        println!("-----------------------");

        // send request interval
        sleep(Duration::from_secs(5)).await;
    }
}
