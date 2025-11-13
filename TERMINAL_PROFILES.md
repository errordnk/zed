# Terminal Connection Profiles

Windows Terminal-style connection profiles for Zed Terminal.

## Overview

Terminal profiles allow you to configure multiple connection types (local shell, SSH, etc.) that appear in a dropdown menu in the title bar, just like Windows Terminal.

## UI Layout

```
[Title Bar]
  [Project] [Tab 1] [Tab 2] [+] [▼]
                              │   └─ Profile dropdown
                              └─ New terminal button
```

## Configuration

Profiles are configured in your `settings.json`:

```json
{
  "terminal": {
    "profiles": [
      {
        "type": "local",
        "name": "Local Shell"
      },
      {
        "type": "ssh",
        "name": "Production Server",
        "host": "prod.example.com",
        "user": "admin",
        "port": 22
      }
    ]
  }
}
```

## Profile Types

### Local Shell Profile

```json
{
  "type": "local",
  "name": "Local Shell"
}
```

- **type**: Must be `"local"`
- **name**: Display name in the dropdown menu

### SSH Profile

```json
{
  "type": "ssh",
  "name": "Production Server",
  "host": "prod.example.com",
  "user": "admin",
  "port": 22
}
```

- **type**: Must be `"ssh"`
- **name**: Display name in the dropdown menu
- **host**: SSH server hostname or IP address
- **user**: SSH username (optional)
- **port**: SSH port number (optional, defaults to 22)

## Features

✅ **Implemented:**
- Profile configuration via settings.json
- Dropdown menu with all configured profiles
- Icons: Terminal icon for local, Server icon for SSH
- Profile names displayed in menu
- Click to connect (currently creates new terminal)

⏳ **In Progress:**
- Actual SSH connection implementation
- SSH key-based authentication
- SSH password authentication
- Connection status indicators

🔮 **Planned:**
- Profile-specific shell/environment
- Custom colors per profile
- Profile groups/folders
- Import from ~/.ssh/config
- Profile editing UI

## Default Profile

If no profiles are configured, a single "Local Shell" profile is automatically created as the default.

## Example Configuration

See `assets/settings/terminal-profiles-example.json` for a complete example with multiple SSH profiles.

## How It Works

1. **Settings Loading**: Profiles are loaded from `terminal.profiles` in settings.json
2. **UI Rendering**: Dropdown menu is rendered in title bar via `TerminalPanel.render_title_bar_tabs()`
3. **Profile Selection**: Clicking a profile triggers connection (currently placeholder)
4. **Tab Creation**: New tab is created for each connection

## Code Architecture

- **Profile Types**: `crates/terminal/src/terminal_settings.rs` - `TerminalProfile` enum
- **UI Rendering**: `crates/terminal_view/src/terminal_panel.rs` - dropdown menu generation
- **Title Bar**: `crates/title_bar/src/title_bar.rs` - tabs and buttons in title bar

## Development Status

**Phase 1: UI & Configuration** ✅ COMPLETE
- Terminal profiles data structure
- Settings integration
- Dropdown menu UI
- Profile rendering with icons

**Phase 2: SSH Connection** 🚧 TODO
- SSH command generation from profile
- SSH connection establishment
- Error handling and reconnection

**Phase 3: Advanced Features** 📋 PLANNED
- SSH config file import
- Connection management
- Profile editing UI
