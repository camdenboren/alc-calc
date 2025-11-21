// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/editor/src/blink_manager.rs

use gpui::{Context, Timer};
use std::time::Duration;

const INTERVAL: u64 = 500;

pub struct CursorState {
    blink_epoch: usize,
    blinking_paused: bool,
    visible: bool,
    enabled: bool,
}

impl Default for CursorState {
    fn default() -> Self {
        Self::new()
    }
}

impl CursorState {
    pub fn new() -> Self {
        Self {
            blink_epoch: 0,
            blinking_paused: false,
            visible: true,
            enabled: false,
        }
    }

    fn next_blink_epoch(&mut self) -> usize {
        self.blink_epoch += 1;
        self.blink_epoch
    }

    pub fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        self.show_cursor(cx);

        let epoch = self.next_blink_epoch();
        cx.spawn(async move |this, cx| {
            Timer::after(Duration::from_millis(INTERVAL)).await;
            this.update(cx, |this, cx| this.resume_cursor_blinking(epoch, cx))
        })
        .detach();
    }

    fn resume_cursor_blinking(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch {
            self.blinking_paused = false;
            self.blink_cursors(epoch, cx);
        }
    }

    fn blink_cursors(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
            self.visible = !self.visible;
            cx.notify();

            let epoch = self.next_blink_epoch();
            cx.spawn(async move |this, cx| {
                Timer::after(Duration::from_millis(INTERVAL)).await;
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| this.blink_cursors(epoch, cx))
                        .ok();
                }
            })
            .detach();
        }
    }

    pub fn show_cursor(&mut self, cx: &mut Context<Self>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    pub fn enable(&mut self, cx: &mut Context<Self>) {
        if self.enabled {
            return;
        }

        self.enabled = true;
        // Set cursors as invisible and start blinking: this causes cursors
        // to be visible during the next render.
        self.visible = false;
        self.blink_cursors(self.blink_epoch, cx);
    }

    pub fn disable(&mut self, _cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
    }

    pub fn visible(&self) -> bool {
        self.visible
    }
}
