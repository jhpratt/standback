#[cfg(feature = "alloc")]
pub(crate) mod task {
    use alloc::sync::Arc;

    pub trait Wake {
        fn wake(self: Arc<Self>);
        #[cfg(since = "1.41")]
        fn wake_by_ref(self: &Arc<Self>) {
            self.clone().wake();
        }
    }
}
