//! Dedicated Tokio runtime used to drive the SDK's real async work.
//!
//! `#[uniffi::export(async_runtime = "tokio")]` polls the generated future
//! through `async_compat::Compat`, which looks up `Handle::try_current()`
//! and falls back to its own hardcoded *current-thread* runtime whenever
//! there is no ambient Tokio context. UniFFI drives that polling directly
//! from whatever native thread the host (a Kotlin coroutine, a Swift
//! `Task`, ...) happens to call in from, and Tokio's runtime context is
//! thread-local, so merely entering a runtime once during setup would not
//! make later polls on other threads see it.
//!
//! Instead, every exported async method submits its real body to the
//! shared multi-threaded runtime via [`ffi_spawn`], so it always runs on
//! genuine multi-threaded workers regardless of which thread drives the poll
//! loop.
//!
//! The runtime itself is owned by `fedimint-core`
//! ([`fedimint_core::util::ffi`]) so that this crate's exported methods and
//! the module crates' own `subscribe_*` FFI loops all share a single,
//! process-wide worker pool. This module just re-exports those helpers so
//! the local `ffi_export_async!` macro and the `Client` drop glue can keep
//! referring to them by their short names.

use std::future::Future;

// Re-export the shared multi-threaded runtime handle owned by `fedimint-core`
// so the `Client` drop glue can spawn shutdown work onto the same pool.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) use fedimint_core::util::ffi::ffi_runtime;

/// Runs `fut` to completion on the shared multi-threaded [`ffi_runtime`]
/// instead of relying on whatever (possibly current-thread, possibly
/// absent) Tokio context happens to be ambient on the thread driving the
/// FFI poll loop. Delegates to the single runtime owned by `fedimint-core`.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) async fn ffi_spawn<F, T>(fut: F) -> T
where
    F: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    fedimint_core::util::ffi::ffi_spawn(fut).await
}

// On wasm there is no multi-threaded Tokio runtime (and no real OS threads
// to speak of), so there is nothing to guard against: just run the future
// in place.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn ffi_spawn<F: Future>(fut: F) -> F::Output {
    fut.await
}

/// Declares a `#[uniffi::export(async_runtime = "tokio")]` impl block whose
/// async methods each run their body on the dedicated [`ffi_spawn`] runtime.
///
/// Write the impl and its methods exactly as you normally would, with clean
/// bodies. For every `async fn`, the macro rewrites the body to
/// `ffi_spawn(async move { <body> }).await` and emits it as a literal `fn`
/// inside the exported impl.
///
/// The whole impl has to go through a single macro (rather than annotating
/// each method) because `#[uniffi::export]` only accepts literal `fn` items:
/// a per-method macro invocation isn't a `fn` item as far as UniFFI's
/// proc-macro can see, so it would be rejected. Expanding the entire impl in
/// one shot means UniFFI still sees ordinary functions.
///
/// Non-async methods are passed through unchanged.
macro_rules! ffi_export_async {
    (
        impl $ty:ty {
            $(
                $(#[$meth_attr:meta])*
                $meth_vis:vis async fn $name:ident ( $($args:tt)* ) $(-> $ret:ty)? $body:block
            )*
        }
    ) => {
        #[uniffi::export(async_runtime = "tokio")]
        impl $ty {
            $(
                $(#[$meth_attr])*
                $meth_vis async fn $name ( $($args)* ) $(-> $ret)? {
                    $crate::runtime::ffi_spawn(async move $body).await
                }
            )*
        }
    };
}
pub(crate) use ffi_export_async;
