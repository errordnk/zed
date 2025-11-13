# Settings Configuration (Windows Terminal Style)

All application configuration in one place: `settings.json`

## Philosophy

Following Windows Terminal's approach, **everything** is configured through `settings.json`:
- ✅ Terminal connection profiles (local, SSH)
- ✅ Keybindings for all actions
- ✅ Theme and appearance
- ✅ AI providers and models
- ✅ Editor behavior
- ✅ LSP configuration
- ✅ Git integration
- ✅ Project settings

**No separate config files** - one unified settings.json for all configuration.

## Settings Location

```
~/.config/zed/settings.json          # User settings
./project/.zed/settings.json         # Project-specific overrides
```

## Quick Start

See `assets/settings/complete-settings-example.json` for a fully annotated example with all available options.

## Configuration Categories

### 1. Terminal Profiles

Define connection profiles for local shell and SSH servers:

```json
{
  "terminal": {
    "profiles": [
      {"type": "local", "name": "Local Shell"},
      {
        "type": "ssh",
        "name": "Production",
        "host": "prod.example.com",
        "user": "admin",
        "port": 22
      }
    ]
  }
}
```

**Available in:** Dropdown menu (▼) in title bar

### 2. Keybindings

All keyboard shortcuts in one place:

```json
{
  "keymap": [
    {
      "context": "Terminal",
      "bindings": {
        "ctrl-shift-t": "workspace::NewTerminal",
        "ctrl-shift-w": "pane::CloseActiveItem",
        "ctrl-tab": "pane::ActivateNextItem"
      }
    }
  ]
}
```

**Contexts:**
- `Terminal` - when terminal is focused
- `Workspace` - global shortcuts
- `AssistantPanel` - AI assistant shortcuts

### 3. Theme & Appearance

Visual customization:

```json
{
  "theme": {
    "mode": "dark",
    "dark": "One Dark",
    "overrides": {
      "title_bar_background": "#1e1e1e",
      "terminal_background": "#0e0e0e"
    }
  },
  "ui_font_size": 14,
  "buffer_font_family": "JetBrains Mono"
}
```

**Theme properties:**
- `title_bar_background` - title bar color
- `tab_bar_background` - tab bar color
- `tab_active_background` - active tab color
- `terminal_background` - terminal background
- `terminal_foreground` - terminal text color

### 4. AI Providers

Configure multiple AI providers:

```json
{
  "language_models": {
    "default_provider": "anthropic",
    "providers": {
      "anthropic": {
        "api_key": "sk-ant-...",
        "default_model": "claude-3-5-sonnet-20241022"
      },
      "openai": {
        "api_key": "sk-...",
        "default_model": "gpt-4-turbo-preview"
      },
      "ollama": {
        "api_url": "http://localhost:11434",
        "default_model": "codellama:13b"
      }
    }
  }
}
```

**Supported providers:**
- Anthropic Claude
- OpenAI GPT
- Google Gemini
- Ollama (local)
- OpenRouter
- LMStudio
- Mistral

### 5. Assistant Settings

AI assistant configuration:

```json
{
  "assistant": {
    "enabled": true,
    "default_model": {
      "provider": "anthropic",
      "model": "claude-3-5-sonnet-20241022"
    },
    "context": {
      "max_tokens": 100000,
      "include_open_files": true
    }
  }
}
```

## Complete Configuration Structure

```
settings.json
├── terminal           # Terminal profiles & appearance
│   ├── profiles[]     # Connection profiles (local/SSH)
│   ├── shell          # Default shell
│   ├── font_size      # Terminal font size
│   └── env{}          # Environment variables
│
├── keymap[]           # Keybindings by context
│   ├── context        # Terminal, Workspace, etc.
│   └── bindings{}     # Key → action mappings
│
├── theme              # Visual theme
│   ├── mode           # dark/light
│   ├── dark           # Dark theme name
│   ├── light          # Light theme name
│   └── overrides{}    # Custom colors
│
├── language_models    # AI providers
│   ├── default_provider
│   └── providers{}    # Provider configs
│
├── assistant          # AI assistant
│   ├── enabled        # Enable/disable
│   ├── default_model  # Model selection
│   └── context{}      # Context settings
│
├── git                # Git integration
│   ├── enabled
│   └── inline_blame
│
├── lsp{}              # Language servers
├── languages{}        # Language-specific settings
├── project_panel      # File explorer
└── editor settings    # Cursor, formatting, etc.
```

## Examples

### Minimal Configuration

```json
{
  "terminal": {
    "profiles": [
      {"type": "local", "name": "Local Shell"}
    ]
  },
  "theme": {
    "mode": "dark",
    "dark": "One Dark"
  }
}
```

### Developer Setup

```json
{
  "terminal": {
    "profiles": [
      {"type": "local", "name": "Local"},
      {"type": "ssh", "name": "Dev Server", "host": "dev.example.com", "user": "dev"}
    ],
    "font_family": "JetBrains Mono",
    "font_size": 14
  },
  "language_models": {
    "default_provider": "anthropic",
    "providers": {
      "anthropic": {
        "api_key": "sk-ant-...",
        "default_model": "claude-3-5-sonnet-20241022"
      }
    }
  },
  "format_on_save": "on",
  "autosave": "on_focus_change"
}
```

### DevOps Setup

```json
{
  "terminal": {
    "profiles": [
      {"type": "local", "name": "Local"},
      {"type": "ssh", "name": "Production", "host": "prod.example.com", "user": "admin"},
      {"type": "ssh", "name": "Staging", "host": "staging.example.com", "user": "deploy"},
      {"type": "ssh", "name": "Jump Host", "host": "10.0.0.5", "user": "ubuntu", "port": 2222}
    ],
    "shell": {"program": "/bin/bash"},
    "working_directory": "current_project_directory"
  },
  "keymap": [
    {
      "context": "Terminal",
      "bindings": {
        "ctrl-shift-t": "workspace::NewTerminal",
        "ctrl-shift-1": "terminal::ActivateProfile1",
        "ctrl-shift-2": "terminal::ActivateProfile2"
      }
    }
  ]
}
```

## Migration from Other Tools

### From Windows Terminal

Windows Terminal users will find the configuration familiar:

| Windows Terminal | Zed Terminal |
|-----------------|--------------|
| `profiles.list` | `terminal.profiles` |
| `schemes` | `theme.overrides` |
| `keybindings` | `keymap` |
| `defaultProfile` | First profile in array |

### From VS Code

| VS Code | Zed Terminal |
|---------|--------------|
| `settings.json` | `settings.json` ✅ |
| `keybindings.json` | `settings.json` → `keymap` |
| `integrated.terminal.profiles` | `terminal.profiles` |

## Best Practices

1. **Use Comments**: JSON with comments (JSONC) is supported
2. **Validate**: Settings are validated against JSON schema
3. **Version Control**: Commit `.zed/settings.json` for team settings
4. **Personal Overrides**: Use `~/.config/zed/settings.json` for personal settings
5. **Security**: Don't commit API keys - use environment variables or secure storage

## Troubleshooting

### Settings Not Loading

1. Check JSON syntax: `jq . settings.json`
2. Look for validation errors in Zed's developer console
3. Restart Zed after major changes

### Profiles Not Showing

1. Ensure `terminal.profiles` is an array
2. Check profile type is "local" or "ssh"
3. Verify required fields (name, host for SSH)

### Keybindings Not Working

1. Check context matches current focus
2. Look for conflicts with system shortcuts
3. Use Command Palette to see available actions

## Advanced Configuration

See complete example with all options:
```bash
cat assets/settings/complete-settings-example.json
```

## Related Documentation

- `TERMINAL_PROFILES.md` - Detailed profile configuration
- `assets/settings/terminal-profiles-example.json` - Profile examples
- `assets/settings/complete-settings-example.json` - Full configuration reference
