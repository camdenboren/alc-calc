// SPDX-FileCopyrightText: 2025 Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/lumehq/coop/blob/master/crates/ui/src/input/blink_cursor.rs

use gpui::{Context, Timer};
use std::time::Duration;

const INTERVAL: u64 = 500;
const DELAY: u64 = 300;

pub struct CursorState {
    show: bool,
    paused: bool,
    epoch: usize,
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            show: false,
            paused: false,
            epoch: 0,
        }
    }

    pub fn start(&mut self, cx: &mut Context<Self>) {
        self.blink(self.epoch, cx);
    }

    pub fn stop(&mut self, cx: &mut Context<Self>) {
        self.epoch = 0;
        cx.notify();
    }

    fn next_epoch(&mut self) -> usize {
        self.epoch += 1;
        self.epoch
    }

    fn blink(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if self.paused || epoch != self.epoch {
            return;
        }

        self.show = !self.show;
        let epoch = self.next_epoch();
        cx.notify();

        cx.spawn(async move |this, cx| {
            Timer::after(Duration::from_millis(INTERVAL)).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| this.blink(epoch, cx)).ok();
            }
        })
        .detach();
    }

    pub fn show(&self) -> bool {
        self.paused || self.show
    }

    pub fn pause(&mut self, cx: &mut Context<Self>) {
        self.paused = true;
        let epoch = self.next_epoch();
        cx.notify();

        cx.spawn(async move |this, cx| {
            Timer::after(Duration::from_millis(DELAY)).await;
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| {
                    this.paused = false;
                    this.blink(epoch, cx);
                })
                .ok();
            }
        })
        .detach();
    }
}
