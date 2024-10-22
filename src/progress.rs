use libc::{ioctl, winsize, STDERR_FILENO, TIOCGWINSZ};
use std::io::{self, Write};
use std::mem::zeroed;

fn get_terminal_width() -> usize {
    unsafe {
        let mut ws: winsize = zeroed();
        if ioctl(STDERR_FILENO, TIOCGWINSZ, &mut ws) == 0 {
            ws.ws_col as usize
        } else {
            80
        }
    }
}

pub struct ProgressBar {
    max_idx: usize,
    last_idx: usize,
}

impl ProgressBar {
    pub fn new(max_idx: usize) -> Self {
        ProgressBar {
            max_idx,
            last_idx: 0,
        }
    }

    pub fn update(&mut self) {
        let str_width = 10; // buffer for the status string
        let max_width = get_terminal_width();
        let max_bar_width = max_width - str_width;

        let progress_ratio = self.last_idx as f64 / self.max_idx as f64;
        let bar_width = (progress_ratio * max_bar_width as f64).round() as usize;

        self.last_idx += 1;

        eprint!(
            "\r {}{}▎{}%",
            "█".repeat(bar_width),
            " ".repeat(max_bar_width - bar_width),
            (progress_ratio * 100.0).round()
        );
        io::stderr().flush().unwrap();
    }
}
