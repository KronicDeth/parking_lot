use core::ops::Add;
use core::time::Duration;

/// A measurement of a monotonically nondecreasing clock.
#[derive(Copy, Clone, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Instant(Milliseconds);

impl Instant {
    /// Returns an instant corresponding to "now".
    pub fn now() -> Instant {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");

        Instant(performance.now() as Milliseconds)
    }

    /// Returns `Some(t)` where `t` is the time `self + duration` if `t` can be represented as
    /// `Instant` (which means it's inside the bounds of the underlying data structure), `None`
    /// otherwise.
    pub fn checked_add(&self, duration: Duration) -> Option<Instant> {
        Duration::from_millis(self.0 as u64)
            .checked_add(duration)
            .map(|sum| Instant(sum.as_millis()))
    }
}

impl Add<Duration> for Instant {
    type Output = Instant;

    fn add(self, duration: Duration) -> Self {
        Instant(self.0 + duration.as_millis())
    }
}

type Milliseconds = u128;
