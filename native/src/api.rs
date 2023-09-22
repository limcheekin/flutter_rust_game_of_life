mod app;
mod err;
use std::future::Future;
use std::pin::Pin;

use flutter_rust_bridge::{IntoDart, RustOpaque, StreamSink};
use surrealdb::engine::any::Any;
use surrealdb::Surreal;

// REF: https://github.com/fzyzcjy/flutter_rust_bridge/issues/966#issuecomment-1560155050
pub struct SurrealDB {
    pub db: RustOpaque<Surreal<Any>>,
}

pub(crate) struct AsyncAdapter;

impl AsyncAdapter {
    pub(crate) fn spawn_stream<T: IntoDart + 'static>(
        sink: StreamSink<T>,
        f: impl for<'a> FnOnce(&'a StreamSink<T>) -> Pin<Box<dyn Future<Output = ()> + 'a>> + 'static,
    ) -> anyhow::Result<()> {
        futures::executor::block_on(async move {
            f(&sink).await;
            sink.close();
        });

        Ok(())
    }

    pub(crate) fn spawn<F: Future + 'static>(f: F) {
        futures::executor::block_on(f);
    }
}

impl SurrealDB {
    pub fn connect(&self, sink: StreamSink<()>, endpoint: String) {
        let db = self.db.clone();
        AsyncAdapter::spawn(async move {
            match db.connect(endpoint).await {
                Err(e) => {
                    log::error!("{e}");
                }
                Ok(_) => (),
            }
            sink.close();
        });
    }
}
