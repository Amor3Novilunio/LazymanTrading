# Lazyman Trading
A Bot designed for taking advantage of Convert Feature of an Centralized Exchanges

---

## Introduction

This bot was designed for people who want to play with crypto but as a Hoarder not a Trader

### Problem

if you are like me who only buy token and store them in my wallet and convert them back to USDT when its profitable you will know its a hassle to do that because you have to monitor the token if its increasing or decreasing and manually converting when its time to revert to USDT

### Lazyman Solution

  Lazyman is Designed to make that process easier for you what this bot is designed to do is to skim every profit you make converting it to USDT when the configured threshold is triggered and gives you the ability to re-buy when the token goes down to a certain threshold and all that depends on how you configure your token configuration

### Lazyman Env Setup
  | Env                     | Description/Sample
  |-                        |-
  | API_URL_GENERAL         | https://api.binance.com,https://api1.binance.com,https://api2.binance.com,https://api3.binance.com,https://api4.binance.com
  | API_URL_SPOT_TRADING    | https://api.binance.com,https://api-gcp.binance.com,https://api1.binance.com,https://api2.binance.com,https://api3.binance.com,https://api4.binance.com
  | API_KEY                 | Your Exchange Api Key
  | SECRET_KEY              | Your Exchange Secret Key 

### Lazyman Configuration

    Location : ./Token/< TokenName >.json
    | Configuration |Description | Formula | Usage
    |-------------------| - | - | -
    | origin_price | The price of the token the moment you bought it in USDT. | USDT | 500.0
    | sell_percentage | The threshold based on how much you want to profit in USDT when the token reaches to that price in USDT | (sell_percentage / 100.0) * new*origin_price | 2.0 is 2%, 20.0% is 20%
    | buy_price | How much you want to buy when the buy_percentage is triggered | USDT | 50.0
    | buy_percentage | The threshold on how much drop in value are you willing to buy in this token again | (token.buy_percentage / 100.0) * new_origin_price | 2.0 is 2%, 20.0% is 20%  
    | limiter | The threshold to stop buy_percentage re-buy when it gets triggered | USDT | 500.0

### Full Circle Process

    ```
        * 1 XRP = 3 USDT
        * We bought 10 XRP = 30 USDT
        * Lazyman config:

        * Sell percentage = 5%
        * Buy percentage = 10%
        * Buy size = 10 USDT each time
        * Limiter = 100 USDT (max reserve to use)
        * We also keep 100 USDT in reserve for future buys

        ---

        ### Sell Flow (when price goes up)

        * If XRP increases enough, Lazyman sells a small piece.
        * Example:

        * Threshold = 30 × 0.05 = 1.5 USDT
        * Lazyman sells 0.5 XRP (≈1.5 USDT)
        * Result:

        * Wallet has +1.5 USDT saved
        * We now hold 9.5 XRP
        * Total value is the same, but now some money is safe in USDT

        ---

        ### Buy Flow (when price drops)

        * If XRP drops by 10%, Lazyman buys more using reserves.
        * Example:

        * Price falls from 3.0 → 2.7 USDT
        * Lazyman spends 10 USDT from reserve
        * Buys ≈3.7 XRP
        * Result:

        * XRP balance increases to 13.7 XRP
        * Reserve decreases to 90 USDT
        * Origin price is updated from 30 → 40 USDT (because we added 10 more)

        ---

        ### Full Cycle Example

        1. Start: 10 XRP = 30 USDT
        2. Price dips → Lazyman buys 3.7 XRP with 10 USDT (now 13.7 XRP)
        3. Price recovers → Lazyman sells 0.67 XRP for 2.0 USDT
        4. End result:

        * XRP left ≈ 13.0 XRP
        * Wallet has +2.0 USDT
        * Reserve = 90 USDT
        * Total value ≈ 41.1 USDT
        * Net gain = +1.1 USDT

        ---

        * **Buy Flow** adds more tokens cheaply when price dips.
        * **Sell Flow** saves a little USDT when price recovers.
        * Over time, this “buy low, sell high” cycle repeats.
        * The limiter stops the bot from spending all your money if price keeps dropping.

    ```

# WARNING 
  The bots lifetime depends on how much token you have
  
  This is currently untested because binance have not yet accepted my converting survey to have access to convert API
  
  Best Used for High Volatility Tokens Like DOGE or PEPE 
  
  but can also be used for BTC or other Tokens it all depends on how you configure the configs


# Future
   Machine Learning Statistical Basis to automate the process of the config's basically will check the previous records and create a highest possible optimal 
   
   configuration for the selected token so that you wont have to worry about manually configuring every time the token value changes