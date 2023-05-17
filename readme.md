## Alpaca Trader

This started as my Rust learning project four+ years ago, but was abandoned until recently for Rust work 
employer at the time didn't want open-sourced. The work on this of the last few months comprises mostly my 
attempts to remember best practices I've learned, mostly from Rust books. This project is also the 
rudimentary foundation for Alpaca trading bot I also am not open-sourcing, since perhaps it'll make me five 
dollars some day. For now this does two things:  
1. Collects websocket data from Alpaca
2. Provides an Actix/Handlebars front end to the trade data in my postgres database with minimal authentication capability.

Technologies used:
1. Rust
2. Actix
3. Postgresql
4. Sqlx
5. Docker (no compose)
6. Bash/Make

Currently the not-shown trading client polls the Postgres database (described here by the sqlx migrations). It 
takes actions via the Alpaca API based on data as it appears in the database. In the near-term roadmap I will 
implement RabbitMQ to stream the websocket data to both the database for persistence and longer-term analysis, 
and also directly to the trading client.

Originally these two "services" were one monolithic multithreaded app with crossbeam communications 
(see alpaca-collector repo). It was fun to try to figure out how to get Tokio to run async operations in 
multiple threads, but it got old quickly; mostly unnecessarily coupling because I could, not because it was
a great idea. This at least lets me work on parts separately without bringing down the whole thing while 
Rust compiles or whatever.

