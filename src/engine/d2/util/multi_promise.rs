use crate::engine::d2::{platform::Dynamic, Disposer};

use super::Promise;
/// Multiplexes a list of promises into a single promise. When all promises have been fulfilled, the
/// success signal will be emitted with an array containing their respective results. Progress and
/// error signals are also aggregated.
pub struct MultiPromise {
    pub inner: Promise<Vec<Dynamic>>,
    promises: Vec<Promise<Dynamic>>,
    success_count: usize,
    disposer: Disposer,
}

impl MultiPromise {
    pub fn new(promises: Vec<Promise<Dynamic>>) -> Self {
        let mut instance = Self {
            inner: Promise::new(),
            promises,
            success_count: 0,
            disposer: Disposer::new(),
        };

        instance.on_progress_changed();

        // let disposer = &mut instance.disposer;

        // for promise in instance.promises.iter() {
        //     disposer
        //         .connect1(promise.error.clone(), Box::new(|msg| instance.on_error(*msg)));

        //     disposer
        //         .connect0(promise.progress_changed.clone(), Box::new(|| instance.on_progress_changed()));

        //     // let pending = promise.get(Box::new(|v| instance.on_success(v)));

        //     // if let Some(paending) = pending {
        //     //     // instance.disposer.add(Box::new(pending));
        //     //     // todo!("should deal with it");
        //     // }
        //     // // todo!("should deal with it");
        // }

        instance
    }

    pub fn on_success(&mut self, v: &Dynamic) {
        if self.promises.is_empty() {
            return;
        }

        self.success_count += 1;
        if self.success_count >= self.promises.len() {
            let mut results = Vec::new();
            for promise in self.promises.iter() {
                results.push(promise.result());
            }
            self.finalize();
            todo!("should deal with it");
            // self.inner.set_result(Some(results));
        }
    }

    pub fn on_error(&mut self, message: String) {
        if self.promises.is_empty() {
            return;
        }
        self.finalize();
        self.inner.error.emit(message);
    }

    pub fn on_progress_changed(&mut self) {
        if self.promises.is_empty() {
            return;
        }

        self.inner.progress = 0.0;
        self.inner.total = 0.0;

        for promise in self.promises.iter() {
            self.inner.progress += promise.progress();
            self.inner.total += promise.total();
        }

        self.inner.progress_changed.emit();
    }

    fn finalize(&mut self) {
        self.disposer.dispose();
        self.promises.clear(); // Marks that all promises are fulfilled, or there was an error
    }
}

impl AsRef<Promise<Vec<Dynamic>>> for MultiPromise {
    fn as_ref(&self) -> &Promise<Vec<Dynamic>> {
        &self.inner
    }
}
