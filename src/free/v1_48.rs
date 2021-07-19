pub(crate) mod future {
    use core::future::Future;
    use core::pin::Pin;
    use core::task::{Context, Poll};

    pub struct Ready<T>(Option<T>);

    impl<T> Unpin for Ready<T> {}

    impl<T> Future for Ready<T> {
        type Output = T;

        fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
            Poll::Ready(self.0.take().expect("Ready polled after completion"))
        }
    }

    pub fn ready<T>(t: T) -> Ready<T> {
        Ready(Some(t))
    }

    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Pending<T> {
        _data: core::marker::PhantomData<T>,
    }

    pub fn pending<T>() -> Pending<T> {
        Pending { _data: core::marker::PhantomData }
    }

    impl<T> Future for Pending<T> {
        type Output = T;

        fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T> {
            Poll::Pending
        }
    }

    impl<T> Unpin for Pending<T> {}

    impl<T> Clone for Pending<T> {
        fn clone(&self) -> Self {
            pending()
        }
    }
}
