use serde::Serialize;
use sha2::{Digest, Sha256};
use std::io::{self, Write};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub struct PriceTicks(u64);

#[derive(Clone, Copy, Debug, Serialize)]
#[serde(transparent)]
pub struct QuantityLots(u64);

#[derive(Debug, Serialize)]
pub struct Level {
    pub price_ticks: PriceTicks,
    pub quantity_lots: QuantityLots,
}

#[derive(Debug, Serialize)]
pub struct MarketEvent {
    pub schema_version: &'static str,
    pub event_id: String,
    pub sequence: u32,
    pub event_time_ns: u64,
    pub receive_time_ns: u64,
    pub symbol: &'static str,
    pub bids: [Level; 2],
    pub asks: [Level; 2],
    pub trade: Level,
}

#[derive(Debug, Serialize)]
pub struct ObservationReceipt {
    pub schema_version: &'static str,
    pub kind: &'static str,
    pub event_id: String,
    pub sequence: u32,
    pub observation_time_ns: u64,
    pub event_sha256: String,
    pub trade_quote_atoms: u64,
    pub cumulative_trade_quote_atoms: u64,
}

#[derive(Clone, Debug, Serialize)]
pub struct Scenario {
    pub schema_version: &'static str,
    pub generator: &'static str,
    pub seed: u64,
    pub events: u32,
    pub start_time_ns: u64,
    pub interval_ns: u64,
    pub receive_latency_ns: u64,
    pub price_tick_denominator: u64,
    pub quantity_lot_denominator: u64,
    pub quote_atom_denominator: u64,
}

impl Scenario {
    pub fn new(seed: u64, events: u32) -> Result<Self> {
        if !(1..=1_000_000).contains(&events) {
            return Err(invalid("events must be in 1..=1000000").into());
        }
        Ok(Self {
            schema_version: "synthetic-scenario/v1",
            generator: "splitmix64-l2-trade/v1",
            seed,
            events,
            start_time_ns: 1_700_000_000_000_000_000,
            interval_ns: 1_000_000,
            receive_latency_ns: 100_000,
            price_tick_denominator: 100,
            quantity_lot_denominator: 1_000,
            quote_atom_denominator: 100_000,
        })
    }
}

pub fn invalid(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidInput, message)
}

pub fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub fn json_line(value: &impl Serialize) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

struct SplitMix64(u64);

impl SplitMix64 {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e3779b97f4a7c15);
        let mut value = self.0;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d049bb133111eb);
        value ^ (value >> 31)
    }
}

fn level(price_ticks: u64, quantity_lots: u64) -> Level {
    Level {
        price_ticks: PriceTicks(price_ticks),
        quantity_lots: QuantityLots(quantity_lots),
    }
}

fn event(seed: u64, sequence: u32, random: &mut SplitMix64, scenario: &Scenario) -> MarketEvent {
    let mid = 10_000 + random.next() % 201;
    let spread = 1 + random.next() % 5;
    let bid_size = 1 + random.next() % 100;
    let ask_size = 1 + random.next() % 100;
    let trade_size = 1 + random.next() % 25;
    let trade_price = if random.next().is_multiple_of(2) {
        mid + spread
    } else {
        mid - spread
    };
    let event_time_ns = scenario.start_time_ns + u64::from(sequence - 1) * scenario.interval_ns;
    MarketEvent {
        schema_version: "synthetic-market-event/v1",
        event_id: format!("synthetic:{seed}:{sequence}"),
        sequence,
        event_time_ns,
        receive_time_ns: event_time_ns + scenario.receive_latency_ns,
        symbol: "SYNBASE/SYNQUOTE",
        bids: [
            level(mid - spread, bid_size),
            level(mid - spread - 1, bid_size + 1),
        ],
        asks: [
            level(mid + spread, ask_size),
            level(mid + spread + 1, ask_size + 1),
        ],
        trade: level(trade_price, trade_size),
    }
}

fn observe(event: &MarketEvent, bytes: &[u8], cumulative: u64) -> Result<ObservationReceipt> {
    let bids = &event.bids;
    let asks = &event.asks;
    if !(bids[0].price_ticks.0 > bids[1].price_ticks.0
        && bids[1].price_ticks.0 > 0
        && asks[0].price_ticks.0 < asks[1].price_ticks.0
        && bids[0].price_ticks.0 < asks[0].price_ticks.0
        && event.trade.price_ticks.0 >= bids[0].price_ticks.0
        && event.trade.price_ticks.0 <= asks[0].price_ticks.0
        && event.receive_time_ns >= event.event_time_ns
        && bids
            .iter()
            .chain(asks)
            .chain([&event.trade])
            .all(|l| l.quantity_lots.0 > 0))
    {
        return Err(
            invalid("synthetic event has an invalid book, trade or visibility time").into(),
        );
    }
    let quote = event
        .trade
        .price_ticks
        .0
        .checked_mul(event.trade.quantity_lots.0)
        .ok_or_else(|| invalid("trade quote atoms overflow"))?;
    let total = cumulative
        .checked_add(quote)
        .ok_or_else(|| invalid("cumulative trade quote atoms overflow"))?;
    Ok(ObservationReceipt {
        schema_version: "market-observation-receipt/v1",
        kind: "market-observation",
        event_id: event.event_id.clone(),
        sequence: event.sequence,
        observation_time_ns: event.receive_time_ns,
        event_sha256: sha256(bytes),
        trade_quote_atoms: quote,
        cumulative_trade_quote_atoms: total,
    })
}

#[derive(Debug, Serialize)]
pub struct ReplaySummary {
    pub events: u32,
    pub event_sha256: String,
    pub receipt_sha256: String,
    pub cumulative_trade_quote_atoms: u64,
}

pub fn replay(
    seed: u64,
    events: u32,
    event_output: &mut impl Write,
    receipt_output: &mut impl Write,
) -> Result<ReplaySummary> {
    let scenario = Scenario::new(seed, events)?;
    let mut random = SplitMix64(seed);
    let mut event_hash = Sha256::new();
    let mut receipt_hash = Sha256::new();
    let mut cumulative = 0;
    for sequence in 1..=events {
        let event = event(seed, sequence, &mut random, &scenario);
        let event_bytes = json_line(&event)?;
        let receipt = observe(&event, &event_bytes, cumulative)?;
        let receipt_bytes = json_line(&receipt)?;
        event_output.write_all(&event_bytes)?;
        receipt_output.write_all(&receipt_bytes)?;
        event_hash.update(&event_bytes);
        receipt_hash.update(&receipt_bytes);
        cumulative = receipt.cumulative_trade_quote_atoms;
    }
    Ok(ReplaySummary {
        events,
        event_sha256: format!("{:x}", event_hash.finalize()),
        receipt_sha256: format!("{:x}", receipt_hash.finalize()),
        cumulative_trade_quote_atoms: cumulative,
    })
}
