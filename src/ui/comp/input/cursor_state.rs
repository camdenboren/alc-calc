// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

// Adapted from: https://github.com/zed-industries/zed/blob/main/crates/editor/src/blink_manager.rs

use gpui::Context;
use std::time::Duration;

const INTERVAL: u64 = 500;

/// A `CursorState` manager for the `TextInput` element that provides functions for
/// controlling the blinking state of the parent input's cursor
///
/// The timing for both blinking intervals and pauses rely on GPUI's async executor, so
/// the parent `TextInput` should observe it's `CursorState` to update accordingly
///
/// To ensure expected UX for user input, remember to
/// 1. Call `pause_blinking()` in `TextInput` when handling user input
/// 2. Call `show_cursor()` app-wide when focusing the parent `TextInput`
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
    /// Create a `CursorState` for the parent `TextInput`, which defaults to being visible
    /// w/ blinking disabled
    ///
    /// # Examples
    /// ```
    /// use alc_calc::ui::comp::input::cursor_state::CursorState;
    /// use gpui::{Entity, Subscription, prelude::*};
    ///
    /// struct TextInput {
    ///     pub cursor_state: Entity<CursorState>,
    ///     _subscriptions: Vec<Subscription>,
    /// };
    ///
    /// impl TextInput {
    ///     pub fn new(cx: &mut Context<Self>) -> Self {
    ///         let cursor_state = cx.new(|_| {
    ///             CursorState::new()
    ///         });
    ///
    ///         Self {
    ///             cursor_state: cursor_state.clone(),
    ///             _subscriptions: vec![
    ///                 cx.observe(
    ///                     &cursor_state,
    ///                     |_, _, cx| cx.notify()
    ///                 ),
    ///                 // it's also a good idea to
    ///                 // disable blinking when
    ///                 // the window is inactive, but
    ///                 // this is omitted for brevity
    ///             ],
    ///         }
    ///     }
    /// }
    /// ```
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

    /// Show the cursor and pause blinking for `INTERVAL` before resuming via GPUI's async
    /// executor
    pub fn pause_blinking(&mut self, cx: &mut Context<Self>) {
        self.show_cursor(cx);

        let epoch = self.next_blink_epoch();
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(INTERVAL))
                .await;
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

    /// So long as the cursor is enabled and not paused (and the epoch matches, see below
    /// for elaboration), recursively toggle cursor visibility and notify observers each
    /// `INTERVAL` via GPUI's async executor
    ///
    /// In addition to the aforementioned requirements, `blink_cursors` will only recurse
    /// when the given epoch matches what's internally designated as the next one, which
    /// should prevent opaque edge cases related to simultaneous and contradictory state
    /// changes. _Ostensibly, this may occur if `pause_blinking` is called (as it's
    /// publicly accessible) in between the epochs associated w/ each `INTERVAL`_
    fn blink_cursors(&mut self, epoch: usize, cx: &mut Context<Self>) {
        if epoch == self.blink_epoch && self.enabled && !self.blinking_paused {
            self.visible = !self.visible;
            cx.notify();

            let epoch = self.next_blink_epoch();
            cx.spawn(async move |this, cx| {
                cx.background_executor()
                    .timer(Duration::from_millis(INTERVAL))
                    .await;
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| this.blink_cursors(epoch, cx));
                }
            })
            .detach();
        }
    }

    /// Set the cursor to visible (though don't necessarily enable blinking) and notify
    /// observers
    pub fn show_cursor(&mut self, cx: &mut Context<Self>) {
        if !self.visible {
            self.visible = true;
            cx.notify();
        }
    }

    /// Enable cursor blinking on the next render
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

    /// Hide the cursor and disable blinking
    pub fn disable(&mut self, _cx: &mut Context<Self>) {
        self.visible = false;
        self.enabled = false;
    }

    /// Whether the cursor is visible
    pub fn visible(&self) -> bool {
        self.visible
    }
}
