use super::{Disposable, Signal0, Signal1};

/// Represents a value that isn't ready yet, but may become available in the future.
pub struct Promise<A> {
    /// Whether the result is available yet.
    pub has_result: bool,

    /// Emitted when the promise is fulfilled and the result has become available.
    pub success: Signal1<A>,

    /// An error message emitted if there was a problem during loading.
    pub error: Signal1<String>,

    /// May be emitted during loading when the progress or total counts have been updated.
    pub progress_changed: Signal0,

    /// The end result fulfilled by the promise. When accessing, throws an error if the result is not
    /// yet available. Read hasResult to check availability first, or use get(). When setting, throws
    /// an error if the result was already previously assigned.
    pub result: Option<A>,
    /// Context on how close this promise is to being fulfilled. For file IO, these are in bytes.
    pub progress: f32,
    pub total: f32,
}

impl<A> Promise<A> {
    pub fn new() -> Self {
        Self {
            success: Signal1::new(None),
            error: Signal1::new(None),
            progress_changed: Signal0::new(None),
            has_result: false,
            progress: 0.0,
            total: 0.0,
            result: None,
        }
    }

    pub fn result(&self) -> &Option<A> {
        if !self.has_result {
            panic!("Promise result not yet available");
        }

        &self.result
    }

    pub fn set_result(&self, result: Option<A>) -> Option<A> {
        todo!("should deal with it");
        // if self.hasResult {
        //     panic!("Promise result already assigned");
        // }

        // self.result = result;
        // self.hasResult = true;
        // self.success.emit(result);

        // result
    }

    /// Passes the result to the callback now if the result is available, otherwise calls it later.
    /// @returns If the callback was not called yet, a handle that can be disposed to cancel the
    /// request.
    pub fn get(&self, function: Box<dyn Fn(&A) + 'static>) -> Option<impl Disposable> {
        if self.has_result {
            if let Some(ref result) = self.result {
                function(result);
            }

            return None;
        }

        let conn = self.success.connect(function, false).once().clone();
        Some(conn)
    }

    #[inline]
    pub fn progress(&self) -> f32 {
        self.progress
    }

    pub fn set_progress(&mut self, progress: f32) {
        if self.progress != progress {
            self.progress = progress;
            self.progress_changed.emit();
        }
    }

    pub fn set_total(&mut self, total: f32) {
        if self.total != total {
            self.total = total;
            self.progress_changed.emit();
        }
    }

    #[inline]
    pub fn total(&self) -> f32 {
        self.total
    }
}
