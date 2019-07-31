//! Allows substitution of the `Instant` type with `Duration` when `Instant::now()` is not supported
//! for a given target.

cfg_if::cfg_if! {
  if #[cfg(all(target_arch = "wasm32", feature = "time_web_sys"))] {
     mod web_sys;
     pub use self::web_sys::*;
  } else {
     mod std;
     pub use self::std::*;
  }
}
