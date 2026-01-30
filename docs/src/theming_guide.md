# Theming Guide

A `Custom` theme is automatically created in alc-calc's platform-specific config directory when you select `Custom` in the app's theme menu.

You can modify the RGB values in `alc-calc/theme.toml` in your text editor of choice and reload the theme in the UI by either

1. Restarting alc-calc or
2. Selecting `Custom` in the theme menu

## Reference

### Values Available on All OSes

| Value               |
| ------------------- |
| variant             |
| text                |
| subtext             |
| inactivetext        |
| background          |
| foreground          |
| foreground_inactive |
| field               |
| field_text          |
| cursor              |
| highlight           |
| border              |
| separator           |
| scrollbar           |
| scrollbar_hover     |

### Values Specific to Linux and macOS

| Value             |
| ----------------- |
| titlebar          |
| titlebar_inactive |

### Values Specific to Linux

| Value                 |
| --------------------- |
| close_button          |
| close_button_hover    |
| close_button_click    |
| close_button_inactive |
