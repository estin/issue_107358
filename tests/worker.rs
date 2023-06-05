use issue_107358::{Message, Worker};
use std::sync::Once;
use tokio::sync::mpsc;

static LOG_INIT: Once = Once::new();

#[test]
fn it_works_sync() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_stack_size(100 * 1024 * 1024)
        .build()
        .unwrap();

    let local_set = tokio::task::LocalSet::new();
    rt.block_on(local_set.run_until(it_works()));
}

// #[ntex::test]
async fn it_works() {
    LOG_INIT.call_once(|| {
        std::env::set_var("RUST_LOG", "trace");
        let _ = env_logger::builder().is_test(true).try_init();
    });

    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let mut tasks: Vec<ntex::rt::JoinHandle<()>> = Vec::new();

    let job = Box::pin(async move {
        loop {
            let Some(message) = rx.recv().await else {
                continue
            };

            let span = tracing::info_span!("im-worker", consume_lag_ms = tracing::field::Empty);
            let fill_stack = [0; 100 * 1024];

            tokio::spawn(async move {
                let worker = Worker {};
                log::info!("{}", fill_stack.len());

                sentry::with_scope(
                    |scope| {
                        scope.set_extra("lag_ms", 1000.into());
                    },
                    || {
                        sentry::capture_message(
                            "Kafka consume lag",
                            sentry::protocol::Level::Warning,
                        )
                    },
                );

                let guard = span.enter();

                worker.process_incoming(message).await;

                drop(guard);
            });
        }
    });

    tasks.push(ntex::rt::spawn(job));

    for i in 1..10000 {
        tx.send(Message::new(i.to_string())).unwrap();
    }

    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
}
