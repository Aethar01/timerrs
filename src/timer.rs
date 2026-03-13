use chrono::{DateTime, Local};
use std::time::{Duration, Instant};

pub struct Timer {
    pub name: Option<String>,
    pub start_time: DateTime<Local>,
    pub duration: Duration,
    elapsed_before_pause: Duration,
    resume_time: Option<Instant>,
    pub is_paused: bool,
}

impl Timer {
    pub fn new(name: Option<String>, duration: Duration) -> Self {
        Self {
            name,
            start_time: Local::now(),
            duration,
            elapsed_before_pause: Duration::ZERO,
            resume_time: Some(Instant::now()),
            is_paused: false,
        }
    }

    pub fn toggle_pause(&mut self) {
        if self.is_paused {
            self.resume();
        } else {
            self.pause();
        }
    }

    pub fn pause(&mut self) {
        if !self.is_paused {
            if let Some(resume_time) = self.resume_time {
                self.elapsed_before_pause += resume_time.elapsed();
            }
            self.resume_time = None;
            self.is_paused = true;
        }
    }

    pub fn resume(&mut self) {
        if self.is_paused {
            self.resume_time = Some(Instant::now());
            self.is_paused = false;
        }
    }

    pub fn remaining_time(&self) -> Duration {
        let elapsed = if self.is_paused {
            self.elapsed_before_pause
        } else {
            self.elapsed_before_pause
                + self
                    .resume_time
                    .map(|t| t.elapsed())
                    .unwrap_or(Duration::ZERO)
        };

        self.duration.checked_sub(elapsed).unwrap_or(Duration::ZERO)
    }

    pub fn is_finished(&self) -> bool {
        self.remaining_time().is_zero()
    }

    pub fn progress(&self) -> f64 {
        let remaining = self.remaining_time();
        let elapsed = self.duration.as_secs_f64() - remaining.as_secs_f64();
        (elapsed / self.duration.as_secs_f64()).clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_timer_initialization() {
        let duration = Duration::from_secs(10);
        let timer = Timer::new(Some("Test".to_string()), duration);
        assert_eq!(timer.name, Some("Test".to_string()));
        assert_eq!(timer.duration, duration);
        assert!(!timer.is_paused);
    }

    #[test]
    fn test_timer_no_name() {
        let duration = Duration::from_secs(60);
        let timer = Timer::new(None, duration);
        assert_eq!(timer.name, None);
        assert!(!timer.is_paused);
    }

    #[test]
    fn test_timer_remaining_time() {
        let duration = Duration::from_secs(10);
        let timer = Timer::new(None, duration);
        let remaining = timer.remaining_time();
        assert!(remaining <= Duration::from_secs(10));
        assert!(remaining > Duration::from_secs(9));
    }

    #[test]
    fn test_timer_progress() {
        let duration = Duration::from_millis(100);
        let timer = Timer::new(None, duration);
        thread::sleep(Duration::from_millis(50));
        let progress = timer.progress();
        assert!(progress > 0.0);
        assert!(progress < 1.0);
    }

    #[test]
    fn test_timer_pause() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(None, duration);

        thread::sleep(Duration::from_millis(50));
        timer.pause();

        assert!(timer.is_paused);
        let remaining_before = timer.remaining_time();

        thread::sleep(Duration::from_millis(50));
        let remaining_after = timer.remaining_time();

        assert_eq!(remaining_before, remaining_after);
    }

    #[test]
    fn test_timer_resume() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(None, duration);

        thread::sleep(Duration::from_millis(50));
        timer.pause();
        assert!(timer.is_paused);

        timer.resume();
        assert!(!timer.is_paused);
    }

    #[test]
    fn test_timer_toggle_pause() {
        let duration = Duration::from_secs(10);
        let mut timer = Timer::new(None, duration);

        assert!(!timer.is_paused);
        timer.toggle_pause();
        assert!(timer.is_paused);
        timer.toggle_pause();
        assert!(!timer.is_paused);
    }

    #[test]
    fn test_timer_finished() {
        let duration = Duration::ZERO;
        let timer = Timer::new(None, duration);
        assert!(timer.is_finished());
    }

    #[test]
    fn test_timer_not_finished() {
        let duration = Duration::from_secs(3600);
        let timer = Timer::new(None, duration);
        assert!(!timer.is_finished());
    }

    #[test]
    fn test_timer_progress_clamped() {
        let duration = Duration::from_millis(1);
        let timer = Timer::new(None, duration);
        thread::sleep(Duration::from_millis(10));
        assert!((timer.progress() - 1.0).abs() < 0.001);
    }
}
