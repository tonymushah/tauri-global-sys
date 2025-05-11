#![allow(dead_code)]

use std::sync::{Arc, Mutex};

use futures::{future::AbortHandle as FutureAbortHandle, stream::AbortHandle as StreamAbortHandle};
use leptos::prelude::on_cleanup;

pub struct UseAbortReturns<
    H,
    A: Fn() + Clone + Send + Sync + 'static,
    P: Fn(H) + Clone + Send + Sync + 'static,
> {
    pub abort: A,
    pub push_handle: P,
    pub aborts: Arc<Mutex<Vec<H>>>,
}

pub fn use_stream_aborts(
    auto_clean_up: bool,
) -> UseAbortReturns<
    StreamAbortHandle,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn(StreamAbortHandle) + Clone + Send + Sync + 'static,
> {
    let aborts = Arc::new(Mutex::new(Vec::<StreamAbortHandle>::new()));
    let abort = {
        let aborts = aborts.clone();
        move || {
            if let Ok(lock) = aborts.lock() {
                for handle in lock.iter() {
                    handle.abort();
                }
            }
        }
    };
    if auto_clean_up {
        let abort = abort.clone();
        on_cleanup(move || {
            log::trace!("Event cleanup");
            abort()
        });
    }
    let push_handle = {
        let aborts = aborts.clone();
        move |handle: StreamAbortHandle| {
            if let Ok(mut lock) = aborts.lock() {
                lock.push(handle);
            }
        }
    };
    UseAbortReturns {
        abort,
        aborts,
        push_handle,
    }
}

pub fn use_future_aborts(
    auto_clean_up: bool,
) -> UseAbortReturns<
    FutureAbortHandle,
    impl Fn() + Clone + Send + Sync + 'static,
    impl Fn(FutureAbortHandle) + Clone + Send + Sync + 'static,
> {
    let aborts = Arc::new(Mutex::new(Vec::<FutureAbortHandle>::new()));
    let abort = {
        let aborts = aborts.clone();
        move || {
            if let Ok(lock) = aborts.lock() {
                for handle in lock.iter() {
                    handle.abort();
                }
            }
        }
    };
    if auto_clean_up {
        on_cleanup(abort.clone());
    }
    let push_handle = {
        let aborts = aborts.clone();
        move |handle: FutureAbortHandle| {
            if let Ok(mut lock) = aborts.lock() {
                lock.push(handle);
            }
        }
    };
    UseAbortReturns {
        abort,
        aborts,
        push_handle,
    }
}
