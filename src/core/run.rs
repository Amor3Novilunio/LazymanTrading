use crate::{
    core::request::{funding, types::RequestProps},
    models::token::Token,
};
use dialoguer::Select;
use std::path::Path;

pub async fn run(props: RequestProps) {
    // sends a request to wallet and check what tokens do we have
    let result = funding::funding(RequestProps {
        binance_api_collection: props.binance_api_collection,
        env: props.env,
    })
    .await;

    // shows an option where you can select which token the bot will be used for
    let mut asset_collection: Vec<String> = Vec::new();

    for funding in result.as_array().unwrap() {
        asset_collection.push(funding["asset"].as_str().unwrap().to_string());
    }

    let selection = Select::new()
        .with_prompt("Pick A Token Where the Bot Will Run")
        .items(&asset_collection)
        .default(0)
        .interact()
        .unwrap();

    let selected_token = &asset_collection[selection];

    // after selecting we need to make a sure a configuration in token folder exist
    let token_path_string = format!("./token/{}.json", selected_token.to_uppercase());
    let token_path = Path::new(&token_path_string);

    // let mut token = Token {
    //     origin_price: 800.0,
    //     sell_percentage: 2.0,
    //     buy_percentage: 5.0,
    //     buy_origin_price: 160.0,
    //     limiter: 200.0,
    //     wallet_usdt: 5.0,
    // };

    if token_path.is_file() {
        return;
        // Collect the configuration FILE HERE
    } else {
        // create file HERE
        // file does not exist
    
    }
    // after finding the correct configuration
    //
    //
    // it will loop on spot trading request
    //
    // monitor the selected TOKEN in USDT
    // if the token increased from the origin price when you invested and reached the sell percentage thresh hold : run sell logic
    // apply the sell percentage (check diagram for reference) 
    // if the token decreased from the origin price when you invested and reached the buy percentage thresh hold : run buy logic -> apply limit logic
    //  
}
