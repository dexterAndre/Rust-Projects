pub mod time {
    pub use std::time::SystemTime;

    pub fn now_ms() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis();
    }
    pub fn now_us() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_micros();
    }
    pub fn now_ns() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    }

    pub struct TimerScoped {
        t0: u128,
    }

    // Construction
    impl TimerScoped {
        pub fn new() -> Self {
            let t0 = now_us();
            return Self { t0 };
        }
    }
    // Destruction
    impl Drop for TimerScoped {
        fn drop(&mut self) {
            let t = now_us() - self.t0;
            println!("Operation took: {}us time to finish. ", t);
        }
    }
}