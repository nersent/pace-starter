use nersent_pace::{
    core::{context::Context, incremental::Incremental},
    strategy::trade::TradeDirection,
    ta::{cross_over_threshold::CrossOverThreshold, cross_under_threshold::CrossUnderThreshold},
};

#[derive(Clone, Copy, Debug)]
pub struct CustomStrategyConfig {
    threshold_overbought: f64,
    threshold_oversold: f64,
}

impl Default for CustomStrategyConfig {
    fn default() -> Self {
        return Self {
            threshold_overbought: 0.0,
            threshold_oversold: 0.0,
        };
    }
}

/// MACD example strategy.
pub struct CustomStrategy {
    pub ctx: Context,
    pub config: CustomStrategyConfig,
    cross_over: CrossOverThreshold,
    cross_under: CrossUnderThreshold,
}

impl CustomStrategy {
    pub fn new(ctx: Context, config: CustomStrategyConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            config,
            cross_over: CrossOverThreshold::new(ctx.clone(), config.threshold_overbought),
            cross_under: CrossUnderThreshold::new(ctx.clone(), config.threshold_oversold),
        };
    }
}

impl Incremental<Option<f64>, Option<TradeDirection>> for CustomStrategy {
    fn next(&mut self, macd_delta: Option<f64>) -> Option<TradeDirection> {
        let cross_over = self.cross_over.next(macd_delta);
        let cross_under = self.cross_under.next(macd_delta);

        if cross_over {
            return Some(TradeDirection::Long);
        } else if cross_under {
            return Some(TradeDirection::Short);
        }

        return None;
    }
}
