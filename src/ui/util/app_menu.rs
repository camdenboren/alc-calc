// SPDX-FileCopyrightText: Camden Boren
// SPDX-License-Identifier: GPL-3.0-or-later

use crate::ui::{About, CloseWindow, Copy, Cut, Hide, Minimize, NewWindow, Paste, Quit, SelectAll};
use gpui::{Menu, MenuItem, OsAction, SystemMenuType};

pub fn app_menu() -> Vec<Menu> {
    vec![
        Menu {
            name: "alc-calc".into(),
            items: vec![
                MenuItem::action("About alc-calc…", About),
                MenuItem::Separator,
                MenuItem::os_submenu("Services", SystemMenuType::Services),
                MenuItem::Separator,
                MenuItem::action("Hide alc-calc", Hide),
                MenuItem::Separator,
                MenuItem::action("Quit alc-calc", Quit),
            ],
        },
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New Window", NewWindow),
                MenuItem::Separator,
                MenuItem::action("Close Window", CloseWindow),
            ],
        },
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::os_action("Cut", Cut, OsAction::Cut),
                MenuItem::os_action("Copy", Copy, OsAction::Copy),
                MenuItem::os_action("Paste", Paste, OsAction::Paste),
                MenuItem::os_action("Select All", SelectAll, OsAction::SelectAll),
            ],
        },
        Menu {
            name: "Window".into(),
            items: vec![MenuItem::action("Minimize", Minimize)],
        },
    ]
}

pub fn app_dock_menu() -> Vec<MenuItem> {
    vec![MenuItem::action("New Window", NewWindow)]
}
