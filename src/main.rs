use lazyman_trading::modules::json::read;

fn main() {
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
}
