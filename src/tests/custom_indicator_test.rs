#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use nersent_pace::{
        core::incremental::Incremental,
        testing::{
            array_snapshot::ArraySnapshot,
            fixture::{DataFrameFixtureUtils, Fixture},
        },
    };

    use crate::custom_indicator::{CustomIndicator, CustomIndicatorConfig};

    fn format_path(path: &str) -> PathBuf {
        Path::new(&format!("fixtures/macd/{}", path)).to_path_buf()
    }

    fn _test(target: &mut CustomIndicator, expected: &[Option<f64>]) {
        let mut snapshot = ArraySnapshot::<Option<f64>>::new();
        for _ in target.ctx.clone() {
            let output = target.next(());
            snapshot.push(output);
        }
        snapshot.assert(expected);
    }

    #[test]
    fn fast_12_slow_26_sig_9() {
        let (df, ctx) = Fixture::load_ctx(&format_path("fast_12_slow_26_sig_9.csv"));
        _test(
            &mut CustomIndicator::new(ctx.clone(), CustomIndicatorConfig::default()),
            &df.test_target(),
        );
    }
}
