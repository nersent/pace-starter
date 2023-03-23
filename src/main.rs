use std::path::Path;

use chrono::Datelike;
use custom_indicator::{CustomIndicator, CustomIndicatorConfig};
use custom_strategy::{CustomStrategy, CustomStrategyConfig};
use nersent_pace::{
    core::{
        context::Context, data_provider::DataProvider,
        in_memory_data_provider::InMemoryDataProvider, incremental::Incremental,
    },
    polars::io::read_df,
    strategy::{
        metrics::{
            cobra_metrics::{CobraMetrics, CobraMetricsConfig},
            tradingview_metrics::{TradingViewMetrics, TradingViewMetricsConfig},
        },
        strategy::{Strategy, StrategyConfig},
    },
};

mod custom_indicator;
mod custom_strategy;
mod tests;

fn main() {
    // Load dataset into Polars dataframe
    let data_path = Path::new("data/btc_1d.csv");
    let df = read_df(&data_path);

    // Create context for all components
    let ctx = Context::new(InMemoryDataProvider::from_df(&df).to_arc());

    // Declare strategy component to handle all trades
    let mut strategy = Strategy::new(
        ctx.clone(),
        StrategyConfig {
            initial_capital: 1000.0,
            ..StrategyConfig::default()
        },
    );

    // Declare metric providers.
    // We need to pass an immutable reference to strategy, for initialization of default metric values
    let mut metrics =
        TradingViewMetrics::new(ctx.clone(), &strategy, TradingViewMetricsConfig::default());
    let mut cobra_metrics =
        CobraMetrics::new(ctx.clone(), &strategy, CobraMetricsConfig::default());

    // Target indicator and strategy
    let mut macd_indicator = CustomIndicator::new(ctx.clone(), CustomIndicatorConfig::default());
    let mut macd_strategy = CustomStrategy::new(ctx.clone(), CustomStrategyConfig::default());

    // If you want to test your strategy since the beggining, set this to None
    let strategy_start_year: Option<i32> = Some(2017); // None

    // Iterate over all bars
    for _ in ctx.clone() {
        let macd_delta = macd_indicator.next(());

        if strategy_start_year.is_some()
            && ctx.bar.datetime().unwrap().year() < strategy_start_year.unwrap()
        {
            continue;
        }

        let trade = macd_strategy.next(macd_delta);

        strategy.next(trade);
        // We need to pass an immutable reference to strategy, so metrics can access internal data
        metrics.next(&strategy);
        cobra_metrics.next(&strategy);
    }

    let currency = "USD";
    metrics.data.print_overview(currency);
    metrics.data.plot_net_equity((236, 100)); // Change plot size, if your terminal doesn't fit it
    metrics.data.print_summary(currency);
    cobra_metrics.data.print();
}
