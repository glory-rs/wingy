//! A paused Activity may have explicit application work without a render surface.
//! Keep proxy wakes independent from redraws; pending redraws wait for Resume.
pub(crate) fn has_work(running: bool, pending_redraw: bool, proxy_wake: bool) -> bool {
    proxy_wake || (running && pending_redraw)
}

#[cfg(test)]
mod tests {
    use super::has_work;

    #[test]
    fn external_activity_does_not_park_explicit_proxy_work() {
        assert!(has_work(false, false, true));
        assert!(has_work(false, true, true));
    }

    #[test]
    fn suspended_redraws_do_not_force_busy_polling() {
        assert!(!has_work(false, true, false));
        assert!(!has_work(false, false, false));
    }

    #[test]
    fn resumed_redraw_and_proxy_requests_both_wake_the_loop() {
        assert!(has_work(true, true, false));
        assert!(has_work(true, false, true));
        assert!(!has_work(true, false, false));
    }

    #[test]
    fn coalesced_proxy_is_consumed_but_a_later_one_still_wakes() {
        let mut proxy = true;
        assert!(has_work(false, true, proxy));
        assert!(std::mem::replace(&mut proxy, false));
        assert!(!has_work(false, true, proxy));
        proxy = true;
        assert!(has_work(false, true, proxy));
    }
}
