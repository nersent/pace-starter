use nersent_pace::{
    core::{context::Context, incremental::Incremental},
    pinescript::common::ps_diff,
    ta::exponential_moving_average::Ema,
};

pub struct CustomIndicatorConfig {
    pub fast_length: usize,
    pub slow_length: usize,
    pub macd_length: usize,
}

impl Default for CustomIndicatorConfig {
    fn default() -> Self {
        return Self {
            fast_length: 12,
            slow_length: 26,
            macd_length: 9,
        };
    }
}

/// MACD example indicator.
pub struct CustomIndicator {
    pub config: CustomIndicatorConfig,
    pub ctx: Context,
    fast_ema: Ema,
    slow_ema: Ema,
    a_macd_ema: Ema,
}

impl CustomIndicator {
    pub fn new(ctx: Context, config: CustomIndicatorConfig) -> Self {
        return Self {
            ctx: ctx.clone(),
            fast_ema: Ema::new(ctx.clone(), config.fast_length),
            slow_ema: Ema::new(ctx.clone(), config.slow_length),
            a_macd_ema: Ema::new(ctx.clone(), config.macd_length),
            config,
        };
    }
}

/**
 * ```pine
 * fastLength = input(12)
 * slowlength = input(26)
 * MACDLength = input(9)
 * MACD = ta.ema(close, fastLength) - ta.ema(close, slowlength)
 * aMACD = ta.ema(MACD, MACDLength)
 * delta = MACD - aMACD
 * ```
 */
impl Incremental<(), Option<f64>> for CustomIndicator {
    fn next(&mut self, _: ()) -> Option<f64> {
        let close = self.ctx.bar.close();

        let macd = ps_diff(self.fast_ema.next(close), self.slow_ema.next(close));
        let amacd = self.a_macd_ema.next(macd);
        let delta = ps_diff(macd, amacd);

        return delta;
    }
}
