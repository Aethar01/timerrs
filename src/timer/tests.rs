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
