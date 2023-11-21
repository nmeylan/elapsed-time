#![feature(stmt_expr_attributes)]
#![feature(proc_macro_hygiene)]

#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use elapsed_time::elapsed_block;
    #[cfg(feature = "tracing")]
    use tracing_subscriber::util::SubscriberInitExt;

    #[test]
    fn on_block() {
        #[cfg(feature = "tracing")]
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(tracing::level_filters::LevelFilter::DEBUG)
            .with_test_writer()
            .finish()
            .init();
        #[elapsed_block(block_name_1)]
        {
            sleep(Duration::from_secs(1))
        }
    }
}