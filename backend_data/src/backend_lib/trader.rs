//! frontend_ui.rs
//!

use crate::models::*;
use chrono::{DateTime, Utc};
/**

    Buy and sell based on messages received from rest and websocket services.

    Considerations:
        - granularity of quote information (ws comes in fast, rest may only be every five seconds)


    -- reqt: hashmap for each symbol (basically reflecting the database)
    -- reqt: for each symbol, need a list of the previous trade prices to compute moving averages


*/
use crossbeam::channel::Sender;
use crossbeam_channel::Receiver;
use rust_decimal::Decimal;
use std::collections::HashMap;
use std::thread;
use std::thread::JoinHandle;

// const EMA_ARRAY:[usize;4] = [Ema::EMA04, Ema::EMA08, Ema::EMA15, Ema::EMA20];

#[derive(Debug)]
pub enum TraderMsg {
    TradeRest(AlpacaTradeRest),
    TradeWs(AlpWsTrade),
    QuoteWs(AlpWsQuote),
}

struct TraderBook {
    _stock_hmap: HashMap<String, StockState>,
}
//
// impl TraderBook {
// 	fn _refresh_moving_averages_for_stock(&mut self, stock_sym:&str) {
//
// 		// 	// loop through the 4, 8, 15, 20 emas
// 		for ema_kind in &EMA_ARRAY {
//
// 			tracing::debug!("[refresh_moving_averages_for_stock] refreshing for ema{}", ema_kind);
//
// 			// 0. compute new moving average; will cause divide by zero on first so don't unwrap
// 			// get the specific stock
// 			let stock_opt =  self.stock_hmap.get(stock_sym);
// 			match stock_opt {
// 				None => {
// 					tracing::debug!("[refresh_moving_averages_for_stock] this stock symbol isn't in the trade book hmap");
// 				},
// 				Some(stock) => {
//
// 					// compute the moving average
// 					let new_moving_avg_decimal_opt = stock.compute_moving_average_x(ema_kind);
// 					tracing::debug!("[refresh_moving_averages_for_stock] historical avg: {:?}", &new_moving_avg_decimal_opt);
// 					match new_moving_avg_decimal_opt{
// 						None => {
// 							tracing::debug!("[refresh_moving_averages_for_stock] this ema option isn't in the trade book hmap (yet)");
// 						},
// 						Some(new_moving_avg_decimal) => {
//
// 							// 1. get the stock
// 							let stock_state_opt = self.stock_hmap.get_mut(stock_sym);
// 							match stock_state_opt {
// 								None => {
// 									tracing::debug!("[refresh_moving_averages_for_stock] this stock doesn't exist in the stock hmap");
// 								}
// 								Some(stock_state) => {
//
// 									// TODO: test purpose only
// 									stock_state.print_ema_hist(ema_kind);
//
// 									// 2. get the ema hmap
// 									let ema_hmap_ref = &mut stock_state.ema_hist_hmap;
//
// 									// 3. get the vector of ema history
// 									let ema_hist_vec_ref_opt = ema_hmap_ref.get_mut(ema_kind);
// 									match ema_hist_vec_ref_opt {
// 										None=>{
// 											tracing::debug!("[refresh_moving_averages_for_stock] a vector of ema history doesn't exist for this ema: {}, creating new vec", ema_kind);
//
// 											// 4a. compute new moving average
// 											// moved to the top of this fn
//
// 											// 4b. create new TimePrice
// 											let new_moving_average = TimePrice{ dt: Utc::now(), price: new_moving_avg_decimal };
//
// 											// create a new vector, add it to the ema history hashmap
// 											let mut new_vec_for_this_ema:Vec<TimePrice> = vec!();
// 											new_vec_for_this_ema.push(new_moving_average);
// 											tracing::debug!("[refresh_moving_averages_for_stock] created new ema hist vec: {:?}", &new_vec_for_this_ema);
// 											ema_hmap_ref.insert(ema_kind.to_owned(), new_vec_for_this_ema);
// 										},
// 										Some(ema_hist_vec_ref) => {
//
// 											// 4a. compute new moving average
// 											// moved to the top of this fn
//
// 											// 4b. create new TimePrice
// 											let new_moving_average = TimePrice{ dt: Utc::now(), price: new_moving_avg_decimal };
//
// 											// 5. Push new ema to vector
// 											ema_hist_vec_ref.push(new_moving_average);
// 											tracing::debug!("[TEST] updated ema hist vec: {:?}", ema_hist_vec_ref);
// 										}
// 									}
//
// 									// TODO: test purpose only
// 									stock_state.print_ema_hist(ema_kind);
//
// 								}
// 							}
// 						}
// 					}
// 				}
// 			}
//
//
//
// 		}
// 	}
//
//
//
//
//
//
//
//
//
// 	//
// 	//
// 	//
// 	//
// 	// 	// loop through the 4, 8, 15, 20 emas
// 	// 	for ema_kind in &ema_array {
// 	//
// 	// 		// TODO: test only
// 	// 		self.print_ema_hist(ema_kind);
// 	//
// 	// 		// calculate the ema and save to history
// 	// 		// TODO: 1. get this moving average
// 	// 		if let Some(this_current_ema) = trade_state_mut_ref.ema_x(ema_kind) {
// 	//
// 	// 			// 2. save ema to history hmap/vector
// 	// 			if let Some(ema_hist) = trade_book.get_ema_history_mut_for_symbol(&symbol, ema_kind) {
// 	// 				let new_ema_entry = TimePrice { dt: Utc::now(), price: this_current_ema };
// 	// 				ema_hist.push(new_ema_entry);
// 	// 				// sort reverse order
// 	// 				ema_hist.sort_by(|a, b| b.cmp(a));
// 	// 				ema_hist.dedup();
// 	//
// 	// 				tracing::debug!("[recalculate] {} historical trade price average: {}", ema_kind, this_current_ema);
// 	// 			}
// 	// 		}
// 	// 		// TODO: test only
// 	// 		.print_ema_hist(ema_kind);
// 	//
// 	//
// 	// 		// }
// 	// 		// else {
// 	// 		// 	tracing::debug!("[recalculate] no mean {} yet", ema_kind);
// 	// 		// }
// 	//
// 	// }
//
// }

// https://stackoverflow.com/questions/36928569/how-can-i-create-enums-with-constant-values-in-rust
#[non_exhaustive]
#[derive(Debug, Clone)]
struct Ema;
impl Ema {
    // pub const EMA04: usize = 4;
    // pub const EMA08: usize = 8;
    // pub const EMA15: usize = 15;
    // pub const EMA20: usize = 20;
}

#[derive(Debug, Clone)]
enum EmaDiff {
    // DIFF4_8,
    // DIFF4_15,
    // DIFF4_20,
    // DIFF8_15,
    // DIFF8_20,
    // DIFF15_20,
}

//
#[derive(Clone, Debug)]
struct StockState {
    _buy_status: bool,
    _price_trade_current: Decimal,
    _dtg_current_trade: DateTime<Utc>,
    _price_bid: Decimal,
    _price_ask: Decimal,

    _trade_price_history: Vec<TimePrice>,

    _ema_hist_hmap: HashMap<usize, Vec<TimePrice>>,

    _ema_diffs: HashMap<EmaDiff, Vec<TimePrice>>,
}

impl StockState {
    // fn new()-> StockState{
    // 	StockState{
    // 		buy_status:false,
    // 		price_trade_current:Decimal::from_u8(0).unwrap(),
    // 		dtg_current_trade:DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
    // 		_price_ask:Decimal::from_u8(0).unwrap(),
    // 		_price_bid:Decimal::from_u8(0).unwrap(),
    //
    // 		trade_price_history: vec!(),
    // 		ema_hist_hmap:HashMap::new(),
    // 		_ema_diffs:HashMap::new(),
    //
    // 	}
    // }

    // When there's a new trade price update, run this to push the old one to history
    // fn push_trade_price_to_history(&mut self){
    // 	self.trade_price_history.push(
    // 		TimePrice {
    // 			dt: self.dtg_current_trade,
    // 			price: self.price_trade_current,
    // 		});
    //
    // 	//TODO: sort descending (by date); could also do v.sort(), v.reverse()
    // 	self.trade_price_history.sort_by(|a, b| b.cmp(a));
    //
    // 	//TODO: dedup by timestamp
    // 	self.trade_price_history.dedup_by(|a, b| a.dt == b.dt );
    //
    // }

    // fn print_ema_hist(&self, ema:&usize){
    //
    // 	if let Some(ema_hist_vec) = self.ema_hist_hmap.get(ema) {
    // 		tracing::debug!("[print_ema_hist] ema history for {}: {:?}", ema, &ema_hist_vec);
    // 	}
    //
    // }

    // Average only the most recent 20 prices
    // fn compute_moving_average_x(&self, x:&usize) -> Option<Decimal> {
    // 	// 1 because it's initialized to zero and don't care about the 1, and it'd be a div by zero anyway
    //
    // 	if self.trade_price_history.len() < *x {
    // 		None
    // 	}
    // 	else {
    //
    // 		let slice_20_opt = self.trade_price_history.as_slice().get(0..*x);
    // 		if let Some(slice) = slice_20_opt {
    //
    // 			tracing::debug!("[compute_moving_average_x] moving avg vector: {:?}", slice);
    //
    // 			if slice.len() >= 1 {
    // 				// avg = sum / len
    // 				Some(slice.iter().fold(Decimal::from_u8(0).unwrap(), |accum, a| accum + a.price ) / Decimal::from(slice.len()))
    // 			} else {
    // 				None
    // 			}
    // 		}
    // 		else {
    // 			None
    // 		}
    // 	}
    // }
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct TimePrice {
    dt: DateTime<Utc>,
    price: Decimal,
    // partialord will sort by variables top to bottom; i'd never want to sort by date then price, but also never want
    // duplicate dates so that shouldn't matter
}

//
// /// Start a thread to listen for messages to the Trader; returns to main thread so main will have to keep itself open to keep this thread from getting closed
// ///
pub fn start() -> Sender<TraderMsg> {
    // Channel for websocket thread to send to database thread
    let (tx, rx) = crossbeam::channel::unbounded();
    let _ = thread::spawn(move || trader_communication_thread(rx));
    tx
}

fn trader_communication_thread(rx: Receiver<TraderMsg>) -> JoinHandle<()> {
    tracing::debug!("[trader_thread]");

    let _trade_book = TraderBook {
        _stock_hmap: HashMap::new(),
    };

    loop {
        // TODO: this macro is so annoying; causes intellij code completion not to work
        crossbeam::channel::select! {
            recv(rx) -> result => {
                if let Ok(msg) = result {
                    match msg {

                        TraderMsg::TradeRest(rest_trade) => {
                            // tracing::debug!("[trader_thread] TraderMsg::LastTradeRest received: {:?}", &last_trade);

                            // TODO: just do tsla for now
                            if rest_trade.symbol.to_lowercase() == "tsla" {




                                // process_trade_rest(&mut trade_book, &rest_trade);







                            }
                        },

                        TraderMsg::TradeWs(_ws_trade) => {
                            // tracing::debug!("[trader_thread] TraderMsg::LastTradeWs received: {:?}", &last_trade);
                            // let _ = recalculate(&symbol_map, ws_trade.symbol);
                        },

                        TraderMsg::QuoteWs(_quote) => {
                            // tracing::debug!("[trader_thread] TraderMsg::QuoteWs received: {:?}", &quote);
                            // let _ = recalculate(&symbol_map, quote.symbol);
                        }
                    }
                }
            }
        }
    }
}

//
// fn process_trade_rest(trade_book:&mut TraderBook, ws_trade:&AlpTradeRest){
//
// 	tracing::debug!("[update_rest_trade] trade_state (before): {}:{:?}", &ws_trade.symbol,trade_book.stock_hmap.get(&ws_trade.symbol));
// 	tracing::debug!("[update_rest_trade] changing to: {:?}", &ws_trade);
//
// 	// push or update new trade price
// 	// TODO: is there a push or partial update?
// 	if !trade_book.stock_hmap.contains_key(&ws_trade.symbol) {
//
// 		// insert new
// 		let mut new_trade_state = StockState::new();
// 		// TODO: change this back to the alpaca date vice when we logged it
// 		new_trade_state.dtg_current_trade = ws_trade.dtg_updated;
// 		new_trade_state.price_trade_current = ws_trade.price;
// 		let k = ws_trade.symbol.to_owned();
// 		let v = new_trade_state;
// 		trade_book.stock_hmap.insert(k, v);
//
// 	} else {
//
// 		// update existing
// 		let v = trade_book.stock_hmap.get_mut(&(ws_trade.symbol)).unwrap();
// 		// save the previous current price to the history queue
// 		v.push_trade_price_to_history();
// 		v.price_trade_current = ws_trade.price;
// 		// TODO: change this back to the alpaca date vice when we logged it;
// 		// dtg_updated is nice because it changes after market hours
// 		v.dtg_current_trade =  ws_trade.dtg_updated;
//
// 	}
//
// 	tracing::debug!("[update_rest_trade] trade_state (after): {}:{:?}", &ws_trade.symbol, trade_book.stock_hmap.get(&ws_trade.symbol));
//
// 	recalculate(trade_book, &ws_trade.symbol);
// }

// // recompute the various moving averages
// fn recalculate(trade_book:&mut TraderBook, symbol:&str) {
//
// 	trade_book.refresh_moving_averages_for_stock(symbol);
//
//
// }
