use std::time::{Duration, Instant};

pub struct Reporter {
    pub last_reporting: Instant,
    pub report_after: Duration,
    pub last_result: Instant,
    pub result_after: Duration,
}

impl Reporter {
    pub fn new(sec_status: u64, sec_result: u64) -> Self {
        Self {
            last_reporting: std::time::Instant::now(),
            report_after: Duration::new(sec_status, 0),
            last_result: std::time::Instant::now(),
            result_after: Duration::new(sec_result, 0),
        }
    }

    pub fn update_last_reporting(&mut self) {
        self.last_reporting = std::time::Instant::now()
    }

    pub fn update_last_result(&mut self) {
        self.last_result = std::time::Instant::now()
    }

    // pub fn check_time(&self) {
    //     if self.last_reporting.elapsed() > self.report_after {
    //         let mio = (result.num_total as f64 / 100_000.0).round() / 10.0;
    //         let p = (result.num_total as f64 / total_to_check as f64 * 1000.0).round() / 10.0;
    //         println!("Working: {} = {} million, {p}%", result.num_total, mio);
    //         self.last_reporting = std::time::Instant::now();
    //         if self.last_result.elapsed() > self.result_after {
    //             println!("Current result {}", self.result);
    //             self.last_result = std::time::Instant::now();
    //         }
    //     }
    // }
}
