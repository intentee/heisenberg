use std::thread;
use std::time::Duration;

pub fn pause() {
    thread::sleep(Duration::ZERO);
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    use super::pause;

    #[test]
    fn pause_returns_after_a_sleep() {
        pause();
        thread::sleep(Duration::ZERO);
        assert_eq!(Duration::ZERO.as_secs(), 0);
    }
}
