use crate::args::Args;
use crate::timer::Timer;
#[cfg(feature = "notify")]
use std::process::Command;
#[cfg(feature = "notify")]
use std::time::{Duration, Instant};

#[cfg(feature = "notify")]
pub struct NotificationState {
    last_notify: Instant,
    enabled: bool,
}

#[cfg(not(feature = "notify"))]
pub struct NotificationState;

impl NotificationState {
    #[cfg(feature = "notify")]
    pub fn new(args: &Args) -> Result<Self, String> {
        if args.notify {
            if Command::new("notify-send")
                .arg("--version")
                .stdout(std::process::Stdio::null())
                .stderr(std::process::Stdio::null())
                .spawn()
                .is_err()
            {
                return Err(
                    "notify-send not found. Please install it to use notifications.".into(),
                );
            }
        }
        Ok(Self {
            last_notify: Instant::now(),
            enabled: args.notify,
        })
    }

    #[cfg(not(feature = "notify"))]
    pub fn new(_args: &Args) -> Result<Self, String> {
        Ok(Self)
    }

    #[cfg(feature = "notify")]
    pub fn update(&mut self, timer: &Timer) {
        if !self.enabled {
            return;
        }

        if self.last_notify.elapsed() >= Duration::from_secs(1) {
            self.send(timer);
            self.last_notify = Instant::now();
        }
    }

    #[cfg(not(feature = "notify"))]
    pub fn update(&mut self, _timer: &Timer) {}

    #[cfg(feature = "notify")]
    pub fn force_update(&mut self, timer: &Timer) {
        if !self.enabled {
            return;
        }
        self.send(timer);
        self.last_notify = Instant::now();
    }

    #[cfg(not(feature = "notify"))]
    pub fn force_update(&mut self, _timer: &Timer) {}

    #[cfg(feature = "notify")]
    fn send(&self, timer: &Timer) {
        let name = timer.name.as_deref().unwrap_or("Timer");
        let progress = (timer.progress() * 100.0) as i32;
        let remaining = timer.remaining_time();

        let minutes = remaining.as_secs() / 60;
        let seconds = remaining.as_secs() % 60;
        let mut body = format!("Remaining: {:02}:{:02}", minutes, seconds);

        if timer.is_paused {
            body.push_str(" (Paused)");
        }

        let stack_tag = format!("timerrs-{}", name);

        let mut cmd = Command::new("notify-send");
        cmd.arg(name)
            .arg(body)
            .arg("-h")
            .arg(format!("int:value:{}", progress))
            .arg("-h")
            .arg(format!("string:x-dunst-stack-tag:{}", stack_tag))
            .arg("-a")
            .arg("timerrs");

        if timer.is_finished() {
            cmd.arg("-u").arg("critical");
        }

        let _ = cmd.status();
    }
}
