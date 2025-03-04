# Arbitrage Bot

This project is an arbitrage bot that checks for price differences between Binance and Jupiter for the SOL/USDT trading pair. It identifies arbitrage opportunities and prints them to the console.

## Project Structure

- `src/main.rs`: The main Rust file containing the logic for fetching prices and checking for arbitrage opportunities.

## Dependencies

- `reqwest`: For making HTTP requests.
- `serde`: For deserializing JSON responses.
- `serde_derive`: For deriving serialization and deserialization traits.

## How to Run

1. Ensure you have Rust installed. If not, you can install it from [here](https://www.rust-lang.org/tools/install).
2. Clone this repository.
3. Navigate to the project directory.
4. Run the following command to build and run the project:

    ```sh
    cargo run
    ```

The bot will start fetching prices every 10 seconds and print any arbitrage opportunities it finds.

## License

This project is licensed under the MIT License.
