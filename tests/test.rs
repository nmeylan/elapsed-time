#[cfg(test)]
mod tests {
    use std::thread::sleep;
    use std::time::Duration;
    use elapsed_time::elapsed;
    #[cfg(feature = "tracing")]
    use tracing_subscriber::util::SubscriberInitExt;

    #[elapsed]
    #[test]
    fn on_method() {
        #[cfg(feature = "tracing")]
        tracing_subscriber::fmt::Subscriber::builder()
            .with_max_level(tracing::level_filters::LevelFilter::DEBUG)
            .with_test_writer()
            .finish()
            .init();
        sleep(Duration::from_secs(1))
    }

}