use std::sync::atomic::{AtomicBool, Ordering};

static ACTIVE: AtomicBool = AtomicBool::new(false);

/// Android Activities may be recreated inside one process, but their event
/// loops must never overlap. Keep the permit until all event-loop fields drop.
#[derive(Debug)]
pub(crate) struct EventLoopLifetime;

impl EventLoopLifetime {
    pub(crate) fn acquire() -> Option<Self> {
        ACTIVE.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).ok().map(|_| Self)
    }
}

impl Drop for EventLoopLifetime {
    fn drop(&mut self) {
        ACTIVE.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn activity_loops_are_exclusive_and_release_after_drop_or_unwind() {
        let first = EventLoopLifetime::acquire().unwrap();
        assert!(EventLoopLifetime::acquire().is_none());
        drop(first);
        assert!(
            std::panic::catch_unwind(|| {
                let _next = EventLoopLifetime::acquire().unwrap();
                assert!(EventLoopLifetime::acquire().is_none());
                panic!("construction failed");
            })
            .is_err()
        );
        let next = EventLoopLifetime::acquire().unwrap();
        assert!(EventLoopLifetime::acquire().is_none());
        drop(next);
        assert!(EventLoopLifetime::acquire().is_some());
    }
}
