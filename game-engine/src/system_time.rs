pub mod time {
    pub use std::time::SystemTime;



    pub struct TimerScoped {
        t_ms: u128,
        t_us: u128,
        t_ns: u128,
    }

    // Construction
    impl TimerScoped {
        pub fn new() -> Self {
            let t_ms = now_ms();
            let t_us = now_us();
            let t_ns = now_ns();
            return Self { t_ms, t_us, t_ns };
        }
    }
    // Destruction
    impl Drop for TimerScoped {
        fn drop(&mut self) {
            let t_ms = now_ms() - self.t_ms;
            let t_s = t_ms as f32 / 1000.0;
            let t_us = now_us() - self.t_us;
            let t_ns = now_ns() - self.t_ns;
            println!("Operation finished in: ");
            println!("s:  {}", t_s);
            println!("ms: {}", t_ms);
            println!("us: {}", t_us);
            println!("ns: {}", t_ns);
        }
    }

    // Timer functionality
    pub fn now_ms() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_millis();
    }
    pub fn now_ms_workaround() -> u128 {
        return now_us() / 1000.0 as u128;
    }
    pub fn now_us() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_micros();
    }
    pub fn now_ns() -> u128 {
        return std::time::SystemTime::now().duration_since(std::time::SystemTime::UNIX_EPOCH).unwrap().as_nanos();
    }
}